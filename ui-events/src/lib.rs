// Copyright 2025 the UI Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! A cross-platform input event abstraction modeled after W3C UI Events specifications.
//!
//! Provides common vocabulary types for working with pointer events (mouse, touch, pen) and
//! keyboard events in a platform-agnostic way. The crate aims to closely follow W3C standards
//! while remaining practical for native application development.
//!
//! Includes support for:
//!
//! - Pointer events (down/move/up, pressure, tilt, etc.)
//! - Keyboard events (key codes, modifiers, location)
//!
//! For integration with [`winit`], use the companion [`ui-events-winit`] adapter crate.
//!
//! ## Features
//!
//! - `std` (enabled by default): Use the Rust standard library.
//! - `kurbo`: Add convenience methods for easily converting dpi positions to kurbo `Point`s.
//!
//! [`ui-events-winit`]: https://docs.rs/ui-events-winit/
//! [`winit`]: https://docs.rs/winit/
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

pub mod keyboard;
pub mod pointer;

mod scroll;

pub use scroll::ScrollDelta;
