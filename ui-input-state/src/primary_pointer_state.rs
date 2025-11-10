// Copyright 2025 the UI Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Primary pointer state across frames.
//!
//! `PrimaryPointerState` maintains the current pointer state along with
//! per-frame button transitions, coalesced historical states for the current
//! frame, and any predicted states provided by the backend. Only events from
//! the primary pointer are processed (see `ui-events` primary pointer semantics).
//!
//! Feed it `ui-events` pointer events as they arrive; query it during your
//! update; and call [`clear_frame`](PrimaryPointerState::clear_frame) at the
//! end of the frame.
//!
//! ## Example:
//!
//! ```no_run
//! use ui_input_state::PrimaryPointerState;
//! use ui_events::pointer::{
//!     PointerEvent, PointerButtonEvent, PointerButton, PointerInfo, PointerType, PointerId,
//!     PointerState, PointerUpdate,
//! };
//! use dpi::PhysicalPosition;
//!
//! let mut ps = PrimaryPointerState::default();
//! // Synthesize a move event
//! let current = PointerState {
//!     time: 1,
//!     position: PhysicalPosition { x: 10.0, y: 20.0 },
//!     scale_factor: 2.0,
//!     ..Default::default()
//! };
//! let ev = PointerEvent::Move(PointerUpdate {
//!     pointer: PointerInfo {
//!         pointer_id: Some(PointerId::PRIMARY),
//!         persistent_device_id: None,
//!         pointer_type: PointerType::Mouse,
//!     },
//!     current,
//!     coalesced: vec![],
//!     predicted: vec![],
//! });
//! ps.process_pointer_event(ev);
//! let lp = ps.current_logical_position();
//! assert_eq!(lp.x, 5.0);
//! ```
extern crate alloc;
use alloc::vec::Vec;

use ui_events::pointer::{
    PointerButton, PointerButtonEvent, PointerButtons, PointerEvent, PointerState, PointerUpdate,
};

use dpi::{LogicalPosition, PhysicalPosition};

/// A stateful view of the primary pointer.
#[derive(Clone, Debug, Default)]
pub struct PrimaryPointerState {
    /// Buttons that were pressed during the current frame.
    just_pressed: PointerButtons,
    /// Buttons that were released during the current frame.
    just_released: PointerButtons,
    /// Current state.
    current: PointerState,
    /// Coalesced states, ordered by `time`.
    coalesced: Vec<PointerState>,
    /// Predicted states, ordered by `time`.
    predicted: Vec<PointerState>,
}

impl PrimaryPointerState {
    /// Return `true` if the `button` was pressed within the last frame.
    ///
    /// This corresponds to having received a [`PointerEvent::Down`] event
    /// for that button on the primary pointing device.
    pub fn is_just_pressed(&self, button: PointerButton) -> bool {
        self.just_pressed.contains(button)
    }

    /// Return `true` if the `button` was released within the last frame.
    ///
    /// This corresponds to having received a [`PointerEvent::Up`] event
    /// for that button on the primary pointing device.
    pub fn is_just_released(&self, button: PointerButton) -> bool {
        self.just_released.contains(button)
    }

    /// Return `true` if the Auxiliary button (usually middle mouse) was
    /// pressed within the last frame.
    pub fn is_auxiliary_just_pressed(&self) -> bool {
        self.is_just_pressed(PointerButton::Auxiliary)
    }

    /// Return `true` if the Auxiliary button was released within the last frame.
    pub fn is_auxiliary_just_released(&self) -> bool {
        self.is_just_released(PointerButton::Auxiliary)
    }

    /// Return `true` if the Primary button (usually left mouse) was
    /// pressed within the last frame.
    pub fn is_primary_just_pressed(&self) -> bool {
        self.is_just_pressed(PointerButton::Primary)
    }

    /// Return `true` if the Primary button was released within the last frame.
    pub fn is_primary_just_released(&self) -> bool {
        self.is_just_released(PointerButton::Primary)
    }

    /// Return `true` if the Secondary button (usually right mouse) was
    /// pressed within the last frame.
    pub fn is_secondary_just_pressed(&self) -> bool {
        self.is_just_pressed(PointerButton::Secondary)
    }

    /// Return `true` if the Secondary button was released within the last frame.
    pub fn is_secondary_just_released(&self) -> bool {
        self.is_just_released(PointerButton::Secondary)
    }

    /// Return `true` if any button is currently held down.
    pub fn is_any_down(&self) -> bool {
        !self.current.buttons.is_empty()
    }

    /// Return `true` if the specified `button` is currently held down.
    pub fn is_down(&self, button: PointerButton) -> bool {
        self.current.buttons.contains(button)
    }

    /// Clear the per-frame state to prepare for a new frame.
    pub fn clear_frame(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
        self.coalesced.clear();
        // TODO: Persist predicted states that are not yet stale.
        self.predicted.clear();
    }

    /// Current position.
    ///
    /// This will only give known positions.
    pub fn current_position(&self) -> PhysicalPosition<f64> {
        self.current.position
    }

    /// Current position (in logical units).
    ///
    /// This will only give known positions.
    pub fn current_logical_position(&self) -> LogicalPosition<f64> {
        self.current.logical_position()
    }

    /// Relative motion this frame.
    pub fn motion(&self) -> PhysicalPosition<f64> {
        let current = self.current.position;
        let first = self
            .coalesced
            .first()
            .map(|s| s.position)
            .unwrap_or(current);
        PhysicalPosition {
            x: current.x - first.x,
            y: current.y - first.y,
        }
    }

    /// Relative motion this frame.
    pub fn logical_motion(&self) -> LogicalPosition<f64> {
        let current = self.current.logical_position();
        let first = self
            .coalesced
            .first()
            .map(|s| s.logical_position())
            .unwrap_or(current);
        LogicalPosition {
            x: current.x - first.x,
            y: current.y - first.y,
        }
    }

    /// Push a state and coalesce the existing one if it is not the initial state.
    fn push_state(&mut self, state: PointerState) {
        if state.time != 0 {
            // If `time` is 0, this is the initial state.
            self.coalesced.push(state);
        }
    }

    /// Update the state based on the given pointer event.
    ///
    /// Only events from the primary pointer are processed. Press and release
    /// events update the `just_pressed`, `just_released`, and `down` states.
    pub fn process_pointer_event(&mut self, event: PointerEvent) {
        if !event.is_primary_pointer() {
            return;
        }

        match event {
            PointerEvent::Down(PointerButtonEvent {
                button: Some(b),
                state,
                ..
            }) => {
                self.just_pressed.insert(b);
                let mut state = state.clone();
                core::mem::swap(&mut self.current, &mut state);
                self.push_state(state);
                // TODO: Propagate button state to predicted states.
                self.predicted.clear();
            }
            PointerEvent::Up(PointerButtonEvent {
                button: Some(b),
                state,
                ..
            }) => {
                self.just_released.insert(b);
                let mut state = state.clone();
                core::mem::swap(&mut self.current, &mut state);
                self.push_state(state);
                // TODO: Propagate button state to predicted states.
                self.predicted.clear();
            }
            PointerEvent::Move(PointerUpdate {
                current,
                coalesced,
                predicted,
                ..
            }) => {
                self.coalesced.push(self.current.clone());
                self.current = current.clone();
                self.coalesced.extend(coalesced);
                self.predicted.clear();
                self.predicted.extend(predicted);
            }
            PointerEvent::Cancel(_) | PointerEvent::Leave(_) => {
                // TODO: Validate these behaviors.
                self.predicted.clear();
                self.coalesced.clear();
                self.current.buttons.clear();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use ui_events::pointer::{
        PointerButtonEvent, PointerEvent, PointerId, PointerInfo, PointerState, PointerType,
    };

    /// Get a monotonically increasing time.
    fn phony_time() -> u64 {
        use core::sync::atomic::AtomicU64;
        use core::sync::atomic::Ordering;
        static TIME: AtomicU64 = AtomicU64::new(0);
        TIME.fetch_add(1, Ordering::SeqCst)
    }

    fn make_down_event(button: PointerButton) -> PointerEvent {
        PointerEvent::Down(PointerButtonEvent {
            button: Some(button),
            pointer: PointerInfo {
                pointer_id: Some(PointerId::PRIMARY),
                persistent_device_id: None,
                pointer_type: PointerType::Mouse,
            },
            state: PointerState {
                time: phony_time(),
                buttons: button.into(),
                ..Default::default()
            },
        })
    }

    fn make_up_event(button: PointerButton) -> PointerEvent {
        PointerEvent::Up(PointerButtonEvent {
            button: Some(button),
            pointer: PointerInfo {
                pointer_id: Some(PointerId::PRIMARY),
                persistent_device_id: None,
                pointer_type: PointerType::Mouse,
            },
            state: PointerState {
                time: phony_time(),
                ..Default::default()
            },
        })
    }

    #[test]
    fn press_and_hold_primary() {
        let mut state = PrimaryPointerState::default();
        state.process_pointer_event(make_down_event(PointerButton::Primary));

        assert!(state.is_primary_just_pressed());
        assert!(state.is_down(PointerButton::Primary));
        assert!(!state.is_primary_just_released());

        state.clear_frame();

        assert!(!state.is_primary_just_pressed());
        assert!(state.is_down(PointerButton::Primary));
    }

    #[test]
    fn press_and_release_primary_same_frame() {
        let mut state = PrimaryPointerState::default();
        state.process_pointer_event(make_down_event(PointerButton::Primary));
        state.process_pointer_event(make_up_event(PointerButton::Primary));

        assert!(state.is_primary_just_pressed());
        assert!(state.is_primary_just_released());
        assert!(!state.is_down(PointerButton::Primary));
    }

    #[test]
    fn release_after_hold() {
        let mut state = PrimaryPointerState::default();
        state.process_pointer_event(make_down_event(PointerButton::Primary));
        state.clear_frame();
        state.process_pointer_event(make_up_event(PointerButton::Primary));

        assert!(!state.is_primary_just_pressed());
        assert!(state.is_primary_just_released());
        assert!(!state.is_down(PointerButton::Primary));
    }

    fn make_move_event(
        position: PhysicalPosition<f64>,
        coalesced: Vec<PhysicalPosition<f64>>,
        predicted: Vec<PhysicalPosition<f64>>,
    ) -> PointerEvent {
        let coalesced = coalesced
            .iter()
            .copied()
            .map(|position| PointerState {
                time: phony_time(),
                position,
                ..Default::default()
            })
            .collect();

        let current = PointerState {
            time: phony_time(),
            position,
            ..Default::default()
        };

        let predicted = predicted
            .iter()
            .copied()
            .map(|position| PointerState {
                time: phony_time(),
                position,
                ..Default::default()
            })
            .collect();

        PointerEvent::Move(PointerUpdate {
            pointer: PointerInfo {
                pointer_id: Some(PointerId::PRIMARY),
                persistent_device_id: None,
                pointer_type: PointerType::Mouse,
            },
            current,
            coalesced,
            predicted,
        })
    }

    fn make_cancel_event() -> PointerEvent {
        PointerEvent::Cancel(PointerInfo {
            pointer_id: Some(PointerId::PRIMARY),
            persistent_device_id: None,
            pointer_type: PointerType::Mouse,
        })
    }

    fn make_leave_event() -> PointerEvent {
        PointerEvent::Leave(PointerInfo {
            pointer_id: Some(PointerId::PRIMARY),
            persistent_device_id: None,
            pointer_type: PointerType::Mouse,
        })
    }

    #[test]
    fn down_updates_current_buttons() {
        let mut state = PrimaryPointerState::default();
        state.process_pointer_event(make_down_event(PointerButton::Primary));

        assert!(state.current.buttons.contains(PointerButton::Primary));
        assert!(state.is_down(PointerButton::Primary));
        assert!(state.is_any_down());
    }

    #[test]
    fn up_updates_current_buttons() {
        let mut state = PrimaryPointerState::default();
        state.process_pointer_event(make_down_event(PointerButton::Primary));
        state.process_pointer_event(make_up_event(PointerButton::Primary));

        assert!(!state.current.buttons.contains(PointerButton::Primary));
        assert!(!state.is_down(PointerButton::Primary));
        assert!(!state.is_any_down());
    }

    #[test]
    fn move_appends_to_coalesced_if_down() {
        let mut state = PrimaryPointerState::default();
        state.process_pointer_event(make_down_event(PointerButton::Primary));

        state.process_pointer_event(make_move_event(
            PhysicalPosition { x: 10.0, y: 10.0 },
            vec![PhysicalPosition { x: 5.0, y: 5.0 }],
            vec![],
        ));

        assert_eq!(state.coalesced.len(), 2);
        assert_eq!(
            state.coalesced[1].position,
            PhysicalPosition { x: 5.0, y: 5.0 }
        );
        assert_eq!(
            state.current.position,
            PhysicalPosition { x: 10.0, y: 10.0 }
        );
    }

    #[test]
    fn move_appends_to_coalesced_even_if_not_down() {
        let mut state = PrimaryPointerState::default();

        state.process_pointer_event(make_move_event(
            PhysicalPosition { x: 10.0, y: 10.0 },
            vec![PhysicalPosition { x: 5.0, y: 5.0 }],
            vec![],
        ));

        assert_eq!(state.coalesced.len(), 2);
        assert_eq!(state.coalesced[0].position, PhysicalPosition::default());
        assert_eq!(
            state.coalesced[1].position,
            PhysicalPosition { x: 5.0, y: 5.0 }
        );
        assert_eq!(
            state.current.position,
            PhysicalPosition { x: 10.0, y: 10.0 }
        );
    }

    #[test]
    fn move_sets_predicted() {
        let mut state = PrimaryPointerState::default();

        state.process_pointer_event(make_move_event(
            PhysicalPosition { x: 10.0, y: 10.0 },
            vec![],
            vec![PhysicalPosition { x: 15.0, y: 15.0 }],
        ));

        assert_eq!(state.predicted.len(), 1);
        assert_eq!(
            state.predicted[0].position,
            PhysicalPosition { x: 15.0, y: 15.0 }
        );
    }

    #[test]
    fn down_clears_predicted() {
        let mut state = PrimaryPointerState::default();

        state.process_pointer_event(make_move_event(
            PhysicalPosition { x: 10.0, y: 10.0 },
            vec![],
            vec![PhysicalPosition { x: 15.0, y: 15.0 }],
        ));

        assert!(!state.predicted.is_empty());

        state.process_pointer_event(make_down_event(PointerButton::Primary));

        assert!(state.predicted.is_empty());
    }

    #[test]
    fn up_clears_predicted() {
        let mut state = PrimaryPointerState::default();
        state.process_pointer_event(make_down_event(PointerButton::Primary));

        state.process_pointer_event(make_move_event(
            PhysicalPosition { x: 10.0, y: 10.0 },
            vec![],
            vec![PhysicalPosition { x: 15.0, y: 15.0 }],
        ));

        assert!(!state.predicted.is_empty());

        state.process_pointer_event(make_up_event(PointerButton::Primary));

        assert!(state.predicted.is_empty());
    }

    #[test]
    fn cancel_clears_states() {
        let mut state = PrimaryPointerState::default();
        state.process_pointer_event(make_down_event(PointerButton::Primary));

        assert!(state.predicted.is_empty());
        assert!(!state.current.buttons.is_empty());

        state.process_pointer_event(make_cancel_event());

        assert!(state.coalesced.is_empty());
        assert!(state.predicted.is_empty());
        assert!(state.current.buttons.is_empty());
    }

    #[test]
    fn leave_clears_states() {
        let mut state = PrimaryPointerState::default();
        state.process_pointer_event(make_down_event(PointerButton::Primary));

        assert!(state.predicted.is_empty());
        assert!(!state.current.buttons.is_empty());

        state.process_pointer_event(make_leave_event());

        assert!(state.coalesced.is_empty());
        assert!(state.predicted.is_empty());
        assert!(state.current.buttons.is_empty());
    }

    #[test]
    fn clear_frame_clears_coalesced_and_predicted() {
        let mut state = PrimaryPointerState::default();

        state.process_pointer_event(make_move_event(
            PhysicalPosition { x: 10.0, y: 10.0 },
            vec![PhysicalPosition { x: 5.0, y: 5.0 }],
            vec![PhysicalPosition { x: 15.0, y: 15.0 }],
        ));

        assert!(!state.coalesced.is_empty());
        assert!(!state.predicted.is_empty());

        state.clear_frame();

        assert!(state.coalesced.is_empty());
        assert!(state.predicted.is_empty());
    }

    #[test]
    fn current_position_and_logical() {
        let mut state = PrimaryPointerState::default();
        let position = PhysicalPosition { x: 100.0, y: 200.0 };
        let scale_factor = 2.0;

        let current = PointerState {
            time: phony_time(),
            position,
            scale_factor,
            ..Default::default()
        };

        state.process_pointer_event(PointerEvent::Move(PointerUpdate {
            pointer: PointerInfo {
                pointer_id: Some(PointerId::PRIMARY),
                persistent_device_id: None,
                pointer_type: PointerType::Mouse,
            },
            current,
            coalesced: vec![],
            predicted: vec![],
        }));

        assert_eq!(state.current_position(), position);
        assert_eq!(
            state.current_logical_position(),
            position.to_logical(scale_factor)
        );
    }

    #[test]
    fn motion_with_coalesced() {
        let mut state = PrimaryPointerState::default();

        state.process_pointer_event(make_move_event(
            PhysicalPosition { x: 10.0, y: 20.0 },
            vec![],
            vec![],
        ));

        state.process_pointer_event(make_move_event(
            PhysicalPosition { x: 30.0, y: 40.0 },
            vec![PhysicalPosition { x: 15.0, y: 25.0 }],
            vec![],
        ));

        assert_eq!(state.motion(), PhysicalPosition { x: 30.0, y: 40.0 });
        assert_eq!(state.logical_motion(), LogicalPosition { x: 30.0, y: 40.0 });
    }

    #[test]
    fn motion_without_coalesced() {
        let mut state = PrimaryPointerState::default();

        state.process_pointer_event(make_move_event(
            PhysicalPosition { x: 30.0, y: 40.0 },
            vec![],
            vec![],
        ));

        assert_eq!(state.motion(), PhysicalPosition { x: 30.0, y: 40.0 });
        assert_eq!(state.logical_motion(), LogicalPosition { x: 30.0, y: 40.0 });
    }
}
