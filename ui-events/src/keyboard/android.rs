// Copyright 2006 The Android Open Source Project
// Copyright 2025 the UI Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Translate Android keycodes to their closest equivalent `Code` and `NamedKey`.
//!
//! The `KEYCODE` values and documentation thereof were derived from [`KeyEvent.java` as of May 2 2025][KeyEvent]
//! and some of them were revised or edited for correctness, formatting, and typos.
//!
//! [KeyEvent]: <https://android.googlesource.com/platform/frameworks/base/+/413c6473c766bce625496a6290b3ee9e5c56bcea/core/java/android/view/KeyEvent.java>

/// Unknown key code.
pub const KEYCODE_UNKNOWN: i32 = 0;

/// Soft Left key.
///
/// Usually situated below the display on phones and used as a multi-function
/// feature key for selecting a software defined function shown on the bottom left
/// of the display.
pub const KEYCODE_SOFT_LEFT: i32 = 1;

/// Soft Right key.
///
/// Usually situated below the display on phones and used as a multi-function
/// feature key for selecting a software defined function shown on the bottom right
/// of the display.
pub const KEYCODE_SOFT_RIGHT: i32 = 2;

/// Home key.
///
/// This key is handled by the framework and is never delivered to applications.
pub const KEYCODE_HOME: i32 = 3;

/// Back key.
pub const KEYCODE_BACK: i32 = 4;

/// Call key.
pub const KEYCODE_CALL: i32 = 5;

/// End Call key.
pub const KEYCODE_ENDCALL: i32 = 6;

/// '0' key.
pub const KEYCODE_0: i32 = 7;

/// '1' key.
pub const KEYCODE_1: i32 = 8;

/// '2' key.
pub const KEYCODE_2: i32 = 9;

/// '3' key.
pub const KEYCODE_3: i32 = 10;

/// '4' key.
pub const KEYCODE_4: i32 = 11;

/// '5' key.
pub const KEYCODE_5: i32 = 12;

/// '6' key.
pub const KEYCODE_6: i32 = 13;

/// '7' key.
pub const KEYCODE_7: i32 = 14;

/// '8' key.
pub const KEYCODE_8: i32 = 15;

/// '9' key.
pub const KEYCODE_9: i32 = 16;

/// '*' key.
pub const KEYCODE_STAR: i32 = 17;

/// '#' key.
pub const KEYCODE_POUND: i32 = 18;

/// Directional Pad Up key.
///
/// May also be synthesized from trackball motions.
pub const KEYCODE_DPAD_UP: i32 = 19;

/// Directional Pad Down key.
///
/// May also be synthesized from trackball motions.
pub const KEYCODE_DPAD_DOWN: i32 = 20;

/// Directional Pad Left key.
///
/// May also be synthesized from trackball motions.
pub const KEYCODE_DPAD_LEFT: i32 = 21;

/// Directional Pad Right key.
///
/// May also be synthesized from trackball motions.
pub const KEYCODE_DPAD_RIGHT: i32 = 22;

/// Directional Pad Center key.
///
/// May also be synthesized from trackball motions.
pub const KEYCODE_DPAD_CENTER: i32 = 23;

/// Volume Up key.
///
/// Adjusts the speaker volume up.
pub const KEYCODE_VOLUME_UP: i32 = 24;

/// Volume Down key.
///
/// Adjusts the speaker volume down.
pub const KEYCODE_VOLUME_DOWN: i32 = 25;

/// Power key.
pub const KEYCODE_POWER: i32 = 26;

/// Camera key.
///
/// Used to launch a camera application or take pictures.
pub const KEYCODE_CAMERA: i32 = 27;

/// Clear key.
pub const KEYCODE_CLEAR: i32 = 28;

/// 'A' key.
pub const KEYCODE_A: i32 = 29;

/// 'B' key.
pub const KEYCODE_B: i32 = 30;

/// 'C' key.
pub const KEYCODE_C: i32 = 31;

/// 'D' key.
pub const KEYCODE_D: i32 = 32;

/// 'E' key.
pub const KEYCODE_E: i32 = 33;

/// 'F' key.
pub const KEYCODE_F: i32 = 34;

/// 'G' key.
pub const KEYCODE_G: i32 = 35;

/// 'H' key.
pub const KEYCODE_H: i32 = 36;

/// 'I' key.
pub const KEYCODE_I: i32 = 37;

/// 'J' key.
pub const KEYCODE_J: i32 = 38;

/// 'K' key.
pub const KEYCODE_K: i32 = 39;

/// 'L' key.
pub const KEYCODE_L: i32 = 40;

/// 'M' key.
pub const KEYCODE_M: i32 = 41;

/// 'N' key.
pub const KEYCODE_N: i32 = 42;

/// 'O' key.
pub const KEYCODE_O: i32 = 43;

/// 'P' key.
pub const KEYCODE_P: i32 = 44;

/// 'Q' key.
pub const KEYCODE_Q: i32 = 45;

/// 'R' key.
pub const KEYCODE_R: i32 = 46;

/// 'S' key.
pub const KEYCODE_S: i32 = 47;

/// 'T' key.
pub const KEYCODE_T: i32 = 48;

/// 'U' key.
pub const KEYCODE_U: i32 = 49;

/// 'V' key.
pub const KEYCODE_V: i32 = 50;

/// 'W' key.
pub const KEYCODE_W: i32 = 51;

/// 'X' key.
pub const KEYCODE_X: i32 = 52;

/// 'Y' key.
pub const KEYCODE_Y: i32 = 53;

/// 'Z' key.
pub const KEYCODE_Z: i32 = 54;

/// ',' key.
pub const KEYCODE_COMMA: i32 = 55;

/// '.' key.
pub const KEYCODE_PERIOD: i32 = 56;

/// Left Alt modifier key.
pub const KEYCODE_ALT_LEFT: i32 = 57;

/// Right Alt modifier key.
pub const KEYCODE_ALT_RIGHT: i32 = 58;

/// Left Shift modifier key.
pub const KEYCODE_SHIFT_LEFT: i32 = 59;

/// Right Shift modifier key.
pub const KEYCODE_SHIFT_RIGHT: i32 = 60;

/// Tab key.
pub const KEYCODE_TAB: i32 = 61;

/// Space key.
pub const KEYCODE_SPACE: i32 = 62;

/// Symbol modifier key.
///
/// Used to enter alternate symbols.
pub const KEYCODE_SYM: i32 = 63;

/// Explorer special function key.
///
/// Used to launch a browser application.
pub const KEYCODE_EXPLORER: i32 = 64;

/// Envelope special function key.
///
/// Used to launch a mail application.
pub const KEYCODE_ENVELOPE: i32 = 65;

/// Enter key.
pub const KEYCODE_ENTER: i32 = 66;

/// Backspace key.
///
/// Deletes characters before the insertion point, unlike [`KEYCODE_FORWARD_DEL`].
pub const KEYCODE_DEL: i32 = 67;

/// '\`' (backtick) key.
pub const KEYCODE_GRAVE: i32 = 68;

/// '-'.
pub const KEYCODE_MINUS: i32 = 69;

/// '=' key.
pub const KEYCODE_EQUALS: i32 = 70;

/// '[' key.
pub const KEYCODE_LEFT_BRACKET: i32 = 71;

/// ']' key.
pub const KEYCODE_RIGHT_BRACKET: i32 = 72;

/// '\' key.
pub const KEYCODE_BACKSLASH: i32 = 73;

/// ';' key.
pub const KEYCODE_SEMICOLON: i32 = 74;

/// ''' (apostrophe) key.
pub const KEYCODE_APOSTROPHE: i32 = 75;

/// '/' key.
pub const KEYCODE_SLASH: i32 = 76;

/// '@' key.
pub const KEYCODE_AT: i32 = 77;

/// Number modifier key.
///
/// Used to enter numeric symbols.
/// This key is not Num Lock; it is more like [`KEYCODE_ALT_LEFT`] and is
/// interpreted as an ALT key by [`MetaKeyKeylistener`][android.text.method.MetaKeyKeyListener].
/// [android.text.method.MetaKeyKeyListener]: <https://developer.android.com/reference/android/text/method/MetaKeyKeyListener>
pub const KEYCODE_NUM: i32 = 78;

/// Headset Hook key.
///
/// Used to hang up calls and stop media.
pub const KEYCODE_HEADSETHOOK: i32 = 79;

/// Camera Focus key.
///
/// Used to focus the camera.
pub const KEYCODE_FOCUS: i32 = 80;

/// '+' key.
pub const KEYCODE_PLUS: i32 = 81;

/// Menu key.
pub const KEYCODE_MENU: i32 = 82;

/// Notification key.
pub const KEYCODE_NOTIFICATION: i32 = 83;

/// Search key.
pub const KEYCODE_SEARCH: i32 = 84;

/// Play/Pause media key.
pub const KEYCODE_MEDIA_PLAY_PAUSE: i32 = 85;

/// Stop media key.
pub const KEYCODE_MEDIA_STOP: i32 = 86;

/// Play Next media key.
pub const KEYCODE_MEDIA_NEXT: i32 = 87;

/// Play Previous media key.
pub const KEYCODE_MEDIA_PREVIOUS: i32 = 88;

/// Rewind media key.
pub const KEYCODE_MEDIA_REWIND: i32 = 89;

/// Fast Forward media key.
pub const KEYCODE_MEDIA_FAST_FORWARD: i32 = 90;

/// Mute key.
///
/// Mute key for the microphone (unlike [`KEYCODE_VOLUME_MUTE`], which is the speaker mute
/// key).
pub const KEYCODE_MUTE: i32 = 91;

/// Page Up key.
pub const KEYCODE_PAGE_UP: i32 = 92;

/// Page Down key.
pub const KEYCODE_PAGE_DOWN: i32 = 93;

/// Picture Symbols modifier key.
///
/// Used to switch symbol sets (Emoji, Kao-moji).
pub const KEYCODE_PICTSYMBOLS: i32 = 94;

/// Switch Charset modifier key.
///
/// Used to switch character sets (Kanji, Katakana).
pub const KEYCODE_SWITCH_CHARSET: i32 = 95;

/// A Button key.
///
/// On a game controller, the A button should be either the button labeled A
/// or the first button on the bottom row of controller buttons.
pub const KEYCODE_BUTTON_A: i32 = 96;

/// B Button key.
///
/// On a game controller, the B button should be either the button labeled B
/// or the second button on the bottom row of controller buttons.
pub const KEYCODE_BUTTON_B: i32 = 97;

/// C Button key.
///
/// On a game controller, the C button should be either the button labeled C
/// or the third button on the bottom row of controller buttons.
pub const KEYCODE_BUTTON_C: i32 = 98;

/// X Button key.
///
/// On a game controller, the X button should be either the button labeled X
/// or the first button on the upper row of controller buttons.
pub const KEYCODE_BUTTON_X: i32 = 99;

/// Y Button key.
///
/// On a game controller, the Y button should be either the button labeled Y
/// or the second button on the upper row of controller buttons.
pub const KEYCODE_BUTTON_Y: i32 = 100;

/// Z Button key.
///
/// On a game controller, the Z button should be either the button labeled Z
/// or the third button on the upper row of controller buttons.
pub const KEYCODE_BUTTON_Z: i32 = 101;

/// L1 Button key.
///
/// On a game controller, the L1 button should be either the button labeled L1 (or L)
/// or the top left trigger button.
pub const KEYCODE_BUTTON_L1: i32 = 102;

/// R1 Button key.
///
/// On a game controller, the R1 button should be either the button labeled R1 (or R)
/// or the top right trigger button.
pub const KEYCODE_BUTTON_R1: i32 = 103;

/// L2 Button key.
///
/// On a game controller, the L2 button should be either the button labeled L2
/// or the bottom left trigger button.
pub const KEYCODE_BUTTON_L2: i32 = 104;

/// R2 Button key.
///
/// On a game controller, the R2 button should be either the button labeled R2
/// or the bottom right trigger button.
pub const KEYCODE_BUTTON_R2: i32 = 105;

/// Left Thumb Button key.
///
/// On a game controller, the left thumb button indicates that the left (or only)
/// joystick is pressed.
pub const KEYCODE_BUTTON_THUMBL: i32 = 106;

/// Right Thumb Button key.
///
/// On a game controller, the right thumb button indicates that the right
/// joystick is pressed.
pub const KEYCODE_BUTTON_THUMBR: i32 = 107;

/// Start Button key.
///
/// On a game controller, the button labeled Start.
pub const KEYCODE_BUTTON_START: i32 = 108;

/// Select Button key.
///
/// On a game controller, the button labeled Select.
pub const KEYCODE_BUTTON_SELECT: i32 = 109;

/// Mode Button key.
///
/// On a game controller, the button labeled Mode.
pub const KEYCODE_BUTTON_MODE: i32 = 110;

/// Escape key.
pub const KEYCODE_ESCAPE: i32 = 111;

/// Forward Delete key.
///
/// Deletes characters ahead of the insertion point, unlike [`KEYCODE_DEL`].
pub const KEYCODE_FORWARD_DEL: i32 = 112;

/// Left Control modifier key.
pub const KEYCODE_CTRL_LEFT: i32 = 113;

/// Right Control modifier key.
pub const KEYCODE_CTRL_RIGHT: i32 = 114;

/// Caps Lock key.
pub const KEYCODE_CAPS_LOCK: i32 = 115;

/// Scroll Lock key.
pub const KEYCODE_SCROLL_LOCK: i32 = 116;

/// Left Meta modifier key.
pub const KEYCODE_META_LEFT: i32 = 117;

/// Right Meta modifier key.
pub const KEYCODE_META_RIGHT: i32 = 118;

/// Function modifier key.
pub const KEYCODE_FUNCTION: i32 = 119;

/// System Request / Print Screen key.
///
/// This key is sent to the app first and only if the app doesn't handle it, the framework
/// handles it (to take a screenshot), unlike [`KEYCODE_SCREENSHOT`] which is
/// fully handled by the framework.
pub const KEYCODE_SYSRQ: i32 = 120;

/// Break / Pause key.
pub const KEYCODE_BREAK: i32 = 121;

/// Home Movement key.
///
/// Used for scrolling or moving the cursor around to the start of a line
/// or to the top of a list.
pub const KEYCODE_MOVE_HOME: i32 = 122;

/// End Movement key.
///
/// Used for scrolling or moving the cursor around to the end of a line
/// or to the bottom of a list.
pub const KEYCODE_MOVE_END: i32 = 123;

/// Insert key.
///
/// Toggles insert / overwrite edit mode.
pub const KEYCODE_INSERT: i32 = 124;

/// Forward key.
///
/// Navigates forward in the history stack.  Complement of [`KEYCODE_BACK`].
pub const KEYCODE_FORWARD: i32 = 125;

/// Play media key.
pub const KEYCODE_MEDIA_PLAY: i32 = 126;

/// Pause media key.
pub const KEYCODE_MEDIA_PAUSE: i32 = 127;

/// Close media key.
///
/// May be used to close a CD tray, for example.
pub const KEYCODE_MEDIA_CLOSE: i32 = 128;

/// Eject media key.
///
/// May be used to eject a CD tray, for example.
pub const KEYCODE_MEDIA_EJECT: i32 = 129;

/// Record media key.
pub const KEYCODE_MEDIA_RECORD: i32 = 130;

/// F1 key.
pub const KEYCODE_F1: i32 = 131;

/// F2 key.
pub const KEYCODE_F2: i32 = 132;

/// F3 key.
pub const KEYCODE_F3: i32 = 133;

/// F4 key.
pub const KEYCODE_F4: i32 = 134;

/// F5 key.
pub const KEYCODE_F5: i32 = 135;

/// F6 key.
pub const KEYCODE_F6: i32 = 136;

/// F7 key.
pub const KEYCODE_F7: i32 = 137;

/// F8 key.
pub const KEYCODE_F8: i32 = 138;

/// F9 key.
pub const KEYCODE_F9: i32 = 139;

/// F10 key.
pub const KEYCODE_F10: i32 = 140;

/// F11 key.
pub const KEYCODE_F11: i32 = 141;

/// F12 key.
pub const KEYCODE_F12: i32 = 142;

/// Num Lock key.
///
/// This is the Num Lock key; it is different from [`KEYCODE_NUM`].
/// This key alters the behavior of other keys on the numeric keypad.
pub const KEYCODE_NUM_LOCK: i32 = 143;

/// Numeric keypad '0' key.
pub const KEYCODE_NUMPAD_0: i32 = 144;

/// Numeric keypad '1' key.
pub const KEYCODE_NUMPAD_1: i32 = 145;

/// Numeric keypad '2' key.
pub const KEYCODE_NUMPAD_2: i32 = 146;

/// Numeric keypad '3' key.
pub const KEYCODE_NUMPAD_3: i32 = 147;

/// Numeric keypad '4' key.
pub const KEYCODE_NUMPAD_4: i32 = 148;

/// Numeric keypad '5' key.
pub const KEYCODE_NUMPAD_5: i32 = 149;

/// Numeric keypad '6' key.
pub const KEYCODE_NUMPAD_6: i32 = 150;

/// Numeric keypad '7' key.
pub const KEYCODE_NUMPAD_7: i32 = 151;

/// Numeric keypad '8' key.
pub const KEYCODE_NUMPAD_8: i32 = 152;

/// Numeric keypad '9' key.
pub const KEYCODE_NUMPAD_9: i32 = 153;

/// Numeric keypad '/' key (for division).
pub const KEYCODE_NUMPAD_DIVIDE: i32 = 154;

/// Numeric keypad '*' key (for multiplication).
pub const KEYCODE_NUMPAD_MULTIPLY: i32 = 155;

/// Numeric keypad '-' key (for subtraction).
pub const KEYCODE_NUMPAD_SUBTRACT: i32 = 156;

/// Numeric keypad '+' key (for addition).
pub const KEYCODE_NUMPAD_ADD: i32 = 157;

/// Numeric keypad '.' key (for decimals or digit grouping).
pub const KEYCODE_NUMPAD_DOT: i32 = 158;

/// Numeric keypad ',' key (for decimals or digit grouping).
pub const KEYCODE_NUMPAD_COMMA: i32 = 159;

/// Numeric keypad Enter key.
pub const KEYCODE_NUMPAD_ENTER: i32 = 160;

/// Numeric keypad '=' key.
pub const KEYCODE_NUMPAD_EQUALS: i32 = 161;

/// Numeric keypad '(' key.
pub const KEYCODE_NUMPAD_LEFT_PAREN: i32 = 162;

/// Numeric keypad ')' key.
pub const KEYCODE_NUMPAD_RIGHT_PAREN: i32 = 163;

/// Volume Mute key.
///
/// Mute key for speaker (unlike [`KEYCODE_MUTE`], which is the mute key for the
/// microphone). This key should normally be implemented as a toggle such that the first press
/// mutes the speaker and the second press restores the original volume.
pub const KEYCODE_VOLUME_MUTE: i32 = 164;

/// Info key.
///
/// Common on TV remotes to show additional information related to what is
/// currently being viewed.
pub const KEYCODE_INFO: i32 = 165;

/// Channel up key.
///
/// On TV remotes, increments the television channel.
pub const KEYCODE_CHANNEL_UP: i32 = 166;

/// Channel down key.
///
/// On TV remotes, decrements the television channel.
pub const KEYCODE_CHANNEL_DOWN: i32 = 167;

/// Zoom in key.
pub const KEYCODE_ZOOM_IN: i32 = 168;

/// Zoom out key.
pub const KEYCODE_ZOOM_OUT: i32 = 169;

/// TV key.
/// On TV remotes, switches to viewing live TV.
pub const KEYCODE_TV: i32 = 170;

/// Window key.
///
/// On TV remotes, toggles picture-in-picture mode or other windowing functions.
/// On Android Wear devices, triggers a display offset.
pub const KEYCODE_WINDOW: i32 = 171;

/// Guide key.
///
/// On TV remotes, shows a programming guide.
pub const KEYCODE_GUIDE: i32 = 172;

/// DVR key.
///
/// On some TV remotes, switches to a DVR mode for recorded shows.
pub const KEYCODE_DVR: i32 = 173;

/// Bookmark key.
///
/// On some TV remotes, bookmarks content or web pages.
pub const KEYCODE_BOOKMARK: i32 = 174;

/// Toggle captions key.
///
/// Switches the mode for closed-captioning text, for example during television shows.
pub const KEYCODE_CAPTIONS: i32 = 175;

/// Settings key.
///
/// Starts the system settings activity.
pub const KEYCODE_SETTINGS: i32 = 176;

/// TV power key.
///
/// On HDMI TV panel devices and Android TV devices that don't support HDMI, toggles the power
/// state of the device.
/// On HDMI source devices, toggles the power state of the HDMI-connected TV via HDMI-CEC and
/// makes the source device follow this power state.
pub const KEYCODE_TV_POWER: i32 = 177;

/// TV input key.
///
/// On TV remotes, switches the input on a television screen.
pub const KEYCODE_TV_INPUT: i32 = 178;

/// Set-top-box power key.
///
/// On TV remotes, toggles the power on an external Set-top-box.
pub const KEYCODE_STB_POWER: i32 = 179;

/// Set-top-box input key.
///
/// On TV remotes, switches the input mode on an external Set-top-box.
pub const KEYCODE_STB_INPUT: i32 = 180;

/// A/V Receiver power key.
///
/// On TV remotes, toggles the power on an external A/V Receiver.
pub const KEYCODE_AVR_POWER: i32 = 181;

/// A/V Receiver input key.
///
/// On TV remotes, switches the input mode on an external A/V Receiver.
pub const KEYCODE_AVR_INPUT: i32 = 182;

/// Red "programmable" key.
///
/// On TV remotes, acts as a contextual/programmable key.
pub const KEYCODE_PROG_RED: i32 = 183;

/// Green "programmable" key.
///
/// On TV remotes, actsas a contextual/programmable key.
pub const KEYCODE_PROG_GREEN: i32 = 184;

/// Yellow "programmable" key.
///
/// On TV remotes, acts as a contextual/programmable key.
pub const KEYCODE_PROG_YELLOW: i32 = 185;

/// Blue "programmable" key.
///
/// On TV remotes, acts as a contextual/programmable key.
pub const KEYCODE_PROG_BLUE: i32 = 186;

/// App switch key.
///
/// Should bring up the application switcher dialog.
pub const KEYCODE_APP_SWITCH: i32 = 187;

/// Generic Game Pad Button #1.
pub const KEYCODE_BUTTON_1: i32 = 188;

/// Generic Game Pad Button #2.
pub const KEYCODE_BUTTON_2: i32 = 189;

/// Generic Game Pad Button #3.
pub const KEYCODE_BUTTON_3: i32 = 190;

/// Generic Game Pad Button #4.
pub const KEYCODE_BUTTON_4: i32 = 191;

/// Generic Game Pad Button #5.
pub const KEYCODE_BUTTON_5: i32 = 192;

/// Generic Game Pad Button #6.
pub const KEYCODE_BUTTON_6: i32 = 193;

/// Generic Game Pad Button #7.
pub const KEYCODE_BUTTON_7: i32 = 194;

/// Generic Game Pad Button #8.
pub const KEYCODE_BUTTON_8: i32 = 195;

/// Generic Game Pad Button #9.
pub const KEYCODE_BUTTON_9: i32 = 196;

/// Generic Game Pad Button #10.
pub const KEYCODE_BUTTON_10: i32 = 197;

/// Generic Game Pad Button #11.
pub const KEYCODE_BUTTON_11: i32 = 198;

/// Generic Game Pad Button #12.
pub const KEYCODE_BUTTON_12: i32 = 199;

/// Generic Game Pad Button #13.
pub const KEYCODE_BUTTON_13: i32 = 200;

/// Generic Game Pad Button #14.
pub const KEYCODE_BUTTON_14: i32 = 201;

/// Generic Game Pad Button #15.
pub const KEYCODE_BUTTON_15: i32 = 202;

/// Generic Game Pad Button #16.
pub const KEYCODE_BUTTON_16: i32 = 203;

/// Language Switch key.
///
/// Toggles the current input language such as switching between English and Japanese on
/// a QWERTY keyboard.  On some devices, the same function may be performed by
/// pressing Shift+Spacebar.
pub const KEYCODE_LANGUAGE_SWITCH: i32 = 204;

/// Manner Mode key.
///
/// Toggles silent or vibrate mode on and off to make the device behave more politely
/// in certain settings such as on a crowded train.  On some devices, the key may only
/// operate when long-pressed.
pub const KEYCODE_MANNER_MODE: i32 = 205;

/// 3D Mode key.
///
/// Toggles the display between 2D and 3D mode.
pub const KEYCODE_3D_MODE: i32 = 206;

/// Contacts special function key.
///
/// Used to launch an address book application.
pub const KEYCODE_CONTACTS: i32 = 207;

/// Calendar special function key.
///
/// Used to launch a calendar application.
pub const KEYCODE_CALENDAR: i32 = 208;

/// Music special function key.
///
/// Used to launch a music player application.
pub const KEYCODE_MUSIC: i32 = 209;

/// Calculator special function key.
///
/// Used to launch a calculator application.
pub const KEYCODE_CALCULATOR: i32 = 210;

/// Japanese full-width / half-width key.
pub const KEYCODE_ZENKAKU_HANKAKU: i32 = 211;

/// Japanese alphanumeric key.
pub const KEYCODE_EISU: i32 = 212;

/// Japanese non-conversion key.
pub const KEYCODE_MUHENKAN: i32 = 213;

/// Japanese conversion key.
pub const KEYCODE_HENKAN: i32 = 214;

/// Japanese katakana / hiragana key.
pub const KEYCODE_KATAKANA_HIRAGANA: i32 = 215;

/// Japanese Yen key.
pub const KEYCODE_YEN: i32 = 216;

/// Japanese Ro key.
pub const KEYCODE_RO: i32 = 217;

/// Japanese kana key.
pub const KEYCODE_KANA: i32 = 218;

/// Assist key.
///
/// Launches the global assist activity.  Not delivered to applications.
pub const KEYCODE_ASSIST: i32 = 219;

/// Brightness Down key.
///
/// Adjusts the screen brightness down.
pub const KEYCODE_BRIGHTNESS_DOWN: i32 = 220;

/// Brightness Up key.
///
/// Adjusts the screen brightness up.
pub const KEYCODE_BRIGHTNESS_UP: i32 = 221;

/// Audio Track key.
///
/// Switches the audio tracks.
pub const KEYCODE_MEDIA_AUDIO_TRACK: i32 = 222;

/// Sleep key.
///
/// Puts the device to sleep.  Behaves somewhat like [`KEYCODE_POWER`] but it
/// has no effect if the device is already asleep.
pub const KEYCODE_SLEEP: i32 = 223;

/// Wakeup key.
///
/// Wakes up the device.  Behaves somewhat like [`KEYCODE_POWER`] but it
/// has no effect if the device is already awake.
pub const KEYCODE_WAKEUP: i32 = 224;

/// Pairing key.
///
/// Initiates peripheral pairing mode. Useful for pairing remote control
/// devices or game controllers, especially if no other input mode is
/// available.
pub const KEYCODE_PAIRING: i32 = 225;

/// Media Top Menu key.
///
/// Goes to the top of media menu.
pub const KEYCODE_MEDIA_TOP_MENU: i32 = 226;

/// '11' key.
pub const KEYCODE_11: i32 = 227;

/// '12' key.
pub const KEYCODE_12: i32 = 228;

/// Last Channel key.
///
/// Goes to the last viewed channel.
pub const KEYCODE_LAST_CHANNEL: i32 = 229;

/// TV data service key.
///
/// Displays data services like weather, sports.
pub const KEYCODE_TV_DATA_SERVICE: i32 = 230;

/// Voice Assist key.
///
/// Launches the global voice assist activity. Not delivered to applications.
pub const KEYCODE_VOICE_ASSIST: i32 = 231;

/// Radio key.
///
/// Toggles TV service / Radio service.
pub const KEYCODE_TV_RADIO_SERVICE: i32 = 232;

/// Teletext key.
///
/// Displays Teletext service.
pub const KEYCODE_TV_TELETEXT: i32 = 233;

/// Number entry key.
///
/// Initiates to enter multi-digit channel nubmber when each digit key is assigned
/// for selecting separate channel. Corresponds to Number Entry Mode (0x1D) of CEC
/// User Control Code.
pub const KEYCODE_TV_NUMBER_ENTRY: i32 = 234;

/// Analog Terrestrial key.
///
/// Switches to analog terrestrial broadcast service.
pub const KEYCODE_TV_TERRESTRIAL_ANALOG: i32 = 235;

/// Digital Terrestrial key.
///
/// Switches to digital terrestrial broadcast service.
pub const KEYCODE_TV_TERRESTRIAL_DIGITAL: i32 = 236;

/// Satellite key.
///
/// Switches to digital satellite broadcast service.
pub const KEYCODE_TV_SATELLITE: i32 = 237;

/// BS key.
///
/// Switches to BS digital satellite broadcasting service available in Japan.
pub const KEYCODE_TV_SATELLITE_BS: i32 = 238;

/// CS key.
///
/// Switches to CS digital satellite broadcasting service available in Japan.
pub const KEYCODE_TV_SATELLITE_CS: i32 = 239;

/// BS/CS key.
///
/// Toggles between BS and CS digital satellite services.
pub const KEYCODE_TV_SATELLITE_SERVICE: i32 = 240;

/// Toggle Network key.
///
/// Toggles selecting broadcast services.
pub const KEYCODE_TV_NETWORK: i32 = 241;

/// Antenna/Cable key.
///
/// Toggles broadcast input source between antenna and cable.
pub const KEYCODE_TV_ANTENNA_CABLE: i32 = 242;

/// HDMI #1 key.
///
/// Switches to HDMI input #1.
pub const KEYCODE_TV_INPUT_HDMI_1: i32 = 243;

/// HDMI #2 key.
///
/// Switches to HDMI input #2.
pub const KEYCODE_TV_INPUT_HDMI_2: i32 = 244;

/// HDMI #3 key.
///
/// Switches to HDMI input #3.
pub const KEYCODE_TV_INPUT_HDMI_3: i32 = 245;

/// HDMI #4 key.
///
/// Switches to HDMI input #4.
pub const KEYCODE_TV_INPUT_HDMI_4: i32 = 246;

/// Composite #1 key.
///
/// Switches to composite video input #1.
pub const KEYCODE_TV_INPUT_COMPOSITE_1: i32 = 247;

/// Composite #2 key.
///
/// Switches to composite video input #2.
pub const KEYCODE_TV_INPUT_COMPOSITE_2: i32 = 248;

/// Component #1 key.
///
/// Switches to component video input #1.
pub const KEYCODE_TV_INPUT_COMPONENT_1: i32 = 249;

/// Component #2 key.
///
/// Switches to component video input #2.
pub const KEYCODE_TV_INPUT_COMPONENT_2: i32 = 250;

/// VGA #1 key.
///
/// Switches to VGA (analog RGB) input #1.
pub const KEYCODE_TV_INPUT_VGA_1: i32 = 251;

/// Audio description key.
///
/// Toggles audio description off / on.
pub const KEYCODE_TV_AUDIO_DESCRIPTION: i32 = 252;

/// Audio description mixing volume up key.
///
/// Louden audio description volume as compared with normal audio volume.
pub const KEYCODE_TV_AUDIO_DESCRIPTION_MIX_UP: i32 = 253;

/// Audio description mixing volume down key.
///
/// Lessen audio description volume as compared with normal audio volume.
pub const KEYCODE_TV_AUDIO_DESCRIPTION_MIX_DOWN: i32 = 254;

/// Zoom mode key.
///
/// Changes Zoom mode (Normal, Full, Zoom, Wide-zoom, etc.)
pub const KEYCODE_TV_ZOOM_MODE: i32 = 255;

/// Contents menu key.
///
/// Goes to the title list. Corresponds to Contents Menu (0x0B) of CEC User Control
/// Code
pub const KEYCODE_TV_CONTENTS_MENU: i32 = 256;

/// Media context menu key.
///
/// Goes to the context menu of media contents. Corresponds to Media Context-sensitive
/// Menu (0x11) of CEC User Control Code.
pub const KEYCODE_TV_MEDIA_CONTEXT_MENU: i32 = 257;

/// Timer programming key.
///
/// Goes to the timer recording menu. Corresponds to Timer Programming (0x54) of
/// CEC User Control Code.
pub const KEYCODE_TV_TIMER_PROGRAMMING: i32 = 258;

/// Help key.
pub const KEYCODE_HELP: i32 = 259;

/// Navigate to previous key.
///
/// Goes backward by one item in an ordered collection of items.
pub const KEYCODE_NAVIGATE_PREVIOUS: i32 = 260;

/// Navigate to next key.
///
/// Advances to the next item in an ordered collection of items.
pub const KEYCODE_NAVIGATE_NEXT: i32 = 261;

/// Navigate in key.
///
/// Activates the item that currently has focus or expands to the next level of a navigation
/// hierarchy.
pub const KEYCODE_NAVIGATE_IN: i32 = 262;

/// Navigate out key.
///
/// Backs out one level of a navigation hierarchy or collapses the item that currently has
/// focus.
pub const KEYCODE_NAVIGATE_OUT: i32 = 263;

/// Primary stem key for Wear.
///
/// Main power/reset button on watch.
pub const KEYCODE_STEM_PRIMARY: i32 = 264;

/// Generic stem key 1 for Wear.
pub const KEYCODE_STEM_1: i32 = 265;

/// Generic stem key 2 for Wear.
pub const KEYCODE_STEM_2: i32 = 266;

/// Generic stem key 3 for Wear.
pub const KEYCODE_STEM_3: i32 = 267;

/// Directional Pad Up-Left.
pub const KEYCODE_DPAD_UP_LEFT: i32 = 268;

/// Directional Pad Down-Left.
pub const KEYCODE_DPAD_DOWN_LEFT: i32 = 269;

/// Directional Pad Up-Right.
pub const KEYCODE_DPAD_UP_RIGHT: i32 = 270;

/// Directional Pad Down-Right.
pub const KEYCODE_DPAD_DOWN_RIGHT: i32 = 271;

/// Skip forward media key.
pub const KEYCODE_MEDIA_SKIP_FORWARD: i32 = 272;

/// Skip backward media key.
pub const KEYCODE_MEDIA_SKIP_BACKWARD: i32 = 273;

/// Step forward media key.
///
/// Steps media forward, one frame at a time.
pub const KEYCODE_MEDIA_STEP_FORWARD: i32 = 274;

/// Step backward media key.
///
/// Steps media backward, one frame at a time.
pub const KEYCODE_MEDIA_STEP_BACKWARD: i32 = 275;

/// put device to sleep unless a wakelock is held.
pub const KEYCODE_SOFT_SLEEP: i32 = 276;

/// Cut key.
pub const KEYCODE_CUT: i32 = 277;

/// Copy key.
pub const KEYCODE_COPY: i32 = 278;

/// Paste key.
pub const KEYCODE_PASTE: i32 = 279;

/// Consumed by the system for navigation up.
pub const KEYCODE_SYSTEM_NAVIGATION_UP: i32 = 280;

/// Consumed by the system for navigation down.
pub const KEYCODE_SYSTEM_NAVIGATION_DOWN: i32 = 281;

/// Consumed by the system for navigation left.
pub const KEYCODE_SYSTEM_NAVIGATION_LEFT: i32 = 282;

/// Consumed by the system for navigation right.
pub const KEYCODE_SYSTEM_NAVIGATION_RIGHT: i32 = 283;

/// Show all apps.
pub const KEYCODE_ALL_APPS: i32 = 284;

/// Refresh key.
pub const KEYCODE_REFRESH: i32 = 285;

/// Thumbs up key.
///
/// Apps can use this to let user upvote content.
pub const KEYCODE_THUMBS_UP: i32 = 286;

/// Thumbs down key.
///
/// Apps can use this to let user downvote content.
pub const KEYCODE_THUMBS_DOWN: i32 = 287;

/// Used to switch current [`Account`][android.accounts.Account] that is consuming content.
///
/// May be consumed by system to set account globally.
///
/// [android.accounts.Account]: <https://developer.android.com/reference/android/accounts/Account>
pub const KEYCODE_PROFILE_SWITCH: i32 = 288;

/// Video Application key #1.
pub const KEYCODE_VIDEO_APP_1: i32 = 289;

/// Video Application key #2.
pub const KEYCODE_VIDEO_APP_2: i32 = 290;

/// Video Application key #3.
pub const KEYCODE_VIDEO_APP_3: i32 = 291;

/// Video Application key #4.
pub const KEYCODE_VIDEO_APP_4: i32 = 292;

/// Video Application key #5.
pub const KEYCODE_VIDEO_APP_5: i32 = 293;

/// Video Application key #6.
pub const KEYCODE_VIDEO_APP_6: i32 = 294;

/// Video Application key #7.
pub const KEYCODE_VIDEO_APP_7: i32 = 295;

/// Video Application key #8.
pub const KEYCODE_VIDEO_APP_8: i32 = 296;

/// Featured Application key #1.
pub const KEYCODE_FEATURED_APP_1: i32 = 297;

/// Featured Application key #2.
pub const KEYCODE_FEATURED_APP_2: i32 = 298;

/// Featured Application key #3.
pub const KEYCODE_FEATURED_APP_3: i32 = 299;

/// Featured Application key #4.
pub const KEYCODE_FEATURED_APP_4: i32 = 300;

/// Demo Application key #1.
pub const KEYCODE_DEMO_APP_1: i32 = 301;

/// Demo Application key #2.
pub const KEYCODE_DEMO_APP_2: i32 = 302;

/// Demo Application key #3.
pub const KEYCODE_DEMO_APP_3: i32 = 303;

/// Demo Application key #4.
pub const KEYCODE_DEMO_APP_4: i32 = 304;

/// Keyboard backlight down
pub const KEYCODE_KEYBOARD_BACKLIGHT_DOWN: i32 = 305;

/// Keyboard backlight up
pub const KEYCODE_KEYBOARD_BACKLIGHT_UP: i32 = 306;

/// Keyboard backlight toggle
pub const KEYCODE_KEYBOARD_BACKLIGHT_TOGGLE: i32 = 307;

/// The primary button on the barrel of a stylus.
///
/// This is usually the button closest to the tip of the stylus.
pub const KEYCODE_STYLUS_BUTTON_PRIMARY: i32 = 308;

/// The secondary button on the barrel of a stylus.
///
/// This is usually the second button from the tip of the stylus.
pub const KEYCODE_STYLUS_BUTTON_SECONDARY: i32 = 309;

/// The tertiary button on the barrel of a stylus.
///
/// This is usually the third button from the tip of the stylus.
pub const KEYCODE_STYLUS_BUTTON_TERTIARY: i32 = 310;

/// A button on the tail end of a stylus.
///
/// The use of this button does not usually correspond to the function of an eraser.
pub const KEYCODE_STYLUS_BUTTON_TAIL: i32 = 311;

/// To open recent apps view (a.k.a. Overview).
///
/// This key is handled by the framework and is never delivered to applications.
pub const KEYCODE_RECENT_APPS: i32 = 312;

/// A button whose usage can be customized by the user through the system.
///
/// User customizable key #1.
pub const KEYCODE_MACRO_1: i32 = 313;

/// A button whose usage can be customized by the user through the system.
///
/// User customizable key #2.
pub const KEYCODE_MACRO_2: i32 = 314;

/// A button whose usage can be customized by the user through the system.
///
/// User customizable key #3.
pub const KEYCODE_MACRO_3: i32 = 315;

/// A button whose usage can be customized by the user through the system.
///
/// User customizable key #4.
pub const KEYCODE_MACRO_4: i32 = 316;

/// To open emoji picker.
pub const KEYCODE_EMOJI_PICKER: i32 = 317;

/// To take a screenshot.
///
/// This key is fully handled by the framework and will not be sent to the foreground app,
/// unlike [`KEYCODE_SYSRQ`] which is sent to the app first and only if the app  doesn't
/// handle it, the framework handles it (to take a screenshot).
pub const KEYCODE_SCREENSHOT: i32 = 318;

/// To start dictate to an input field.
pub const KEYCODE_DICTATE: i32 = 319;

/// AC New.
///
/// e.g. To create a new instance of a window, open a new tab, etc.
pub const KEYCODE_NEW: i32 = 320;

/// AC Close.
///
/// e.g. To close current instance of the application window, close the current tab, etc.
pub const KEYCODE_CLOSE: i32 = 321;

/// To toggle ‘Do Not Disturb’ mode.
pub const KEYCODE_DO_NOT_DISTURB: i32 = 322;

/// To Print.
pub const KEYCODE_PRINT: i32 = 323;

/// To Lock the screen.
pub const KEYCODE_LOCK: i32 = 324;

/// To toggle fullscreen mode (on the current application).
pub const KEYCODE_FULLSCREEN: i32 = 325;

/// F13 key.
pub const KEYCODE_F13: i32 = 326;

/// F14 key.
pub const KEYCODE_F14: i32 = 327;

/// F15 key.
pub const KEYCODE_F15: i32 = 328;

/// F16 key.
pub const KEYCODE_F16: i32 = 329;

/// F17 key.
pub const KEYCODE_F17: i32 = 330;

/// F18 key.
pub const KEYCODE_F18: i32 = 331;

/// F19 key.
pub const KEYCODE_F19: i32 = 332;

/// F20 key.
pub const KEYCODE_F20: i32 = 333;

/// F21 key.
pub const KEYCODE_F21: i32 = 334;

/// F22 key.
pub const KEYCODE_F22: i32 = 335;

/// F23 key.
pub const KEYCODE_F23: i32 = 336;

/// F24 key.
pub const KEYCODE_F24: i32 = 337;

use super::{Code, NamedKey};

/// Translates an Android keycode to its closest equivalent `Code`.
pub fn keycode_to_code(keycode: i32) -> Code {
    match keycode {
        KEYCODE_A => Code::KeyA,
        KEYCODE_B => Code::KeyB,
        KEYCODE_C => Code::KeyC,
        KEYCODE_D => Code::KeyD,
        KEYCODE_E => Code::KeyE,
        KEYCODE_F => Code::KeyF,
        KEYCODE_G => Code::KeyG,
        KEYCODE_H => Code::KeyH,
        KEYCODE_I => Code::KeyI,
        KEYCODE_J => Code::KeyJ,
        KEYCODE_K => Code::KeyK,
        KEYCODE_L => Code::KeyL,
        KEYCODE_M => Code::KeyM,
        KEYCODE_N => Code::KeyN,
        KEYCODE_O => Code::KeyO,
        KEYCODE_P => Code::KeyP,
        KEYCODE_Q => Code::KeyQ,
        KEYCODE_R => Code::KeyR,
        KEYCODE_S => Code::KeyS,
        KEYCODE_T => Code::KeyT,
        KEYCODE_U => Code::KeyU,
        KEYCODE_V => Code::KeyV,
        KEYCODE_W => Code::KeyW,
        KEYCODE_X => Code::KeyX,
        KEYCODE_Y => Code::KeyY,
        KEYCODE_Z => Code::KeyZ,
        KEYCODE_0 => Code::Digit0,
        KEYCODE_1 => Code::Digit1,
        KEYCODE_2 => Code::Digit2,
        KEYCODE_3 => Code::Digit3,
        KEYCODE_4 => Code::Digit4,
        KEYCODE_5 => Code::Digit5,
        KEYCODE_6 => Code::Digit6,
        KEYCODE_7 => Code::Digit7,
        KEYCODE_8 => Code::Digit8,
        KEYCODE_9 => Code::Digit9,

        KEYCODE_COMMA => Code::Comma,
        KEYCODE_PERIOD => Code::Period,
        KEYCODE_MINUS => Code::Minus,
        KEYCODE_EQUALS => Code::Equal,
        KEYCODE_LEFT_BRACKET => Code::BracketLeft,
        KEYCODE_RIGHT_BRACKET => Code::BracketRight,
        KEYCODE_BACKSLASH => Code::Backslash,
        KEYCODE_SEMICOLON => Code::Semicolon,
        KEYCODE_APOSTROPHE => Code::Quote,
        KEYCODE_SLASH => Code::Slash,
        KEYCODE_GRAVE => Code::Backquote,
        KEYCODE_PLUS => Code::NumpadAdd,

        KEYCODE_SHIFT_LEFT => Code::ShiftLeft,
        KEYCODE_SHIFT_RIGHT => Code::ShiftRight,
        KEYCODE_CTRL_LEFT => Code::ControlLeft,
        KEYCODE_CTRL_RIGHT => Code::ControlRight,
        KEYCODE_ALT_LEFT => Code::AltLeft,
        KEYCODE_ALT_RIGHT => Code::AltRight,
        KEYCODE_META_LEFT => Code::MetaLeft,
        KEYCODE_META_RIGHT => Code::MetaRight,
        KEYCODE_CAPS_LOCK => Code::CapsLock,
        KEYCODE_NUM_LOCK => Code::NumLock,
        KEYCODE_SCROLL_LOCK => Code::ScrollLock,
        KEYCODE_FUNCTION => Code::Fn,

        KEYCODE_DPAD_UP => Code::ArrowUp,
        KEYCODE_DPAD_DOWN => Code::ArrowDown,
        KEYCODE_DPAD_LEFT => Code::ArrowLeft,
        KEYCODE_DPAD_RIGHT => Code::ArrowRight,
        // Not exactly equivalent, but popular translation.
        KEYCODE_DPAD_CENTER => Code::Enter,
        KEYCODE_PAGE_UP => Code::PageUp,
        KEYCODE_PAGE_DOWN => Code::PageDown,
        KEYCODE_MOVE_HOME => Code::Home,
        KEYCODE_MOVE_END => Code::End,
        KEYCODE_INSERT => Code::Insert,
        KEYCODE_ESCAPE => Code::Escape,

        KEYCODE_DEL => Code::Backspace,
        KEYCODE_FORWARD_DEL => Code::Delete,
        KEYCODE_CUT => Code::Cut,
        KEYCODE_COPY => Code::Copy,
        KEYCODE_PASTE => Code::Paste,

        KEYCODE_ENTER => Code::Enter,
        KEYCODE_SPACE => Code::Space,
        KEYCODE_TAB => Code::Tab,
        KEYCODE_MENU => Code::ContextMenu,

        KEYCODE_F1 => Code::F1,
        KEYCODE_F2 => Code::F2,
        KEYCODE_F3 => Code::F3,
        KEYCODE_F4 => Code::F4,
        KEYCODE_F5 => Code::F5,
        KEYCODE_F6 => Code::F6,
        KEYCODE_F7 => Code::F7,
        KEYCODE_F8 => Code::F8,
        KEYCODE_F9 => Code::F9,
        KEYCODE_F10 => Code::F10,
        KEYCODE_F11 => Code::F11,
        KEYCODE_F12 => Code::F12,
        KEYCODE_F13 => Code::F13,
        KEYCODE_F14 => Code::F14,
        KEYCODE_F15 => Code::F15,
        KEYCODE_F16 => Code::F16,
        KEYCODE_F17 => Code::F17,
        KEYCODE_F18 => Code::F18,
        KEYCODE_F19 => Code::F19,
        KEYCODE_F20 => Code::F20,
        KEYCODE_F21 => Code::F21,
        KEYCODE_F22 => Code::F22,
        KEYCODE_F23 => Code::F23,
        KEYCODE_F24 => Code::F24,

        KEYCODE_NUMPAD_0 => Code::Numpad0,
        KEYCODE_NUMPAD_1 => Code::Numpad1,
        KEYCODE_NUMPAD_2 => Code::Numpad2,
        KEYCODE_NUMPAD_3 => Code::Numpad3,
        KEYCODE_NUMPAD_4 => Code::Numpad4,
        KEYCODE_NUMPAD_5 => Code::Numpad5,
        KEYCODE_NUMPAD_6 => Code::Numpad6,
        KEYCODE_NUMPAD_7 => Code::Numpad7,
        KEYCODE_NUMPAD_8 => Code::Numpad8,
        KEYCODE_NUMPAD_9 => Code::Numpad9,
        KEYCODE_NUMPAD_ADD => Code::NumpadAdd,
        KEYCODE_NUMPAD_SUBTRACT => Code::NumpadSubtract,
        KEYCODE_NUMPAD_MULTIPLY => Code::NumpadMultiply,
        KEYCODE_NUMPAD_DIVIDE => Code::NumpadDivide,
        KEYCODE_NUMPAD_ENTER => Code::NumpadEnter,
        KEYCODE_NUMPAD_DOT => Code::NumpadDecimal,
        KEYCODE_NUMPAD_COMMA => Code::NumpadComma,
        KEYCODE_NUMPAD_EQUALS => Code::NumpadEqual,
        KEYCODE_NUMPAD_LEFT_PAREN => Code::NumpadParenLeft,
        KEYCODE_NUMPAD_RIGHT_PAREN => Code::NumpadParenRight,
        KEYCODE_CLEAR => Code::NumpadClear,

        KEYCODE_MEDIA_PLAY_PAUSE => Code::MediaPlayPause,
        KEYCODE_MEDIA_STOP => Code::MediaStop,
        KEYCODE_MEDIA_NEXT => Code::MediaTrackNext,
        KEYCODE_MEDIA_PREVIOUS => Code::MediaTrackPrevious,
        KEYCODE_MEDIA_PLAY => Code::MediaPlay,
        KEYCODE_MEDIA_PAUSE => Code::MediaPause,
        KEYCODE_MEDIA_FAST_FORWARD => Code::MediaFastForward,
        KEYCODE_MEDIA_REWIND => Code::MediaRewind,
        KEYCODE_MEDIA_RECORD => Code::MediaRecord,
        KEYCODE_VOLUME_UP => Code::AudioVolumeUp,
        KEYCODE_VOLUME_DOWN => Code::AudioVolumeDown,
        KEYCODE_VOLUME_MUTE => Code::AudioVolumeMute,
        KEYCODE_MUTE => Code::MicrophoneMuteToggle,

        KEYCODE_POWER => Code::Power,
        KEYCODE_SLEEP => Code::Sleep,
        KEYCODE_WAKEUP => Code::WakeUp,
        KEYCODE_BRIGHTNESS_DOWN => Code::BrightnessDown,
        KEYCODE_BRIGHTNESS_UP => Code::BrightnessUp,
        KEYCODE_HELP => Code::Help,
        KEYCODE_SYSRQ => Code::PrintScreen,
        KEYCODE_BREAK => Code::Pause,

        KEYCODE_HENKAN => Code::Convert,
        KEYCODE_MUHENKAN => Code::NonConvert,
        KEYCODE_KATAKANA_HIRAGANA => Code::KanaMode,
        KEYCODE_EISU => Code::Lang2,
        KEYCODE_KANA => Code::Lang1,
        KEYCODE_ZENKAKU_HANKAKU => Code::Lang5,
        KEYCODE_YEN => Code::IntlYen,
        KEYCODE_RO => Code::IntlRo,

        KEYCODE_CALCULATOR => Code::LaunchApp2,
        KEYCODE_ENVELOPE => Code::LaunchMail,
        KEYCODE_EXPLORER => Code::LaunchApp1,

        _ => Code::Unidentified,
    }
}

/// Translates an Android keycode to its closest equivalent `NamedKey`.
///
/// Some keys which are `NamedKey::Unidentified` here will nonetheless
/// have [`Character`][super::Key::Character] translations.
pub fn keycode_to_named_key(keycode: i32) -> NamedKey {
    match keycode {
        KEYCODE_SHIFT_LEFT | KEYCODE_SHIFT_RIGHT => NamedKey::Shift,
        KEYCODE_CTRL_LEFT | KEYCODE_CTRL_RIGHT => NamedKey::Control,
        // See [`KEYCODE_NUM`] for why it is included here.
        KEYCODE_ALT_LEFT | KEYCODE_ALT_RIGHT | KEYCODE_NUM => NamedKey::Alt,
        KEYCODE_META_LEFT | KEYCODE_META_RIGHT => NamedKey::Meta,
        KEYCODE_CAPS_LOCK => NamedKey::CapsLock,
        KEYCODE_NUM_LOCK => NamedKey::NumLock,
        KEYCODE_SCROLL_LOCK => NamedKey::ScrollLock,
        KEYCODE_FUNCTION => NamedKey::Fn,

        KEYCODE_DPAD_UP => NamedKey::ArrowUp,
        KEYCODE_DPAD_DOWN => NamedKey::ArrowDown,
        KEYCODE_DPAD_LEFT => NamedKey::ArrowLeft,
        KEYCODE_DPAD_RIGHT => NamedKey::ArrowRight,
        KEYCODE_PAGE_UP => NamedKey::PageUp,
        KEYCODE_PAGE_DOWN => NamedKey::PageDown,
        KEYCODE_MOVE_HOME => NamedKey::Home,
        KEYCODE_MOVE_END => NamedKey::End,
        KEYCODE_HOME => NamedKey::GoHome,
        KEYCODE_BACK => NamedKey::GoBack,

        KEYCODE_DEL => NamedKey::Backspace,
        KEYCODE_FORWARD_DEL => NamedKey::Delete,
        KEYCODE_INSERT => NamedKey::Insert,

        KEYCODE_ENTER => NamedKey::Enter,
        KEYCODE_TAB => NamedKey::Tab,
        KEYCODE_ESCAPE => NamedKey::Escape,

        KEYCODE_F1 => NamedKey::F1,
        KEYCODE_F2 => NamedKey::F2,
        KEYCODE_F3 => NamedKey::F3,
        KEYCODE_F4 => NamedKey::F4,
        KEYCODE_F5 => NamedKey::F5,
        KEYCODE_F6 => NamedKey::F6,
        KEYCODE_F7 => NamedKey::F7,
        KEYCODE_F8 => NamedKey::F8,
        KEYCODE_F9 => NamedKey::F9,
        KEYCODE_F10 => NamedKey::F10,
        KEYCODE_F11 => NamedKey::F11,
        KEYCODE_F12 => NamedKey::F12,
        KEYCODE_F13 => NamedKey::F13,
        KEYCODE_F14 => NamedKey::F14,
        KEYCODE_F15 => NamedKey::F15,
        KEYCODE_F16 => NamedKey::F16,
        KEYCODE_F17 => NamedKey::F17,
        KEYCODE_F18 => NamedKey::F18,
        KEYCODE_F19 => NamedKey::F19,
        KEYCODE_F20 => NamedKey::F20,
        KEYCODE_F21 => NamedKey::F21,
        KEYCODE_F22 => NamedKey::F22,
        KEYCODE_F23 => NamedKey::F23,
        KEYCODE_F24 => NamedKey::F24,

        KEYCODE_VOLUME_UP => NamedKey::AudioVolumeUp,
        KEYCODE_VOLUME_DOWN => NamedKey::AudioVolumeDown,
        KEYCODE_VOLUME_MUTE => NamedKey::AudioVolumeMute,
        KEYCODE_MEDIA_PLAY_PAUSE => NamedKey::MediaPlayPause,
        KEYCODE_MEDIA_STOP => NamedKey::MediaStop,
        KEYCODE_MEDIA_NEXT => NamedKey::MediaTrackNext,
        KEYCODE_MEDIA_PREVIOUS => NamedKey::MediaTrackPrevious,
        KEYCODE_MEDIA_REWIND => NamedKey::MediaRewind,
        KEYCODE_MEDIA_FAST_FORWARD => NamedKey::MediaFastForward,
        KEYCODE_MEDIA_PLAY => NamedKey::MediaPlay,
        KEYCODE_MEDIA_PAUSE => NamedKey::MediaPause,
        KEYCODE_MUTE => NamedKey::MicrophoneVolumeMute,
        KEYCODE_MEDIA_EJECT => NamedKey::Eject,
        KEYCODE_MEDIA_CLOSE => NamedKey::MediaClose,
        KEYCODE_MEDIA_RECORD => NamedKey::MediaRecord,
        KEYCODE_MEDIA_SKIP_FORWARD => NamedKey::MediaSkipForward,
        KEYCODE_MEDIA_SKIP_BACKWARD => NamedKey::MediaSkipBackward,
        KEYCODE_MEDIA_STEP_FORWARD => NamedKey::MediaStepForward,
        KEYCODE_MEDIA_STEP_BACKWARD => NamedKey::MediaStepBackward,

        KEYCODE_POWER => NamedKey::Power,
        KEYCODE_SLEEP => NamedKey::Standby,
        KEYCODE_WAKEUP => NamedKey::WakeUp,
        KEYCODE_BRIGHTNESS_UP => NamedKey::BrightnessUp,
        KEYCODE_BRIGHTNESS_DOWN => NamedKey::BrightnessDown,
        KEYCODE_TV_POWER => NamedKey::TVPower,
        KEYCODE_STB_POWER => NamedKey::STBPower,
        KEYCODE_AVR_POWER => NamedKey::AVRPower,

        KEYCODE_FORWARD => NamedKey::BrowserForward, // Not exact.
        KEYCODE_SEARCH => NamedKey::BrowserSearch,
        KEYCODE_REFRESH => NamedKey::BrowserRefresh,

        KEYCODE_CALCULATOR => NamedKey::LaunchApplication2,
        KEYCODE_ENVELOPE => NamedKey::LaunchMail,
        KEYCODE_EXPLORER => NamedKey::LaunchWebBrowser,
        KEYCODE_CONTACTS => NamedKey::LaunchContacts,
        KEYCODE_CALENDAR => NamedKey::LaunchCalendar,
        KEYCODE_MUSIC => NamedKey::LaunchMusicPlayer,

        KEYCODE_HENKAN => NamedKey::Convert,
        KEYCODE_MUHENKAN => NamedKey::NonConvert,
        KEYCODE_KATAKANA_HIRAGANA => NamedKey::HiraganaKatakana,
        KEYCODE_KANA => NamedKey::KanjiMode,
        KEYCODE_ZENKAKU_HANKAKU => NamedKey::ZenkakuHankaku,
        KEYCODE_EISU => NamedKey::Eisu,

        KEYCODE_ZOOM_IN => NamedKey::ZoomIn,
        KEYCODE_ZOOM_OUT => NamedKey::ZoomOut,
        KEYCODE_TV_ZOOM_MODE => NamedKey::ZoomToggle,

        KEYCODE_CHANNEL_UP => NamedKey::ChannelUp,
        KEYCODE_CHANNEL_DOWN => NamedKey::ChannelDown,
        KEYCODE_GUIDE => NamedKey::Guide,
        KEYCODE_INFO => NamedKey::Info,
        KEYCODE_SETTINGS => NamedKey::Settings,
        KEYCODE_TV => NamedKey::TV,
        KEYCODE_LAST_CHANNEL => NamedKey::MediaLast,
        KEYCODE_MEDIA_AUDIO_TRACK => NamedKey::MediaAudioTrack,
        KEYCODE_MEDIA_TOP_MENU => NamedKey::MediaTopMenu,
        KEYCODE_NAVIGATE_PREVIOUS => NamedKey::NavigatePrevious,
        KEYCODE_NAVIGATE_NEXT => NamedKey::NavigateNext,
        KEYCODE_NAVIGATE_IN => NamedKey::NavigateIn,
        KEYCODE_NAVIGATE_OUT => NamedKey::NavigateOut,
        KEYCODE_CAPTIONS => NamedKey::ClosedCaptionToggle,
        KEYCODE_TV_TELETEXT => NamedKey::Teletext,
        KEYCODE_TV_NUMBER_ENTRY => NamedKey::TVNumberEntry,
        KEYCODE_TV_TERRESTRIAL_ANALOG => NamedKey::TVTerrestrialAnalog,
        KEYCODE_TV_TERRESTRIAL_DIGITAL => NamedKey::TVTerrestrialDigital,
        KEYCODE_TV_SATELLITE => NamedKey::TVSatellite,
        KEYCODE_TV_SATELLITE_BS => NamedKey::TVSatelliteBS,
        KEYCODE_TV_SATELLITE_CS => NamedKey::TVSatelliteCS,
        KEYCODE_TV_SATELLITE_SERVICE => NamedKey::TVSatelliteToggle,
        KEYCODE_TV_NETWORK => NamedKey::TVNetwork,
        KEYCODE_TV_ANTENNA_CABLE => NamedKey::TVAntennaCable,
        KEYCODE_TV_INPUT => NamedKey::TVInput,
        KEYCODE_TV_INPUT_HDMI_1 => NamedKey::TVInputHDMI1,
        KEYCODE_TV_INPUT_HDMI_2 => NamedKey::TVInputHDMI2,
        KEYCODE_TV_INPUT_HDMI_3 => NamedKey::TVInputHDMI3,
        KEYCODE_TV_INPUT_HDMI_4 => NamedKey::TVInputHDMI4,
        KEYCODE_TV_INPUT_COMPOSITE_1 => NamedKey::TVInputComposite1,
        KEYCODE_TV_INPUT_COMPOSITE_2 => NamedKey::TVInputComposite2,
        KEYCODE_TV_INPUT_COMPONENT_1 => NamedKey::TVInputComponent1,
        KEYCODE_TV_INPUT_COMPONENT_2 => NamedKey::TVInputComponent2,
        KEYCODE_TV_INPUT_VGA_1 => NamedKey::TVInputVGA1,
        KEYCODE_TV_AUDIO_DESCRIPTION => NamedKey::TVAudioDescription,
        KEYCODE_TV_AUDIO_DESCRIPTION_MIX_UP => NamedKey::TVAudioDescriptionMixUp,
        KEYCODE_TV_AUDIO_DESCRIPTION_MIX_DOWN => NamedKey::TVAudioDescriptionMixDown,
        KEYCODE_TV_CONTENTS_MENU => NamedKey::TVContentsMenu,
        KEYCODE_TV_MEDIA_CONTEXT_MENU => NamedKey::TVMediaContext,
        KEYCODE_TV_TIMER_PROGRAMMING => NamedKey::TVTimer,
        KEYCODE_DVR => NamedKey::DVR,
        KEYCODE_STB_INPUT => NamedKey::STBInput,
        KEYCODE_AVR_INPUT => NamedKey::AVRInput,
        KEYCODE_3D_MODE => NamedKey::TV3DMode,

        KEYCODE_PROG_RED => NamedKey::ColorF0Red,
        KEYCODE_PROG_GREEN => NamedKey::ColorF1Green,
        KEYCODE_PROG_YELLOW => NamedKey::ColorF2Yellow,
        KEYCODE_PROG_BLUE => NamedKey::ColorF3Blue,

        KEYCODE_11 => NamedKey::Key11,
        KEYCODE_12 => NamedKey::Key12,

        KEYCODE_PRINT => NamedKey::Print,

        KEYCODE_APP_SWITCH => NamedKey::AppSwitch,
        KEYCODE_CALL => NamedKey::Call,
        KEYCODE_ENDCALL => NamedKey::EndCall,
        KEYCODE_CAMERA => NamedKey::Camera,
        KEYCODE_FOCUS => NamedKey::CameraFocus,
        KEYCODE_HEADSETHOOK => NamedKey::HeadsetHook,
        KEYCODE_NOTIFICATION => NamedKey::Notification,
        KEYCODE_MANNER_MODE => NamedKey::MannerMode,
        KEYCODE_PAIRING => NamedKey::Pairing,

        _ => NamedKey::Unidentified,
    }
}
