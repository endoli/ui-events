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
// TODO: Remove this.
#![expect(missing_docs, reason = "Not written yet")]

// TODO: We could use the `dpi` crate to distinguish between logical and physical positions
// and sizes, but it isn't yet `no_std` compatible.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

mod buttons;
pub use buttons::{PointerButton, PointerButtons};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PointerId(i32);

// TODO: `f64` seems rather extreme. Can it be smaller?
// TODO: Would be nice to use `dpi::LogicalSize`, but that is not `no_std`
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContactGeometry {
    pub width: f64,
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

// TODO: `auxclick`, `click`, `contextmenu`, 'dblclick' from UIEvents?

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
}

// TODO: Instead of non_exhaustive, could have an `Other(String)` variant.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum PointerType {
    #[default]
    Unknown,
    Mouse,
    Pen,
    Touch,
}

#[derive(Clone, Debug)]
pub struct PointerEvent {
    // TODO: This is different from `PointerEvent` and corresponds to the `UIEvent` `Event.type` field.
    // But should it be?
    pub event_type: String,
    pub timestamp: u64,
    // TODO: Use dpi crate for these?
    // TODO: These are i32 in the browser due to backward compat with `MouseEvent` however in CSSOM, they
    // are `double`
    pub screen_x: i32,
    pub screen_y: i32,
    pub client_x: i32,
    pub client_y: i32,
    // TODO: Do layer_x, layer_y as well?
    pub pointer_id: PointerId,
    // TODO: Replace with modifiers state? But `keyboard_types` is overkill for a bitfield and doesn't do `no_std` anyway.
    pub ctrl_key: bool,
    pub shift_key: bool,
    pub alt_key: bool,
    pub meta_key: bool,
    pub button: PointerButton,
    pub buttons: PointerButtons,
    // TODO: What to do about related_target?
    /// This corresponds to the `width` and `height` fields in the `PointerEvent` specification.
    pub contact_geometry: ContactGeometry,
    pub pressure: f32,
    pub tangential_pressure: f32,
    // TODO: Make these into a more Rust-like structure as well?
    // Handle the specified conversion between tilt and altitude/azimuth as well.
    pub tilt_x: i32,
    pub tilt_y: i32,
    pub twist: i32,
    pub altitude_angle: f64,
    pub azimuth_angle: f64,
    pub pointer_type: PointerType,
    pub is_primary: bool,
    pub persistent_device_id: i32,
    pub coalesced_events: Option<Vec<PointerEvent>>,
    pub predicted_events: Option<Vec<PointerEvent>>,
}

#[cfg(test)]
mod tests {
    // CI will fail unless cargo nextest can execute at least one test per workspace.
    // Delete this dummy test once we have an actual real test.
    #[test]
    fn dummy_test_until_we_have_a_real_test() {}
}
