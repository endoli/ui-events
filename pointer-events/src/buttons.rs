// Copyright 2025 the Pointer Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// An indicator of which pointer button was pressed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum PointerButton {
    /// No mouse button.
    None,
    /// Primary button, commonly the left mouse button, touch contact, pen contact.
    Primary,
    /// Secondary button, commonly the right mouse button, pen barrel button.
    Secondary,
    /// Auxiliary button, commonly the middle mouse button.
    Auxiliary,
    /// X1 (back) Mouse.
    X1,
    /// X2 (forward) Mouse.
    X2,
    /// Pen erase button.
    PenEraser,
    /// Other mouse button. This isn't fleshed out yet.
    Other,
}

/// A set of [`PointerButton`]s.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct PointerButtons(u8);

fn button_bit(button: PointerButton) -> u8 {
    match button {
        PointerButton::None => 0,
        PointerButton::Primary => 0b1,
        PointerButton::Secondary => 0b10,
        PointerButton::Auxiliary => 0b100,
        PointerButton::X1 => 0b1000,
        PointerButton::X2 => 0b10000,
        PointerButton::PenEraser => 32,
        // TODO: When we properly do `Other`, this changes
        PointerButton::Other => 0b1000000,
    }
}

impl PointerButtons {
    /// Create a new empty set.
    #[inline]
    pub fn new() -> Self {
        Self(0)
    }

    /// Add the `button` to the set.
    #[inline]
    pub fn insert(&mut self, button: PointerButton) {
        self.0 |= button_bit(button);
    }

    /// Remove the `button` from the set.
    #[inline]
    pub fn remove(&mut self, button: PointerButton) {
        self.0 &= !button_bit(button);
    }

    /// Returns `true` if the `button` is in the set.
    #[inline]
    pub fn contains(self, button: PointerButton) -> bool {
        (self.0 & button_bit(button)) != 0
    }

    /// Returns `true` if the set is empty.
    #[inline]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns `true` if all the `buttons` are in the set.
    #[inline]
    pub fn contains_all(self, buttons: Self) -> bool {
        self.0 & buttons.0 == buttons.0
    }

    /// Adds all the `buttons` to the set.
    pub fn extend(&mut self, buttons: Self) {
        self.0 |= buttons.0;
    }

    /// Clear the set.
    #[inline]
    pub fn clear(&mut self) {
        self.0 = 0;
    }

    /// Count the number of pressed buttons in the set.
    #[inline]
    pub fn count(self) -> u32 {
        self.0.count_ones()
    }
}

impl core::fmt::Debug for PointerButtons {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut tuple = f.debug_tuple("PointerButtons");
        if self.contains(PointerButton::Primary) {
            tuple.field(&"Primary");
        }
        if self.contains(PointerButton::Secondary) {
            tuple.field(&"Secondary");
        }
        if self.contains(PointerButton::Auxiliary) {
            tuple.field(&"Auxiliary");
        }
        if self.contains(PointerButton::X1) {
            tuple.field(&"X1");
        }
        if self.contains(PointerButton::X2) {
            tuple.field(&"X2");
        }
        if self.contains(PointerButton::Other) {
            tuple.field(&"Other");
        }
        tuple.finish()
    }
}

impl core::fmt::Binary for PointerButtons {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Binary::fmt(&self.0, f)
    }
}

impl core::ops::BitOr for PointerButton {
    type Output = PointerButtons;

    fn bitor(self, rhs: Self) -> Self::Output {
        PointerButtons(button_bit(self) | button_bit(rhs))
    }
}

impl core::ops::BitOr<PointerButton> for PointerButtons {
    type Output = Self;

    fn bitor(self, rhs: PointerButton) -> Self {
        Self(self.0 | button_bit(rhs))
    }
}

impl core::ops::BitOrAssign<PointerButton> for PointerButtons {
    fn bitor_assign(&mut self, rhs: PointerButton) {
        self.0 |= button_bit(rhs);
    }
}

impl From<PointerButton> for PointerButtons {
    fn from(button: PointerButton) -> Self {
        Self(button_bit(button))
    }
}
