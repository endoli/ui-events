// Copyright 2025 the UI Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Frame-oriented input state built on `ui-events`.
//!
//! This crate provides simple state containers to make input handling easier in
//! immediate-mode or frame-based UIs. Instead of reacting to each event
//! individually, you feed pointer and keyboard events into the state, query the
//! current and per-frame information during your update, and then call
//! [`InputState::clear_frame`] at the end of the frame.
//!
//! ## What it provides:
//!
//! - [`PrimaryPointerState`]: current pointer state, coalesced and predicted motion,
//!   per-frame button transitions, and helpers for motion in physical/logical units.
//! - [`KeyboardState`]: current modifiers, keys down, and per-frame key transitions.
//! - [`InputState`]: a convenience container bundling both states and a per-frame clear.
//!
//! ## Typical lifecycle per frame:
//!
//! 1. Receive backend events and convert them to `ui-events` types.
//! 2. Update `PrimaryPointerState` and `KeyboardState` with the events.
//! 3. Read state during your UI update (e.g. check just pressed, motion, etc.).
//! 4. Call [`InputState::clear_frame`] before the next frame.
//!
//! ## Example (sketch):
//!
//! ```no_run
//! use ui_input_state::{InputState, PrimaryPointerState, KeyboardState};
//! use ui_events::pointer::PointerEvent;
//! use ui_events::keyboard::KeyboardEvent;
//!
//! let mut input = InputState::default();
//!
//! // 1-2) In your event loop, feed events into state
//! fn on_pointer_event(input: &mut InputState, e: PointerEvent) {
//!     input.primary_pointer.process_pointer_event(e);
//! }
//! fn on_keyboard_event(input: &mut InputState, e: KeyboardEvent) {
//!     input.keyboard.process_keyboard_event(e);
//! }
//!
//! // 3) During your update pass, read state
//! fn update(input: &InputState) {
//!     if input.primary_pointer.is_primary_just_pressed() {
//!         // Begin a drag, for example
//!     }
//!     if input.keyboard.key_str_just_pressed("z") && input.keyboard.modifiers.ctrl() {
//!         // Ctrl+Z
//!     }
//! }
//!
//! // 4) At the end of the frame, clear per-frame transitions
//! fn end_frame(input: &mut InputState) { input.clear_frame(); }
//! ```
//!
//! ## Coordinates and units
//!
//! Pointer positions are stored in physical pixels with a Y-down axis, as in
//! `ui-events`. Use [`PrimaryPointerState::current_logical_position`] and
//! [`PrimaryPointerState::logical_motion`] to work in logical units.
//!
//! ## Features
//!
//! - `std` (enabled by default): Use the Rust standard library.
//! - `libm`: Enable `ui-events/libm` transitively for `no_std` environments.
// LINEBENDER LINT SET - lib.rs - v3
// See https://linebender.org/wiki/canonical-lints/
// These lints shouldn't apply to examples or tests.
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
// These lints shouldn't apply to examples.
#![warn(clippy::print_stdout, clippy::print_stderr)]
// Targeting e.g. 32-bit means structs containing usize can give false positives for 64-bit.
#![cfg_attr(target_pointer_width = "64", warn(clippy::trivially_copy_pass_by_ref))]
// END LINEBENDER LINT SET
#![no_std]

extern crate alloc;

mod input_state;
mod keyboard_state;
mod primary_pointer_state;

pub use crate::input_state::InputState;
pub use crate::keyboard_state::KeyboardState;
pub use crate::primary_pointer_state::PrimaryPointerState;
