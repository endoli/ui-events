// Copyright 2025 the UI Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(missing_docs)]

use alloc::vec::Vec;
use ui_events::pointer::{PointerButton, PointerButtons, PointerEvent};

/// A stateful view of the primary pointer.
#[derive(Clone, Debug, Default)]
pub struct PrimaryPointerState {
    events: Vec<PointerEvent>,
    pub down: PointerButtons,
}

impl PrimaryPointerState {
    /// Return `true` if the `button` was pressed within the last frame.
    ///
    /// This corresponds to having received a [`PointerEvent::Down`] event
    /// for that button on the primary pointing device.
    pub fn just_pressed(&self, button: PointerButton) -> bool {
        self.events
            .iter()
            .any(|e| matches!(e, &PointerEvent::Down { button: b, .. } if b == Some(button)))
    }

    pub fn just_released(&self, button: PointerButton) -> bool {
        self.events
            .iter()
            .any(|e| matches!(e, &PointerEvent::Up { button: b, .. } if b == Some(button)))
    }

    pub fn auxiliary_just_pressed(&self) -> bool {
        self.just_pressed(PointerButton::Auxiliary)
    }

    pub fn auxiliary_just_released(&self) -> bool {
        self.just_released(PointerButton::Auxiliary)
    }

    pub fn primary_just_pressed(&self) -> bool {
        self.just_pressed(PointerButton::Primary)
    }

    pub fn primary_just_released(&self) -> bool {
        self.just_released(PointerButton::Primary)
    }

    pub fn secondary_just_pressed(&self) -> bool {
        self.just_pressed(PointerButton::Secondary)
    }

    pub fn secondary_just_released(&self) -> bool {
        self.just_released(PointerButton::Secondary)
    }

    pub fn any_down(&self) -> bool {
        !self.down.is_empty()
    }

    pub fn is_down(&self, button: PointerButton) -> bool {
        self.down.contains(button)
    }
}

impl PrimaryPointerState {
    /// Clear the per-frame state to prepare for a new frame.
    pub fn clear_frame(&mut self) {
        self.events.clear();
    }

    pub fn process_pointer_event(&mut self, event: PointerEvent) {
        if !event.is_primary_pointer() {
            return;
        }

        match event {
            PointerEvent::Down {
                button: Some(b), ..
            } => {
                self.down.insert(b);
            }
            PointerEvent::Up {
                button: Some(b), ..
            } => {
                self.down.remove(b);
            }
            _ => {}
        }
        self.events.push(event);
    }
}
