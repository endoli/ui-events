// Copyright 2025 the UI Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use ui_events::keyboard::{Code, Key, KeyState, KeyboardEvent, Location, Modifiers};

extern crate alloc;
use alloc::vec::Vec;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
struct KeyInfo(Key, Location, Code);

/// A stateful view of the primary pointer.
#[derive(Clone, Debug, Default)]
pub struct KeyboardState {
    /// Keys that were pressed during the current frame.
    just_pressed: Vec<KeyInfo>,
    /// Keys that were released during the current frame.
    just_released: Vec<KeyInfo>,
    /// Keys that are currently being held down.
    down: Vec<KeyInfo>,
    /// Modifiers state.
    pub modifiers: Modifiers,
}

impl KeyboardState {
    /// Return `true` if the `key` was pressed within the last frame with
    /// any [`Location`].
    pub fn key_just_pressed(&self, key: Key) -> bool {
        self.just_pressed.iter().any(|KeyInfo(k, ..)| k == &key)
    }

    /// Return `true` if the `key` was pressed within the last frame with `location`.
    pub fn key_just_pressed_location(&self, key: Key, location: Location) -> bool {
        self.just_pressed
            .iter()
            .any(|KeyInfo(k, l, _)| k == &key && l == &location)
    }

    /// Return `true` if the `Code` was pressed within the last frame.
    pub fn code_just_pressed(&self, code: Code) -> bool {
        self.just_pressed.iter().any(|KeyInfo(_, _, c)| c == &code)
    }

    /// Return `true` if the `key` was released within the last frame with
    /// any [`Location`].
    pub fn key_just_released(&self, key: Key) -> bool {
        self.just_released.iter().any(|KeyInfo(k, ..)| k == &key)
    }

    /// Return `true` if the `key` was released within the last frame with `location`.
    pub fn key_just_released_location(&self, key: Key, location: Location) -> bool {
        self.just_released
            .iter()
            .any(|KeyInfo(k, l, _)| k == &key && l == &location)
    }

    /// Return `true` if the `Code` was released within the last frame.
    pub fn code_just_released(&self, code: Code) -> bool {
        self.just_released.iter().any(|KeyInfo(_, _, c)| c == &code)
    }

    /// Return `true` if any key is currently held down.
    pub fn is_any_down(&self) -> bool {
        !self.down.is_empty()
    }

    /// Return `true` if the `key` is currently pressed with any [`Location`].
    pub fn key_down(&self, key: Key) -> bool {
        self.down.iter().any(|KeyInfo(k, ..)| k == &key)
    }

    /// Return `true` if the `key` is currently pressed with `location`.
    pub fn key_down_location(&self, key: Key, location: Location) -> bool {
        self.down
            .iter()
            .any(|KeyInfo(k, l, _)| k == &key && l == &location)
    }

    /// Return `true` if the `code` is currently pressed with any [`Location`].
    pub fn code_down(&self, code: Code) -> bool {
        self.down.iter().any(|KeyInfo(_, _, c)| c == &code)
    }

    /// Clear the per-frame state to prepare for a new frame.
    pub fn clear_frame(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    /// Update the state based on the given pointer event.
    ///
    /// Only events from the primary pointer are processed. Press and release
    /// events update the `just_pressed`, `just_released`, and `down` states.
    pub fn process_keyboard_event(&mut self, event: KeyboardEvent) {
        self.modifiers = event.modifiers;
        let info = KeyInfo(event.key, event.location, event.code);
        match event.state {
            KeyState::Down => {
                self.just_pressed.push(info.clone());
                self.down.push(info.clone());
            }
            KeyState::Up => {
                self.just_released.push(info.clone());
                self.down.retain(|other| other != &info);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ui_events::keyboard::{Code, NamedKey};

    fn make_key_down_event(key: Key) -> KeyboardEvent {
        KeyboardEvent {
            state: KeyState::Down,
            key,
            location: Location::Standard,
            code: Code::Unidentified,
            modifiers: Default::default(),
            is_composing: false,
            repeat: false,
        }
    }

    fn make_key_up_event(key: Key) -> KeyboardEvent {
        KeyboardEvent {
            state: KeyState::Up,
            key,
            location: Location::Standard,
            code: Code::Unidentified,
            modifiers: Default::default(),
            is_composing: false,
            repeat: false,
        }
    }

    #[test]
    fn press_and_hold_a() {
        let mut state = KeyboardState::default();
        state.process_keyboard_event(make_key_down_event(Key::Character("A".into())));

        assert!(state.key_just_pressed(Key::Character("A".into())));
        assert!(state.key_down(Key::Character("A".into())));
        assert!(!state.key_just_released(Key::Character("A".into())));

        state.clear_frame();

        assert!(!state.key_just_pressed(Key::Character("A".into())));
        assert!(state.key_down(Key::Character("A".into())));
    }

    #[test]
    fn press_and_release_a() {
        let mut state = KeyboardState::default();
        state.process_keyboard_event(make_key_down_event(Key::Character("A".into())));
        state.process_keyboard_event(make_key_up_event(Key::Character("A".into())));

        assert!(state.key_just_pressed(Key::Character("A".into())));
        assert!(state.key_just_released(Key::Character("A".into())));
        assert!(!state.key_down(Key::Character("A".into())));
    }

    #[test]
    fn release_after_hold() {
        let mut state = KeyboardState::default();
        state.process_keyboard_event(make_key_down_event(Key::Character("A".into())));
        state.clear_frame();
        state.process_keyboard_event(make_key_up_event(Key::Character("A".into())));

        assert!(!state.key_just_pressed(Key::Character("A".into())));
        assert!(state.key_just_released(Key::Character("A".into())));
        assert!(!state.key_down(Key::Character("A".into())));
    }

    fn make_code_down_event(code: Code) -> KeyboardEvent {
        KeyboardEvent {
            state: KeyState::Down,
            key: Key::Named(NamedKey::Unidentified),
            location: Location::Standard,
            code,
            modifiers: Default::default(),
            is_composing: false,
            repeat: false,
        }
    }

    fn make_code_up_event(code: Code) -> KeyboardEvent {
        KeyboardEvent {
            state: KeyState::Up,
            key: Key::Named(NamedKey::Unidentified),
            location: Location::Standard,
            code,
            modifiers: Default::default(),
            is_composing: false,
            repeat: false,
        }
    }

    #[test]
    fn press_and_hold_a_code() {
        let mut state = KeyboardState::default();
        state.process_keyboard_event(make_code_down_event(Code::KeyA));

        assert!(state.code_just_pressed(Code::KeyA));
        assert!(state.code_down(Code::KeyA));
        assert!(!state.code_just_released(Code::KeyA));

        state.clear_frame();

        assert!(!state.code_just_pressed(Code::KeyA));
        assert!(state.code_down(Code::KeyA));
    }

    #[test]
    fn press_and_release_a_code() {
        let mut state = KeyboardState::default();
        state.process_keyboard_event(make_code_down_event(Code::KeyA));
        state.process_keyboard_event(make_code_up_event(Code::KeyA));

        assert!(state.code_just_pressed(Code::KeyA));
        assert!(state.code_just_released(Code::KeyA));
        assert!(!state.code_down(Code::KeyA));
    }

    #[test]
    fn release_after_hold_code() {
        let mut state = KeyboardState::default();
        state.process_keyboard_event(make_code_down_event(Code::KeyA));
        state.clear_frame();
        state.process_keyboard_event(make_code_up_event(Code::KeyA));

        assert!(!state.code_just_pressed(Code::KeyA));
        assert!(state.code_just_released(Code::KeyA));
        assert!(!state.code_down(Code::KeyA));
    }
}
