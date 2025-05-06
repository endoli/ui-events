// Copyright 2025 the UI Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! This crate provides an adapter between [`winit`] events and
//! [`ui-events`].
//!
//! The primary entry point is [`WindowEventReducer`].

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

extern crate std;
use std::time::Instant;

use ui_events::{
    keyboard::KeyboardEvent,
    pointer::{PointerEvent, PointerId, PointerInfo, PointerState, PointerType, PointerUpdate},
};
use winit::{
    event::{ElementState, Force, Touch, TouchPhase, WindowEvent},
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
    pub fn reduce(&mut self, we: &WindowEvent) -> Option<WindowEventTranslation> {
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
                self.primary_state.x = position.x as f32;
                self.primary_state.y = position.y as f32;

                Some(WindowEventTranslation::Pointer(self.counter.attach_count(
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
                    PointerEvent::Down {
                        pointer: PRIMARY_MOUSE,
                        button,
                        state: self.primary_state.clone(),
                    },
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
                    PointerEvent::Up {
                        pointer: PRIMARY_MOUSE,
                        button,
                        state: self.primary_state.clone(),
                    },
                )))
            }
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
                    x: location.x as f32,
                    y: location.y as f32,
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
                    match phase {
                        Started => PointerEvent::Down {
                            pointer,
                            button: None,
                            state,
                        },
                        Moved => PointerEvent::Move(PointerUpdate {
                            pointer,
                            current: state,
                            coalesced: vec![],
                            predicted: vec![],
                        }),
                        Cancelled => PointerEvent::Cancel(pointer),
                        Ended => PointerEvent::Up {
                            pointer,
                            button: None,
                            state,
                        },
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
    x: f32,
    /// y coordinate.
    y: f32,
}

#[derive(Debug, Default)]
struct TapCounter {
    taps: Vec<TapState>,
}

impl TapCounter {
    /// Enhance a [`PointerEvent`] with a `count`.
    fn attach_count(&mut self, e: PointerEvent) -> PointerEvent {
        match e {
            PointerEvent::Down {
                button,
                pointer,
                state,
            } => {
                let e = if let Some(i) =
                    self.taps.iter().position(|TapState { x, y, up_time, .. }| {
                        let dx = (x - state.x).abs();
                        let dy = (y - state.y).abs();
                        (dx * dx + dy * dy).sqrt() < 4.0 && (up_time + 500_000_000) > state.time
                    }) {
                    let count = self.taps[i].count + 1;
                    self.taps[i].count = count;
                    self.taps[i].pointer_id = pointer.pointer_id;
                    self.taps[i].down_time = state.time;
                    self.taps[i].up_time = state.time;
                    self.taps[i].x = state.x;
                    self.taps[i].y = state.y;

                    PointerEvent::Down {
                        button,
                        pointer,
                        state: PointerState { count, ..state },
                    }
                } else {
                    let s = TapState {
                        pointer_id: pointer.pointer_id,
                        down_time: state.time,
                        up_time: state.time,
                        count: 1,
                        x: state.x,
                        y: state.y,
                    };
                    self.taps.push(s);
                    PointerEvent::Down {
                        button,
                        pointer,
                        state: PointerState { count: 1, ..state },
                    }
                };
                self.clear_expired(state.time);
                e
            }
            PointerEvent::Up {
                button,
                pointer,
                ref state,
            } => {
                if let Some(i) = self
                    .taps
                    .iter()
                    .position(|TapState { pointer_id, .. }| *pointer_id == pointer.pointer_id)
                {
                    self.taps[i].up_time = state.time;
                    PointerEvent::Up {
                        button,
                        pointer,
                        state: PointerState {
                            count: self.taps[i].count,
                            ..state.clone()
                        },
                    }
                } else {
                    e.clone()
                }
            }
            PointerEvent::Move(PointerUpdate {
                pointer,
                ref current,
                ref coalesced,
                ref predicted,
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
                    PointerEvent::Move(PointerUpdate {
                        pointer,
                        current: PointerState {
                            count,
                            ..current.clone()
                        },
                        coalesced: coalesced
                            .iter()
                            .cloned()
                            .map(|u| PointerState { count, ..u })
                            .collect(),
                        predicted: predicted
                            .iter()
                            .cloned()
                            .map(|u| PointerState { count, ..u })
                            .collect(),
                    })
                } else {
                    e
                }
            }
            PointerEvent::Cancel(p) | PointerEvent::Leave(p) => {
                self.taps
                    .retain(|TapState { pointer_id, .. }| *pointer_id != p.pointer_id);
                e.clone()
            }
            PointerEvent::Enter(..) | PointerEvent::Scroll { .. } => e.clone(),
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
