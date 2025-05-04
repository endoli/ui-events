// Copyright 2025 the UI Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Support routines for converting keyboard data from [`winit`]

use ui_events::keyboard::{Location, Modifiers};
use winit::keyboard::{KeyLocation, ModifiersState};

/// Convert a [`winit::keyboard::KeyLocation`] to a [`ui_events::keyboard::Location`].
pub fn from_winit_location(winit_location: KeyLocation) -> Location {
    match winit_location {
        KeyLocation::Standard => Location::Standard,
        KeyLocation::Left => Location::Left,
        KeyLocation::Right => Location::Right,
        KeyLocation::Numpad => Location::Numpad,
    }
}

/// Convert a [`winit::keyboard::ModifiersState`] to a [`ui_events::keyboard::Modifiers`].
pub fn from_winit_modifier_state(modifiers_state: ModifiersState) -> Modifiers {
    let mut modifiers = Modifiers::default();
    if modifiers_state.control_key() {
        modifiers.insert(Modifiers::CONTROL);
    }
    if modifiers_state.alt_key() {
        modifiers.insert(Modifiers::ALT);
    }
    if modifiers_state.shift_key() {
        modifiers.insert(Modifiers::SHIFT);
    }
    if modifiers_state.super_key() {
        modifiers.insert(Modifiers::META);
    }
    modifiers
}
