// Copyright 2025 the UI Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Keyboard types
//!
//! This module re-exports the excellent [`keyboard_types`] crate so you can use
//! a consistent set of key codes, modifiers, and locations alongside pointer
//! events from this crate.
//!
//! ## Example: recognizing a common shortcut
//!
//! ```
//! use ui_events::keyboard::{Key, Modifiers};
//!
//! fn is_copy(mods: Modifiers, key: Key) -> bool {
//!     // On most platforms: Ctrl+C, on macOS: Meta+C
//!     (mods.ctrl() || mods.meta()) && key == Key::Character("c".into())
//! }
//! ```

pub use keyboard_types::*;

#[cfg(target_os = "android")]
pub mod android;
