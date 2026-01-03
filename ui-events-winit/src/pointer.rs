// Copyright 2025 the UI Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Support routines for converting pointer data from [`winit`].

use ui_events::pointer::PointerButton;
use winit::event::MouseButton;

/// Try to make a [`PointerButton`] from a [`MouseButton`].
///
/// Because values of [`MouseButton::Other`] can start at 0, they are mapped
/// to the arbitrary buttons B7..B32.
/// Values greater than 25 will not be mapped.
pub fn try_from_winit_button(b: MouseButton) -> Option<PointerButton> {
    Some(match b {
        MouseButton::Left => PointerButton::Primary,
        MouseButton::Right => PointerButton::Secondary,
        MouseButton::Middle => PointerButton::Auxiliary,
        MouseButton::Back => PointerButton::X1,
        MouseButton::Forward => PointerButton::X2,
        MouseButton::Button6 => PointerButton::PenEraser,
        MouseButton::Button7 => PointerButton::B7,
        MouseButton::Button8 => PointerButton::B8,
        MouseButton::Button9 => PointerButton::B9,
        MouseButton::Button10 => PointerButton::B10,
        MouseButton::Button11 => PointerButton::B11,
        MouseButton::Button12 => PointerButton::B12,
        MouseButton::Button13 => PointerButton::B13,
        MouseButton::Button14 => PointerButton::B14,
        MouseButton::Button15 => PointerButton::B15,
        MouseButton::Button16 => PointerButton::B16,
        MouseButton::Button17 => PointerButton::B17,
        MouseButton::Button18 => PointerButton::B18,
        MouseButton::Button19 => PointerButton::B19,
        MouseButton::Button20 => PointerButton::B20,
        MouseButton::Button21 => PointerButton::B21,
        MouseButton::Button22 => PointerButton::B22,
        MouseButton::Button23 => PointerButton::B23,
        MouseButton::Button24 => PointerButton::B24,
        MouseButton::Button25 => PointerButton::B25,
        MouseButton::Button26 => PointerButton::B26,
        MouseButton::Button27 => PointerButton::B27,
        MouseButton::Button28 => PointerButton::B28,
        MouseButton::Button29 => PointerButton::B29,
        MouseButton::Button30 => PointerButton::B30,
        MouseButton::Button31 => PointerButton::B31,
        MouseButton::Button32 => PointerButton::B32,
    })
}
