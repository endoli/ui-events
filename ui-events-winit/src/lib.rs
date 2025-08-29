// Copyright 2025 the UI Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! This crate bridges [`winit`]'s native input events (mouse, touch, keyboard, etc.)
//! into the [`ui-events`] model.
//!
//! The primary entry point is [`WindowEventReducer`].
//!
//! [`ui-events`]: https://docs.rs/ui-events/

// LINEBENDER LINT SET - lib.rs - v3
// See https://linebender.org/wiki/canonical-lints/
// These lints shouldn't apply to examples or tests.
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
// These lints shouldn't apply to examples.
#![warn(clippy::print_stdout, clippy::print_stderr)]
// Targeting e.g. 32-bit means structs containing usize can give false positives for 64-bit.
#![cfg_attr(target_pointer_width = "64", warn(clippy::trivially_copy_pass_by_ref))]
// END LINEBENDER LINT SET
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

pub mod keyboard;
pub mod pointer;

extern crate alloc;
use alloc::{vec, vec::Vec};

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

#[cfg(not(target_arch = "wasm32"))]
pub use std::time::Instant;

#[cfg(target_arch = "wasm32")]
pub use web_time::Instant;

use ui_events::{
    keyboard::KeyboardEvent,
    pointer::{
        PointerButtonEvent, PointerEvent, PointerId, PointerInfo, PointerScrollEvent, PointerState,
        PointerType, PointerUpdate,
    },
    ScrollDelta,
};
use winit::{
    event::{ElementState, Force, MouseScrollDelta, Touch, TouchPhase, WindowEvent},
    keyboard::ModifiersState,
};

/// Manages stateful transformations of winit [`WindowEvent`].
///
/// Store a single instance of this per window, then call [`WindowEventReducer::reduce`]
/// on each [`WindowEvent`] for that window.
/// Use the [`WindowEventTranslation`] value to receive [`PointerEvent`]s and [`KeyboardEvent`]s.
///
/// This handles:
///  - [`ModifiersChanged`][`WindowEvent::ModifiersChanged`]
///  - [`KeyboardInput`][`WindowEvent::KeyboardInput`]
///  - [`Touch`][`WindowEvent::Touch`]
///  - [`MouseInput`][`WindowEvent::MouseInput`]
///  - [`MouseWheel`][`WindowEvent::MouseWheel`]
///  - [`CursorMoved`][`WindowEvent::CursorMoved`]
///  - [`CursorEntered`][`WindowEvent::CursorEntered`]
///  - [`CursorLeft`][`WindowEvent::CursorLeft`]
#[derive(Debug, Default)]
pub struct WindowEventReducer {
    /// State of modifiers.
    modifiers: ModifiersState,
    /// State of the primary mouse pointer.
    primary_state: PointerState,
    /// Click and tap counter.
    counter: TapCounter,
    /// First time an event was received..
    first_instant: Option<Instant>,
}

#[allow(clippy::cast_possible_truncation)]
impl WindowEventReducer {
    /// Process a [`WindowEvent`].
    pub fn reduce(
        &mut self,
        scale_factor: f64,
        we: &WindowEvent,
    ) -> Option<WindowEventTranslation> {
        const PRIMARY_MOUSE: PointerInfo = PointerInfo {
            pointer_id: Some(PointerId::PRIMARY),
            // TODO: Maybe transmute device.
            persistent_device_id: None,
            pointer_type: PointerType::Mouse,
        };

        let time = Instant::now()
            .duration_since(*self.first_instant.get_or_insert_with(Instant::now))
            .as_nanos() as u64;

        self.primary_state.time = time;

        match we {
            WindowEvent::ModifiersChanged(m) => {
                self.modifiers = m.state();
                self.primary_state.modifiers = keyboard::from_winit_modifier_state(self.modifiers);
                None
            }
            WindowEvent::KeyboardInput { event, .. } => Some(WindowEventTranslation::Keyboard(
                keyboard::from_winit_keyboard_event(event.clone(), self.modifiers),
            )),
            WindowEvent::CursorEntered { .. } => Some(WindowEventTranslation::Pointer(
                PointerEvent::Enter(PRIMARY_MOUSE),
            )),
            WindowEvent::CursorLeft { .. } => Some(WindowEventTranslation::Pointer(
                PointerEvent::Leave(PRIMARY_MOUSE),
            )),
            WindowEvent::CursorMoved { position, .. } => {
                self.primary_state.position = *position;

                Some(WindowEventTranslation::Pointer(self.counter.attach_count(
                    scale_factor,
                    PointerEvent::Move(PointerUpdate {
                        pointer: PRIMARY_MOUSE,
                        current: self.primary_state.clone(),
                        coalesced: vec![],
                        predicted: vec![],
                    }),
                )))
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button,
                ..
            } => {
                let button = pointer::try_from_winit_button(*button);
                if let Some(button) = button {
                    self.primary_state.buttons.insert(button);
                }

                Some(WindowEventTranslation::Pointer(self.counter.attach_count(
                    scale_factor,
                    PointerEvent::Down(PointerButtonEvent {
                        pointer: PRIMARY_MOUSE,
                        button,
                        state: self.primary_state.clone(),
                    }),
                )))
            }
            WindowEvent::MouseInput {
                state: ElementState::Released,
                button,
                ..
            } => {
                let button = pointer::try_from_winit_button(*button);
                if let Some(button) = button {
                    self.primary_state.buttons.remove(button);
                }

                Some(WindowEventTranslation::Pointer(self.counter.attach_count(
                    scale_factor,
                    PointerEvent::Up(PointerButtonEvent {
                        pointer: PRIMARY_MOUSE,
                        button,
                        state: self.primary_state.clone(),
                    }),
                )))
            }
            WindowEvent::MouseWheel { delta, .. } => Some(WindowEventTranslation::Pointer(
                PointerEvent::Scroll(PointerScrollEvent {
                    pointer: PRIMARY_MOUSE,
                    delta: match *delta {
                        MouseScrollDelta::LineDelta(x, y) => ScrollDelta::LineDelta(x, y),
                        MouseScrollDelta::PixelDelta(p) => ScrollDelta::PixelDelta(p),
                    },
                    state: self.primary_state.clone(),
                }),
            )),
            WindowEvent::Touch(Touch {
                phase,
                id,
                location,
                force,
                ..
            }) => {
                let pointer = PointerInfo {
                    pointer_id: PointerId::new(id.saturating_add(1)),
                    pointer_type: PointerType::Touch,
                    persistent_device_id: None,
                };

                use TouchPhase::*;

                let state = PointerState {
                    time,
                    position: *location,
                    modifiers: self.primary_state.modifiers,
                    pressure: if matches!(phase, Ended | Cancelled) {
                        0.0
                    } else {
                        match force {
                            Some(Force::Calibrated { force, .. }) => (force * 0.5) as f32,
                            Some(Force::Normalized(q)) => *q as f32,
                            _ => 0.5,
                        }
                    },
                    ..Default::default()
                };

                Some(WindowEventTranslation::Pointer(self.counter.attach_count(
                    scale_factor,
                    match phase {
                        Started => PointerEvent::Down(PointerButtonEvent {
                            pointer,
                            button: None,
                            state,
                        }),
                        Moved => PointerEvent::Move(PointerUpdate {
                            pointer,
                            current: state,
                            coalesced: vec![],
                            predicted: vec![],
                        }),
                        Cancelled => PointerEvent::Cancel(pointer),
                        Ended => PointerEvent::Up(PointerButtonEvent {
                            pointer,
                            button: None,
                            state,
                        }),
                    },
                )))
            }
            _ => None,
        }
    }
}

/// Result of [`WindowEventReducer::reduce`].
#[derive(Debug)]
pub enum WindowEventTranslation {
    /// Resulting [`KeyboardEvent`].
    Keyboard(KeyboardEvent),
    /// Resulting [`PointerEvent`].
    Pointer(PointerEvent),
}

#[derive(Clone, Debug)]
struct TapState {
    /// Pointer ID used to attach tap counts to [`PointerEvent::Move`].
    pointer_id: Option<PointerId>,
    /// Nanosecond timestamp when the tap went Down.
    down_time: u64,
    /// Nanosecond timestamp when the tap went Up.
    ///
    /// Resets to `down_time` when tap goes Down.
    up_time: u64,
    /// The local tap count as of the last Down phase.
    count: u8,
    /// x coordinate.
    x: f64,
    /// y coordinate.
    y: f64,
}

#[derive(Debug, Default)]
struct TapCounter {
    taps: Vec<TapState>,
}

impl TapCounter {
    /// Enhance a [`PointerEvent`] with a `count`.
    fn attach_count(&mut self, scale_factor: f64, e: PointerEvent) -> PointerEvent {
        match e {
            PointerEvent::Down(mut event) => {
                let pointer_id = event.pointer.pointer_id;
                let position = event.state.position;
                let time = event.state.time;

                let slop = match event.pointer.pointer_type {
                    // This is on the low side of double tap slop, validated
                    // experimentally to work on a few touchscreen laptops.
                    PointerType::Touch => 12.0,
                    PointerType::Pen => 6.0,
                    // This is slightly more forgiving than the default on Windows for mice.
                    // In order to make the slop calculation more similar between devices,
                    // this uses a slightly different method than Windows, which tests if the
                    // tap is in a box, rather than in a circle, centered on the anchor point.
                    _ => 2.0,
                } * core::f64::consts::SQRT_2
                    * scale_factor;

                if let Some(tap) =
                    self.taps.iter_mut().find(|TapState { x, y, up_time, .. }| {
                        let dx = (x - position.x).abs();
                        let dy = (y - position.y).abs();
                        (dx * dx + dy * dy).sqrt() < slop && (up_time + 500_000_000) > time
                    })
                {
                    let count = tap.count + 1;
                    event.state.count = count;
                    tap.count = count;
                    tap.pointer_id = pointer_id;
                    tap.down_time = time;
                    tap.up_time = time;
                    tap.x = position.x;
                    tap.y = position.y;
                } else {
                    let s = TapState {
                        pointer_id,
                        down_time: time,
                        up_time: time,
                        count: 1,
                        x: position.x,
                        y: position.y,
                    };
                    self.taps.push(s);
                    event.state.count = 1;
                };
                self.clear_expired(time);
                PointerEvent::Down(event)
            }
            PointerEvent::Up(mut event) => {
                let p_id = event.pointer.pointer_id;
                if let Some(tap) = self.taps.iter_mut().find(|state| state.pointer_id == p_id) {
                    tap.up_time = event.state.time;
                    event.state.count = tap.count;
                }
                PointerEvent::Up(event)
            }
            PointerEvent::Move(PointerUpdate {
                pointer,
                mut current,
                mut coalesced,
                mut predicted,
            }) => {
                if let Some(TapState { count, .. }) = self
                    .taps
                    .iter()
                    .find(
                        |TapState {
                             pointer_id,
                             down_time,
                             up_time,
                             ..
                         }| {
                            *pointer_id == pointer.pointer_id && down_time == up_time
                        },
                    )
                    .cloned()
                {
                    current.count = count;
                    for event in coalesced.iter_mut() {
                        event.count = count;
                    }
                    for event in predicted.iter_mut() {
                        event.count = count;
                    }
                    PointerEvent::Move(PointerUpdate {
                        pointer,
                        current,
                        coalesced,
                        predicted,
                    })
                } else {
                    PointerEvent::Move(PointerUpdate {
                        pointer,
                        current,
                        coalesced,
                        predicted,
                    })
                }
            }
            PointerEvent::Cancel(p) => {
                self.taps
                    .retain(|TapState { pointer_id, .. }| *pointer_id != p.pointer_id);
                PointerEvent::Cancel(p)
            }
            PointerEvent::Leave(p) => {
                self.taps
                    .retain(|TapState { pointer_id, .. }| *pointer_id != p.pointer_id);
                PointerEvent::Leave(p)
            }
            e @ (PointerEvent::Enter(..) | PointerEvent::Scroll(..)) => e,
        }
    }

    /// Clear expired taps.
    ///
    /// `t` is the time of the last received event.
    /// All events have the same time base on Android, so this is valid here.
    fn clear_expired(&mut self, t: u64) {
        self.taps.retain(
            |TapState {
                 down_time, up_time, ..
             }| { down_time == up_time || (up_time + 500_000_000) > t },
        );
    }
}

#[cfg(test)]
mod tests {
    // CI will fail unless cargo nextest can execute at least one test per workspace.
    // Delete this dummy test once we have an actual real test.
    #[test]
    fn dummy_test_until_we_have_a_real_test() {}
}
