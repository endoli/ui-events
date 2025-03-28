// Copyright 2025 the Pointer Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// A set of keyboard modifiers.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct Modifiers(u8);

#[expect(missing_docs, reason = "fill in later")]
impl Modifiers {
    const CTRL_KEY: u8 = 1;
    const SHIFT_KEY: u8 = 2;
    const ALT_KEY: u8 = 4;
    const META_KEY: u8 = 8;

    /// Create a new empty set.
    #[inline]
    pub fn new() -> Self {
        Self(0)
    }

    #[must_use]
    #[inline]
    pub fn ctrl_key(self) -> bool {
        self.contains(Self::CTRL_KEY)
    }

    #[inline]
    pub fn set_ctrl_key(&mut self, pressed: bool) {
        self.set_modifier(Self::CTRL_KEY, pressed);
    }

    #[must_use]
    #[inline]
    pub fn shift_key(self) -> bool {
        self.contains(Self::SHIFT_KEY)
    }

    #[inline]
    pub fn set_shift_key(&mut self, pressed: bool) {
        self.set_modifier(Self::SHIFT_KEY, pressed);
    }

    #[must_use]
    #[inline]
    pub fn alt_key(self) -> bool {
        self.contains(Self::ALT_KEY)
    }

    #[inline]
    pub fn set_alt_key(&mut self, pressed: bool) {
        self.set_modifier(Self::ALT_KEY, pressed);
    }

    #[must_use]
    #[inline]
    pub fn meta_key(self) -> bool {
        self.contains(Self::META_KEY)
    }

    #[inline]
    pub fn set_meta_key(&mut self, pressed: bool) {
        self.set_modifier(Self::META_KEY, pressed);
    }

    fn set_modifier(&mut self, modifier: u8, pressed: bool) {
        if pressed {
            self.insert(modifier);
        } else {
            self.remove(modifier);
        }
    }

    /// Add the `modifier` to the set.
    #[inline]
    fn insert(&mut self, modifier: u8) {
        self.0 |= modifier;
    }

    /// Remove the `modifier` from the set.
    #[inline]
    fn remove(&mut self, modifier: u8) {
        self.0 &= !modifier;
    }

    /// Returns `true` if the `modifier` is in the set.
    #[inline]
    fn contains(self, modifier: u8) -> bool {
        (self.0 & modifier) != 0
    }

    /// Returns `true` if the set is empty.
    #[inline]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Clear the set.
    #[inline]
    pub fn clear(&mut self) {
        self.0 = 0;
    }

    /// Count the number of pressed modifiers in the set.
    #[inline]
    pub fn count(self) -> u32 {
        self.0.count_ones()
    }
}

impl core::fmt::Debug for Modifiers {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut tuple = f.debug_tuple("Modifiers");
        if self.contains(Self::CTRL_KEY) {
            tuple.field(&"ctrl");
        }
        if self.contains(Self::SHIFT_KEY) {
            tuple.field(&"shift");
        }
        if self.contains(Self::ALT_KEY) {
            tuple.field(&"alt");
        }
        if self.contains(Self::META_KEY) {
            tuple.field(&"meta");
        }
        tuple.finish()
    }
}

impl core::fmt::Binary for Modifiers {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Binary::fmt(&self.0, f)
    }
}
