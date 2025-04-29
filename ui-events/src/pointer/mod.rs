// Copyright 2025 the UI Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Pointer Event Types

mod buttons;

pub use buttons::{PointerButton, PointerButtons};

/// A unique identifier for the pointing device.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PointerId(i64);

/// The type of device that has generated a pointer event.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum PointerType {
    /// The type of device could not be determined.
    #[default]
    Unknown,
    /// A mouse.
    Mouse,
    /// A pen or stylus.
    Pen,
    /// A touch contact.
    Touch,
}
