// Copyright 2025 the UI Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Support routines for converting pointer data from [`web-sys`].

use ui_events::pointer::PointerButton;

/// Try to make a [`PointerButton`] from a [`web_sys::MouseEvent::button`].
///
/// Values less than 0 or greater than 31 will not be mapped.
///
/// This corresponds to §5.1.1.2 of the Pointer Events Level 2
/// specification.
pub fn try_from_web_button(b: i16) -> Option<PointerButton> {
    Some(match b {
        0 => PointerButton::Primary,
        1 => PointerButton::Secondary,
        2 => PointerButton::Auxiliary,
        3 => PointerButton::X1,
        4 => PointerButton::X2,
        5 => PointerButton::PenEraser,
        6 => PointerButton::B7,
        7 => PointerButton::B8,
        8 => PointerButton::B9,
        9 => PointerButton::B10,
        10 => PointerButton::B11,
        11 => PointerButton::B12,
        12 => PointerButton::B13,
        13 => PointerButton::B14,
        14 => PointerButton::B15,
        15 => PointerButton::B16,
        16 => PointerButton::B17,
        17 => PointerButton::B18,
        18 => PointerButton::B19,
        19 => PointerButton::B20,
        20 => PointerButton::B21,
        21 => PointerButton::B22,
        22 => PointerButton::B23,
        23 => PointerButton::B24,
        24 => PointerButton::B25,
        25 => PointerButton::B26,
        26 => PointerButton::B27,
        27 => PointerButton::B28,
        28 => PointerButton::B29,
        29 => PointerButton::B30,
        30 => PointerButton::B31,
        31 => PointerButton::B32,
        _ => {
            return None;
        }
    })
}
