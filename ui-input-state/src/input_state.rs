// Copyright 2025 the UI Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::PrimaryPointerState;

/// A stateful view of the input data for a frame, rather than
/// processing it event-by-event.
#[derive(Debug, Default)]
pub struct InputState {
    /// The state of the primary pointer.
    pub primary_pointer: PrimaryPointerState,
}

impl InputState {
    /// Clear the per-frame state to prepare for a new frame.
    pub fn clear_frame(&mut self) {
        self.primary_pointer.clear_frame();
    }
}
