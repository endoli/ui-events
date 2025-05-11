// Copyright 2025 the UI Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use ui_events::pointer::{PointerButton, PointerButtonEvent, PointerButtons, PointerEvent};

/// A stateful view of the primary pointer.
#[derive(Clone, Debug, Default)]
pub struct PrimaryPointerState {
    /// Buttons that were pressed during the current frame.
    just_pressed: PointerButtons,
    /// Buttons that were released during the current frame.
    just_released: PointerButtons,
    /// Buttons that are currently being held down.
    down: PointerButtons,
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
        !self.down.is_empty()
    }

    /// Return `true` if the specified `button` is currently held down.
    pub fn is_down(&self, button: PointerButton) -> bool {
        self.down.contains(button)
    }

    /// Clear the per-frame state to prepare for a new frame.
    pub fn clear_frame(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
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
                button: Some(b), ..
            }) => {
                self.just_pressed.insert(b);
                self.down.insert(b);
            }
            PointerEvent::Up(PointerButtonEvent {
                button: Some(b), ..
            }) => {
                self.just_released.insert(b);
                self.down.remove(b);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ui_events::pointer::{
        PointerButtonEvent, PointerEvent, PointerId, PointerInfo, PointerState, PointerType,
    };

    fn make_down_event(button: PointerButton) -> PointerEvent {
        PointerEvent::Down(PointerButtonEvent {
            button: Some(button),
            pointer: PointerInfo {
                pointer_id: Some(PointerId::PRIMARY),
                persistent_device_id: None,
                pointer_type: PointerType::Mouse,
            },
            state: PointerState::default(),
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
            state: PointerState::default(),
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
}
