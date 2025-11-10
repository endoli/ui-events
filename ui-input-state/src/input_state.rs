// Copyright 2025 the UI Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Frame-level input state aggregation.
//!
//! `InputState` groups together [`PrimaryPointerState`] and [`KeyboardState`]
//! and provides a single [`clear_frame`](InputState::clear_frame) call to reset
//! per-frame transitions. Feed events into the contained states as they arrive
//! from your backend, then query during your UI update pass.
//!
//! ## Example:
//!
//! ```no_run
//! use ui_input_state::InputState;
//! use ui_events::keyboard::{KeyboardEvent, KeyState, Key, Location, Code, Modifiers};
//! use ui_events::pointer::{
//!     PointerEvent, PointerButtonEvent, PointerInfo, PointerType, PointerId,
//! };
//! use dpi::{PhysicalPosition, PhysicalSize};
//!
//! let mut input = InputState::default();
//!
//! // Minimal keyboard event
//! let key_down = KeyboardEvent {
//!     state: KeyState::Down,
//!     key: Key::Character("a".into()),
//!     location: Location::Standard,
//!     code: Code::KeyA,
//!     modifiers: Modifiers::empty(),
//!     is_composing: false,
//!     repeat: false,
//! };
//! input.keyboard.process_keyboard_event(key_down);
//!
//! // Minimal pointer down
//! let pointer_down = PointerEvent::Down(PointerButtonEvent {
//!     button: None,
//!     pointer: PointerInfo {
//!         pointer_id: Some(PointerId::PRIMARY),
//!         persistent_device_id: None,
//!         pointer_type: PointerType::Mouse,
//!     },
//!     state: ui_events::pointer::PointerState {
//!         time: 0,
//!         position: PhysicalPosition { x: 0.0, y: 0.0 },
//!         buttons: Default::default(),
//!         modifiers: Modifiers::empty(),
//!         count: 1,
//!         contact_geometry: PhysicalSize { width: 1.0, height: 1.0 },
//!         orientation: Default::default(),
//!         pressure: 0.5,
//!         tangential_pressure: 0.0,
//!         scale_factor: 1.0,
//!     },
//! });
//! input.primary_pointer.process_pointer_event(pointer_down);
//!
//! // Use state during your frame
//! let _any_key = input.keyboard.is_any_down();
//! let _pointer_pos = input.primary_pointer.current_position();
//!
//! // Clear transitions
//! input.clear_frame();
//! ```
use crate::KeyboardState;
use crate::PrimaryPointerState;

/// A stateful view of the input data for a frame, rather than
/// processing it event-by-event.
#[derive(Debug, Default)]
pub struct InputState {
    /// The state of the primary pointer.
    pub primary_pointer: PrimaryPointerState,

    /// The state of the keyboard.
    pub keyboard: KeyboardState,
}

impl InputState {
    /// Clear the per-frame state to prepare for a new frame.
    pub fn clear_frame(&mut self) {
        self.primary_pointer.clear_frame();
        self.keyboard.clear_frame();
    }
}
