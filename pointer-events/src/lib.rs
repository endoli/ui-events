// Copyright 2025 the Pointer Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Pointer Events is a Rust crate which ...
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
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]

// TODO: We could use the `dpi` crate to distinguish between logical and physical positions
// and sizes, but it isn't yet `no_std` compatible.

mod buttons;

pub use buttons::{PointerButton, PointerButtons};

/// A unique identifier for the pointing device.
// TODO: Is this actually available on all platforms, or should
// we do what browsers do and generate something on our own?
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PointerId(i64);

// TODO: `f64` seems rather extreme. Can it be smaller?
// TODO: Would be nice to use `dpi::LogicalSize`, but that is not `no_std`
/// The size of the bounding box of an input, usually touch.
///
/// If this is not provided by the underlying API, platform, or device,
/// then it will default to a single pixel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContactGeometry {
    /// The width of the contact geometry.
    pub width: f64,
    /// The height of the contact geometry.
    pub height: f64,
}

impl Default for ContactGeometry {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
        }
    }
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[expect(missing_docs, reason = "fill in later")]
pub enum PointerEventType {
    PointerOver,
    PointerEnter,
    PointerDown,
    PointerMove,
    PointerRawUpdate,
    PointerUp,
    PointerCancel,
    PointerOut,
    PointerLeave,
    GotPointerCapture,
    LostPointerCapture,
    /// TODO: This is from UI Events and may not belong here.
    AuxClick,
    /// TODO: This is from UI Events and may not belong here.
    Click,
    /// TODO: This is from UI Events and may not belong here.
    ContextMenu,
}

/// The type of device that has generated a pointer event.
// TODO: Instead of non_exhaustive, could have an `Other(String)` variant.
// This would correspond to the vendor-prefixed name discussed in the
// pointer events specification.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[expect(missing_docs, reason = "fill in later")]
pub enum PointerType {
    #[default]
    Unknown,
    Mouse,
    Pen,
    Touch,
}

#[cfg(test)]
mod tests {
    // CI will fail unless cargo nextest can execute at least one test per workspace.
    // Delete this dummy test once we have an actual real test.
    #[test]
    fn dummy_test_until_we_have_a_real_test() {}
}
