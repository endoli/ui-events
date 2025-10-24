// Copyright 2025 the UI Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! UI Theme is a Rust crate which ...
//!
//! ## Features
//!
//! - `std` (enabled by default): Use the Rust standard library.
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

/// The light or dark mode theme.
#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd)]
#[repr(i8)]
pub enum ColorScheme {
    /// The light mode color scheme.
    #[default]
    Light,
    /// The dark mode color scheme.
    Dark,
}
