// Copyright 2025 the UI Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Support routines for converting keyboard data from [`web_sys`].

use alloc::string::ToString;
use ui_events::keyboard::{
    Code, Key, KeyState, KeyboardEvent as UiKeyboardEvent, Location, Modifiers, NamedKey,
};
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

/// Convert a DOM `keydown` event to a `ui-events` [`KeyboardEvent`](ui_events::keyboard::KeyboardEvent).
pub fn from_web_keydown_event(e: &KeyboardEvent) -> UiKeyboardEvent {
    from_web_keyboard_event_with_state(e, KeyState::Down)
}

/// Convert a DOM `keyup` event to a `ui-events` [`KeyboardEvent`](ui_events::keyboard::KeyboardEvent).
pub fn from_web_keyup_event(e: &KeyboardEvent) -> UiKeyboardEvent {
    from_web_keyboard_event_with_state(e, KeyState::Up)
}

/// Convert a DOM [`web_sys::KeyboardEvent`] to a `ui-events` [`Keyboard event`](ui_events::keyboard::KeyboardEvent).
///
/// Infers [`KeyState`] from `event.type`: `"keydown"` => [`KeyState::Down`], `"keyup"` => [`KeyState::Up`].
/// Other types default to [`KeyState::Down`].
pub fn from_web_keyboard_event(e: &KeyboardEvent) -> UiKeyboardEvent {
    let state = match e.type_().as_str() {
        "keyup" => KeyState::Up,
        // Treat keydown/keypress/others uniformly as Down for our purposes.
        _ => KeyState::Down,
    };
    from_web_keyboard_event_with_state(e, state)
}

fn from_web_keyboard_event_with_state(e: &KeyboardEvent, state: KeyState) -> UiKeyboardEvent {
    UiKeyboardEvent {
        key: key_from_web_key_string(e.key().as_str()),
        code: code_from_web_code_string(e.code().as_str()),
        modifiers: modifiers_from_web(e),
        location: try_from_web_location(e.location()).unwrap_or(Location::Standard),
        is_composing: e.is_composing(),
        repeat: e.repeat(),
        state,
    }
}

fn key_from_web_key_string(s: &str) -> Key {
    // Try mapping common named keys first.
    if let Some(named) = named_key_from_web_key_string(s) {
        return Key::Named(named);
    }
    // Heuristic: single Unicode scalar => Character; otherwise leave Unidentified.
    if s.chars().count() == 1 {
        Key::Character(s.to_string())
    } else {
        Key::Named(NamedKey::Unidentified)
    }
}

fn modifiers_from_web(e: &KeyboardEvent) -> Modifiers {
    let mut m = Modifiers::default();
    if e.ctrl_key() {
        m.insert(Modifiers::CONTROL);
    }
    if e.alt_key() {
        m.insert(Modifiers::ALT);
    }
    if e.shift_key() {
        m.insert(Modifiers::SHIFT);
    }
    if e.meta_key() {
        m.insert(Modifiers::META);
    }
    m
}

fn named_key_from_web_key_string(s: &str) -> Option<NamedKey> {
    use NamedKey as NK;
    let out = match s {
        // Modifiers and locks
        "Shift" => NK::Shift,
        "Control" => NK::Control,
        "Alt" => NK::Alt,
        "Meta" => NK::Meta,
        "AltGraph" => NK::AltGraph,
        "CapsLock" => NK::CapsLock,
        "NumLock" => NK::NumLock,
        "ScrollLock" => NK::ScrollLock,

        // Navigation / editing
        "Backspace" => NK::Backspace,
        "Tab" => NK::Tab,
        "Enter" => NK::Enter,
        "Escape" => NK::Escape,
        "Home" => NK::Home,
        "End" => NK::End,
        "PageUp" => NK::PageUp,
        "PageDown" => NK::PageDown,
        "Insert" => NK::Insert,
        "Delete" => NK::Delete,
        "ArrowLeft" => NK::ArrowLeft,
        "ArrowRight" => NK::ArrowRight,
        "ArrowUp" => NK::ArrowUp,
        "ArrowDown" => NK::ArrowDown,

        // System / misc
        "ContextMenu" => NK::ContextMenu,
        "PrintScreen" => NK::PrintScreen,
        "Pause" => NK::Pause,
        "Help" => NK::Help,
        "BrightnessUp" => NK::BrightnessUp,
        "BrightnessDown" => NK::BrightnessDown,
        "Power" => NK::Power,
        "PowerOff" => NK::PowerOff,
        "LogOff" => NK::LogOff,
        "Eject" => NK::Eject,
        "WakeUp" => NK::WakeUp,
        "Sleep" => NK::Standby,

        // IME / language
        "Convert" => NK::Convert,
        "NonConvert" => NK::NonConvert,
        "KanaMode" => NK::KanaMode,

        // Function keys
        "F1" => NK::F1,
        "F2" => NK::F2,
        "F3" => NK::F3,
        "F4" => NK::F4,
        "F5" => NK::F5,
        "F6" => NK::F6,
        "F7" => NK::F7,
        "F8" => NK::F8,
        "F9" => NK::F9,
        "F10" => NK::F10,
        "F11" => NK::F11,
        "F12" => NK::F12,
        "F13" => NK::F13,
        "F14" => NK::F14,
        "F15" => NK::F15,
        "F16" => NK::F16,
        "F17" => NK::F17,
        "F18" => NK::F18,
        "F19" => NK::F19,
        "F20" => NK::F20,
        "F21" => NK::F21,
        "F22" => NK::F22,
        "F23" => NK::F23,
        "F24" => NK::F24,

        // Common media/system keys (best-effort)
        "VolumeUp" => NK::AudioVolumeUp,
        "VolumeDown" => NK::AudioVolumeDown,
        "VolumeMute" | "AudioVolumeMute" => NK::AudioVolumeMute,
        "MediaPlayPause" => NK::MediaPlayPause,
        "MediaStop" => NK::MediaStop,
        "MediaTrackNext" => NK::MediaTrackNext,
        "MediaTrackPrevious" => NK::MediaTrackPrevious,
        "MediaPlay" => NK::MediaPlay,
        "MediaPause" => NK::MediaPause,
        "MediaRecord" => NK::MediaRecord,
        "MediaRewind" => NK::MediaRewind,
        "MediaFastForward" => NK::MediaFastForward,
        "MediaClose" => NK::MediaClose,

        // Editing / control
        "Clear" => NK::Clear,
        "Execute" => NK::Execute,
        "Print" => NK::Print,
        "Redo" => NK::Redo,
        "Undo" => NK::Undo,
        "Copy" => NK::Copy,
        "Cut" => NK::Cut,
        "Paste" => NK::Paste,
        "Select" => NK::Select,
        "Find" => NK::Find,
        "Open" => NK::Open,
        "Save" => NK::Save,
        "Props" => NK::Props,

        // Browser keys
        "BrowserBack" => NK::BrowserBack,
        "BrowserForward" => NK::BrowserForward,
        "BrowserHome" => NK::BrowserHome,
        "BrowserRefresh" => NK::BrowserRefresh,
        "BrowserSearch" => NK::BrowserSearch,
        "BrowserStop" => NK::BrowserStop,
        "BrowserFavorites" => NK::BrowserFavorites,

        _ => return None,
    };
    Some(out)
}

fn code_from_web_code_string(s: &str) -> Code {
    use Code as C;
    match s {
        // Function modifier keys
        "Fn" => C::Fn,
        "FnLock" => C::FnLock,
        // Letters
        "KeyA" => C::KeyA,
        "KeyB" => C::KeyB,
        "KeyC" => C::KeyC,
        "KeyD" => C::KeyD,
        "KeyE" => C::KeyE,
        "KeyF" => C::KeyF,
        "KeyG" => C::KeyG,
        "KeyH" => C::KeyH,
        "KeyI" => C::KeyI,
        "KeyJ" => C::KeyJ,
        "KeyK" => C::KeyK,
        "KeyL" => C::KeyL,
        "KeyM" => C::KeyM,
        "KeyN" => C::KeyN,
        "KeyO" => C::KeyO,
        "KeyP" => C::KeyP,
        "KeyQ" => C::KeyQ,
        "KeyR" => C::KeyR,
        "KeyS" => C::KeyS,
        "KeyT" => C::KeyT,
        "KeyU" => C::KeyU,
        "KeyV" => C::KeyV,
        "KeyW" => C::KeyW,
        "KeyX" => C::KeyX,
        "KeyY" => C::KeyY,
        "KeyZ" => C::KeyZ,

        // Top-row digits
        "Digit0" => C::Digit0,
        "Digit1" => C::Digit1,
        "Digit2" => C::Digit2,
        "Digit3" => C::Digit3,
        "Digit4" => C::Digit4,
        "Digit5" => C::Digit5,
        "Digit6" => C::Digit6,
        "Digit7" => C::Digit7,
        "Digit8" => C::Digit8,
        "Digit9" => C::Digit9,

        // Numpad digits
        "Numpad0" => C::Numpad0,
        "Numpad1" => C::Numpad1,
        "Numpad2" => C::Numpad2,
        "Numpad3" => C::Numpad3,
        "Numpad4" => C::Numpad4,
        "Numpad5" => C::Numpad5,
        "Numpad6" => C::Numpad6,
        "Numpad7" => C::Numpad7,
        "Numpad8" => C::Numpad8,
        "Numpad9" => C::Numpad9,

        // Editing / whitespace
        "Backspace" => C::Backspace,
        "Tab" => C::Tab,
        "Enter" => C::Enter,
        "Escape" => C::Escape,
        "Space" => C::Space,

        // Brackets and punctuation
        "Backquote" => C::Backquote,
        "Minus" => C::Minus,
        "Equal" => C::Equal,
        "BracketLeft" => C::BracketLeft,
        "BracketRight" => C::BracketRight,
        "Backslash" => C::Backslash,
        "Semicolon" => C::Semicolon,
        "Quote" => C::Quote,
        "Comma" => C::Comma,
        "Period" => C::Period,
        "Slash" => C::Slash,

        // Navigation
        "Home" => C::Home,
        "End" => C::End,
        "PageUp" => C::PageUp,
        "PageDown" => C::PageDown,
        "Insert" => C::Insert,
        "Delete" => C::Delete,
        "ArrowLeft" => C::ArrowLeft,
        "ArrowRight" => C::ArrowRight,
        "ArrowUp" => C::ArrowUp,
        "ArrowDown" => C::ArrowDown,

        // Modifiers
        "ShiftLeft" => C::ShiftLeft,
        "ShiftRight" => C::ShiftRight,
        "ControlLeft" => C::ControlLeft,
        "ControlRight" => C::ControlRight,
        "AltLeft" => C::AltLeft,
        "AltRight" => C::AltRight,
        "MetaLeft" => C::MetaLeft,
        "MetaRight" => C::MetaRight,
        "CapsLock" => C::CapsLock,
        "NumLock" => C::NumLock,
        "ScrollLock" => C::ScrollLock,

        // Function keys
        "F1" => C::F1,
        "F2" => C::F2,
        "F3" => C::F3,
        "F4" => C::F4,
        "F5" => C::F5,
        "F6" => C::F6,
        "F7" => C::F7,
        "F8" => C::F8,
        "F9" => C::F9,
        "F10" => C::F10,
        "F11" => C::F11,
        "F12" => C::F12,
        "F13" => C::F13,
        "F14" => C::F14,
        "F15" => C::F15,
        "F16" => C::F16,
        "F17" => C::F17,
        "F18" => C::F18,
        "F19" => C::F19,
        "F20" => C::F20,
        "F21" => C::F21,
        "F22" => C::F22,
        "F23" => C::F23,
        "F24" => C::F24,
        "F25" => C::F25,
        "F26" => C::F26,
        "F27" => C::F27,
        "F28" => C::F28,
        "F29" => C::F29,
        "F30" => C::F30,
        "F31" => C::F31,
        "F32" => C::F32,
        "F33" => C::F33,
        "F34" => C::F34,
        "F35" => C::F35,

        // Numpad operators
        "NumpadAdd" => C::NumpadAdd,
        "NumpadSubtract" => C::NumpadSubtract,
        "NumpadMultiply" => C::NumpadMultiply,
        "NumpadDivide" => C::NumpadDivide,
        "NumpadDecimal" => C::NumpadDecimal,
        "NumpadEnter" => C::NumpadEnter,

        // International and contextual
        "IntlBackslash" => C::IntlBackslash,
        "IntlRo" => C::IntlRo,
        "IntlYen" => C::IntlYen,
        "ContextMenu" => C::ContextMenu,
        "Convert" => C::Convert,
        "KanaMode" => C::KanaMode,
        "Lang1" => C::Lang1,
        "Lang2" => C::Lang2,
        "Lang3" => C::Lang3,
        "Lang4" => C::Lang4,
        "Lang5" => C::Lang5,
        "NonConvert" => C::NonConvert,
        "Help" => C::Help,
        "PrintScreen" => C::PrintScreen,
        "Pause" => C::Pause,

        // Additional numpad variants found on some keyboards
        "NumpadBackspace" => C::NumpadBackspace,
        "NumpadClear" => C::NumpadClear,
        "NumpadClearEntry" => C::NumpadClearEntry,
        "NumpadComma" => C::NumpadComma,
        "NumpadEqual" => C::NumpadEqual,
        "NumpadHash" => C::NumpadHash,
        "NumpadMemoryAdd" => C::NumpadMemoryAdd,
        "NumpadMemoryClear" => C::NumpadMemoryClear,
        "NumpadMemoryRecall" => C::NumpadMemoryRecall,
        "NumpadMemoryStore" => C::NumpadMemoryStore,
        "NumpadMemorySubtract" => C::NumpadMemorySubtract,
        "NumpadParenLeft" => C::NumpadParenLeft,
        "NumpadParenRight" => C::NumpadParenRight,
        "NumpadStar" => C::NumpadStar,

        // Browser / system / media and power
        "BrowserBack" => C::BrowserBack,
        "BrowserFavorites" => C::BrowserFavorites,
        "BrowserForward" => C::BrowserForward,
        "BrowserHome" => C::BrowserHome,
        "BrowserRefresh" => C::BrowserRefresh,
        "BrowserSearch" => C::BrowserSearch,
        "BrowserStop" => C::BrowserStop,
        "Eject" => C::Eject,
        "LaunchApp1" => C::LaunchApp1,
        "LaunchApp2" => C::LaunchApp2,
        "LaunchMail" => C::LaunchMail,
        "MediaPlayPause" => C::MediaPlayPause,
        "MediaSelect" => C::MediaSelect,
        "MediaStop" => C::MediaStop,
        "MediaTrackNext" => C::MediaTrackNext,
        "MediaTrackPrevious" => C::MediaTrackPrevious,
        "Power" => C::Power,
        "Sleep" => C::Sleep,
        "AudioVolumeDown" => C::AudioVolumeDown,
        "AudioVolumeMute" => C::AudioVolumeMute,
        "AudioVolumeUp" => C::AudioVolumeUp,
        "WakeUp" => C::WakeUp,
        "Abort" => C::Abort,
        "Resume" => C::Resume,
        "Suspend" => C::Suspend,
        "Again" => C::Again,
        "Copy" => C::Copy,
        "Cut" => C::Cut,
        "Find" => C::Find,
        "Open" => C::Open,
        "Paste" => C::Paste,
        "Props" => C::Props,
        "Select" => C::Select,
        "Undo" => C::Undo,
        "Hiragana" => C::Hiragana,
        "Katakana" => C::Katakana,

        _ => C::Unidentified,
    }
}
