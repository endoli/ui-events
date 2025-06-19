// Copyright 2025 the UI Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Support routines for converting keyboard data from [`web_sys`].

use ui_events::keyboard::Location;
use web_sys::KeyboardEvent;

/// Convert a [`web_sys::KeyboardEvent::location()`] to a [`ui_events::keyboard::Location`].
pub fn try_from_web_location(location: u32) -> Option<Location> {
    Some(match location {
        KeyboardEvent::DOM_KEY_LOCATION_STANDARD => Location::Standard,
        KeyboardEvent::DOM_KEY_LOCATION_LEFT => Location::Left,
        KeyboardEvent::DOM_KEY_LOCATION_NUMPAD => Location::Numpad,
        KeyboardEvent::DOM_KEY_LOCATION_RIGHT => Location::Right,
        _ => return None,
    })
}
