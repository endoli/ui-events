// Copyright 2025 the UI Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Support routines for converting pointer data from [`web_sys`].

use alloc::vec::Vec;

use dpi::{PhysicalPosition, PhysicalSize};
use js_sys::{Array, Function, Reflect};
use ui_events::keyboard::Modifiers;
use ui_events::pointer::{
    PointerButton, PointerButtonEvent, PointerButtons, PointerEvent, PointerId, PointerInfo,
    PointerState, PointerType, PointerUpdate,
};
use ui_events::ScrollDelta;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{Element, Event, MouseEvent, PointerEvent as WebPointerEvent, WheelEvent};

#[inline]
#[expect(
    clippy::cast_possible_truncation,
    reason = "DOM timestamp is f64 ms; convert to integer ns intentionally"
)]
fn ms_to_ns_u64(ms: f64) -> u64 {
    (ms * 1_000_000.0) as u64
}

#[inline]
#[expect(
    clippy::cast_possible_truncation,
    reason = "DOM wheel line/page deltas are f64; ui-events stores f32"
)]
fn f64_to_f32_delta(v: f64) -> f32 {
    v as f32
}

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

/// Convert a DOM `MouseEvent.buttons()` bitfield into [`PointerButtons`].
pub fn from_web_buttons_mask(mask: u16) -> PointerButtons {
    // Compute in u32 to avoid shifting a 16-bit value by >= 16 (which panics in debug on wasm).
    let mask32 = mask as u32;
    let mut out = PointerButtons::default();
    for (i, btn) in NONZERO_VARIANTS.iter().enumerate() {
        if (mask32 & (1_u32 << i)) != 0 {
            out.insert(*btn);
        }
    }
    out
}

const NONZERO_VARIANTS: [PointerButton; 32] = [
    PointerButton::Primary,
    PointerButton::Secondary,
    PointerButton::Auxiliary,
    PointerButton::X1,
    PointerButton::X2,
    PointerButton::PenEraser,
    PointerButton::B7,
    PointerButton::B8,
    PointerButton::B9,
    PointerButton::B10,
    PointerButton::B11,
    PointerButton::B12,
    PointerButton::B13,
    PointerButton::B14,
    PointerButton::B15,
    PointerButton::B16,
    PointerButton::B17,
    PointerButton::B18,
    PointerButton::B19,
    PointerButton::B20,
    PointerButton::B21,
    PointerButton::B22,
    PointerButton::B23,
    PointerButton::B24,
    PointerButton::B25,
    PointerButton::B26,
    PointerButton::B27,
    PointerButton::B28,
    PointerButton::B29,
    PointerButton::B30,
    PointerButton::B31,
    PointerButton::B32,
];

/// Build a basic [`PointerState`] from a [`MouseEvent`].
///
/// Prefer [`state_from_pointer_event`] when handling W3C Pointer Events,
/// as it includes richer data (pressure, width/height, etc.).
///
/// - Coordinates use `clientX/Y` scaled by `scale_factor` to approximate physical pixels.
/// - Pressure is 0.5 when any button is down, else 0.0.
pub fn state_from_mouse_event(e: &MouseEvent, scale_factor: f64) -> PointerState {
    let css_x = e.client_x() as f64;
    let css_y = e.client_y() as f64;
    let buttons = from_web_buttons_mask(e.buttons());
    let pressure = if buttons.is_empty() { 0.0 } else { 0.5 };
    let time_ns = ms_to_ns_u64(e.time_stamp());
    PointerState {
        time: time_ns, // ms -> ns
        position: PhysicalPosition {
            x: css_x * scale_factor,
            y: css_y * scale_factor,
        },
        buttons,
        modifiers: modifiers_from_mouse(e),
        count: e.detail().clamp(0, 255) as u8,
        contact_geometry: PhysicalSize {
            width: 1.0,
            height: 1.0,
        },
        orientation: Default::default(),
        pressure,
        tangential_pressure: 0.0,
        scale_factor,
    }
}

fn modifiers_from_mouse(e: &MouseEvent) -> Modifiers {
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

fn pointer_info_mouse() -> PointerInfo {
    PointerInfo {
        pointer_id: Some(PointerId::PRIMARY),
        persistent_device_id: None,
        pointer_type: PointerType::Mouse,
    }
}

/// Build a `Down` from a DOM `mousedown`/`pointerdown` represented as [`MouseEvent`].
///
/// Prefer [`down_from_pointer_event`] when handling W3C Pointer Events.
pub fn down_from_mouse_event(e: &MouseEvent, scale_factor: f64) -> PointerEvent {
    PointerEvent::Down(PointerButtonEvent {
        button: try_from_web_button(e.button()),
        pointer: pointer_info_mouse(),
        state: state_from_mouse_event(e, scale_factor),
    })
}

/// Build an `Up` from a DOM `mouseup`/`pointerup` represented as [`MouseEvent`].
///
/// Prefer [`up_from_pointer_event`] when handling W3C Pointer Events.
pub fn up_from_mouse_event(e: &MouseEvent, scale_factor: f64) -> PointerEvent {
    PointerEvent::Up(PointerButtonEvent {
        button: try_from_web_button(e.button()),
        pointer: pointer_info_mouse(),
        state: state_from_mouse_event(e, scale_factor),
    })
}

/// Build a `Move` from a DOM `mousemove`/`pointermove` represented as [`MouseEvent`].
///
/// Prefer [`move_from_pointer_event`] when handling W3C Pointer Events.
pub fn move_from_mouse_event(e: &MouseEvent, scale_factor: f64) -> PointerEvent {
    PointerEvent::Move(PointerUpdate {
        pointer: pointer_info_mouse(),
        current: state_from_mouse_event(e, scale_factor),
        coalesced: Vec::new(),
        predicted: Vec::new(),
    })
}

/// Build an `Enter` from a DOM `mouseenter`/`pointerenter`.
///
/// Prefer [`enter_from_pointer_event`] when handling W3C Pointer Events.
pub fn enter_from_mouse_event(_e: &MouseEvent) -> PointerEvent {
    PointerEvent::Enter(pointer_info_mouse())
}

/// Build a `Leave` from a DOM `mouseleave`/`pointerleave`.
///
/// Prefer [`leave_from_pointer_event`] when handling W3C Pointer Events.
pub fn leave_from_mouse_event(_e: &MouseEvent) -> PointerEvent {
    PointerEvent::Leave(pointer_info_mouse())
}

/// Build a `Scroll` from a DOM `wheel` event.
///
/// `scale_factor` controls conversion of CSS pixel deltas to physical pixels.
pub fn scroll_from_wheel_event(e: &WheelEvent, scale_factor: f64) -> PointerEvent {
    let delta = match e.delta_mode() {
        WheelEvent::DOM_DELTA_PIXEL => ScrollDelta::PixelDelta(PhysicalPosition {
            x: e.delta_x() * scale_factor,
            y: e.delta_y() * scale_factor,
        }),
        WheelEvent::DOM_DELTA_LINE => {
            ScrollDelta::LineDelta(f64_to_f32_delta(e.delta_x()), f64_to_f32_delta(e.delta_y()))
        }
        WheelEvent::DOM_DELTA_PAGE => {
            ScrollDelta::PageDelta(f64_to_f32_delta(e.delta_x()), f64_to_f32_delta(e.delta_y()))
        }
        _ => ScrollDelta::PixelDelta(PhysicalPosition { x: 0.0, y: 0.0 }),
    };

    let me: &MouseEvent = e;
    PointerEvent::Scroll(ui_events::pointer::PointerScrollEvent {
        pointer: pointer_info_mouse(),
        delta,
        state: state_from_mouse_event(me, scale_factor),
    })
}

// PointerEvent (Web) conversions

fn pointer_type_from_str(s: &str) -> PointerType {
    match s {
        "mouse" => PointerType::Mouse,
        "pen" => PointerType::Pen,
        "touch" => PointerType::Touch,
        _ => PointerType::Unknown,
    }
}

fn pointer_info_from_web_pointer(e: &WebPointerEvent) -> PointerInfo {
    let id = if e.is_primary() {
        Some(PointerId::PRIMARY)
    } else {
        let raw = e.pointer_id() as u64;
        // Shift non-primary ids by +1 to avoid colliding with PRIMARY (1).
        PointerId::new(raw.saturating_add(1))
    };
    PointerInfo {
        pointer_id: id,
        persistent_device_id: None,
        pointer_type: pointer_type_from_str(&e.pointer_type()),
    }
}

fn modifiers_from_pointer(e: &WebPointerEvent) -> Modifiers {
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

/// Build a [`PointerState`] from a DOM [`web_sys::PointerEvent`].
///
/// - Coordinates use `clientX/Y` scaled by `scale_factor` to approximate
///   physical pixels.
/// - Uses the event's reported `pressure`, `tangentialPressure`, and
///   `width/height` where available.
pub fn state_from_pointer_event(e: &WebPointerEvent, scale_factor: f64) -> PointerState {
    let css_x = e.client_x() as f64;
    let css_y = e.client_y() as f64;
    let buttons = from_web_buttons_mask(e.buttons());
    let pressure = e.pressure();
    let tangential_pressure = e.tangential_pressure();
    let width = e.width() as f64 * scale_factor;
    let height = e.height() as f64 * scale_factor;
    let time_ns = ms_to_ns_u64(e.time_stamp());
    PointerState {
        time: time_ns,
        position: PhysicalPosition {
            x: css_x * scale_factor,
            y: css_y * scale_factor,
        },
        buttons,
        modifiers: modifiers_from_pointer(e),
        count: e.detail().clamp(0, 255) as u8,
        contact_geometry: PhysicalSize { width, height },
        orientation: Default::default(),
        pressure,
        tangential_pressure,
        scale_factor,
    }
}

/// Build a [`PointerEvent::Down`] from a DOM `pointerdown`.
pub fn down_from_pointer_event(e: &WebPointerEvent, scale_factor: f64) -> PointerEvent {
    PointerEvent::Down(PointerButtonEvent {
        button: try_from_web_button(e.button()),
        pointer: pointer_info_from_web_pointer(e),
        state: state_from_pointer_event(e, scale_factor),
    })
}

/// Build an [`PointerEvent::Up`] from a DOM `pointerup`.
pub fn up_from_pointer_event(e: &WebPointerEvent, scale_factor: f64) -> PointerEvent {
    PointerEvent::Up(PointerButtonEvent {
        button: try_from_web_button(e.button()),
        pointer: pointer_info_from_web_pointer(e),
        state: state_from_pointer_event(e, scale_factor),
    })
}

/// Controls how pointer events are converted.
#[derive(Clone, Copy, Debug)]
pub struct Options {
    /// Scale factor to convert CSS pixels to physical pixels.
    pub scale_factor: f64,
    /// Whether to collect coalesced move samples.
    pub collect_coalesced: bool,
    /// Whether to collect predicted move samples.
    pub collect_predicted: bool,
}

impl Default for Options {
    fn default() -> Self {
        // Defaults avoid allocations on hot paths; enable explicitly when desired.
        Self {
            scale_factor: 1.0,
            collect_coalesced: false,
            collect_predicted: false,
        }
    }
}

impl Options {
    /// Set the scale factor (builder style).
    pub fn with_scale(mut self, scale: f64) -> Self {
        self.scale_factor = scale;
        self
    }
    /// Set whether to collect coalesced samples.
    pub fn with_coalesced(mut self, enabled: bool) -> Self {
        self.collect_coalesced = enabled;
        self
    }
    /// Set whether to collect predicted samples.
    pub fn with_predicted(mut self, enabled: bool) -> Self {
        self.collect_predicted = enabled;
        self
    }
}

/// Build a `Move` from a DOM `pointermove`, with conversion options.
pub fn move_from_pointer_event(e: &WebPointerEvent, opts: &Options) -> PointerEvent {
    let pointer = pointer_info_from_web_pointer(e);
    let current = state_from_pointer_event(e, opts.scale_factor);

    let coalesced_states = if opts.collect_coalesced {
        get_coalesced_events_safe(e, opts.scale_factor)
    } else {
        Vec::new()
    };

    let predicted_states = if opts.collect_predicted {
        get_predicted_events_safe(e, opts.scale_factor)
    } else {
        Vec::new()
    };

    PointerEvent::Move(PointerUpdate {
        pointer,
        current,
        coalesced: coalesced_states,
        predicted: predicted_states,
    })
}

fn collect_states_from_array(arr: &Array, scale_factor: f64) -> Vec<PointerState> {
    let mut out = Vec::new();
    let len = arr.length();
    for i in 0..len {
        let v = arr.get(i);
        if let Ok(pe) = v.dyn_into::<WebPointerEvent>() {
            out.push(state_from_pointer_event(&pe, scale_factor));
        }
    }
    out
}

fn get_coalesced_events_safe(e: &WebPointerEvent, scale_factor: f64) -> Vec<PointerState> {
    let obj = e.as_ref();
    let Ok(v) = Reflect::get(
        obj,
        &web_sys::wasm_bindgen::JsValue::from_str("getCoalescedEvents"),
    ) else {
        return Vec::new();
    };
    if !v.is_function() {
        return Vec::new();
    }
    let f: Function = v.unchecked_into();
    let Ok(jsarr) = f.call0(obj) else {
        return Vec::new();
    };
    let Ok(arr) = jsarr.dyn_into::<Array>() else {
        return Vec::new();
    };
    collect_states_from_array(&arr, scale_factor)
}

fn get_predicted_events_safe(e: &WebPointerEvent, scale_factor: f64) -> Vec<PointerState> {
    let obj = e.as_ref();
    let Ok(v) = Reflect::get(
        obj,
        &web_sys::wasm_bindgen::JsValue::from_str("getPredictedEvents"),
    ) else {
        return Vec::new();
    };
    if !v.is_function() {
        return Vec::new();
    }
    let f: Function = v.unchecked_into();
    let Ok(jsarr) = f.call0(obj) else {
        return Vec::new();
    };
    let Ok(arr) = jsarr.dyn_into::<Array>() else {
        return Vec::new();
    };
    collect_states_from_array(&arr, scale_factor)
}

/// Build an [`PointerEvent::Enter`] from a DOM `pointerenter`.
pub fn enter_from_pointer_event(e: &WebPointerEvent) -> PointerEvent {
    PointerEvent::Enter(pointer_info_from_web_pointer(e))
}

/// Build a [`PointerEvent::Leave`] from a DOM `pointerleave`.
pub fn leave_from_pointer_event(e: &WebPointerEvent) -> PointerEvent {
    PointerEvent::Leave(pointer_info_from_web_pointer(e))
}

/// Build a [`PointerEvent::Cancel`] from a DOM `pointercancel`.
pub fn cancel_from_pointer_event(e: &WebPointerEvent) -> PointerEvent {
    PointerEvent::Cancel(pointer_info_from_web_pointer(e))
}

/// Convert a DOM event (Mouse/Pointer/Wheel) into a `ui-events` [`PointerEvent`]
/// with options to control conversion.
pub fn pointer_event_from_dom_event(ev: &Event, opts: &Options) -> Option<PointerEvent> {
    if let Some(wheel) = ev.dyn_ref::<WheelEvent>() {
        return Some(scroll_from_wheel_event(wheel, opts.scale_factor));
    }
    if let Some(pe) = ev.dyn_ref::<WebPointerEvent>() {
        return Some(match pe.type_().as_str() {
            "pointerdown" => down_from_pointer_event(pe, opts.scale_factor),
            "pointerup" => up_from_pointer_event(pe, opts.scale_factor),
            "pointermove" => move_from_pointer_event(pe, opts),
            "pointerenter" => enter_from_pointer_event(pe),
            "pointerleave" => leave_from_pointer_event(pe),
            "pointercancel" => cancel_from_pointer_event(pe),
            _ => return None,
        });
    }
    if let Some(me) = ev.dyn_ref::<MouseEvent>() {
        return Some(match me.type_().as_str() {
            "mousedown" => down_from_mouse_event(me, opts.scale_factor),
            "mouseup" => up_from_mouse_event(me, opts.scale_factor),
            "mousemove" => move_from_mouse_event(me, opts.scale_factor),
            "mouseenter" => enter_from_mouse_event(me),
            "mouseleave" => leave_from_mouse_event(me),
            _ => return None,
        });
    }
    None
}

/// Set pointer capture on an element using the id from a `PointerEvent`.
pub fn set_pointer_capture(
    el: &Element,
    e: &WebPointerEvent,
) -> Result<(), web_sys::js_sys::JsString> {
    Ok(el.set_pointer_capture(e.pointer_id())?)
}

/// Release pointer capture on an element using the id from a `PointerEvent`.
pub fn release_pointer_capture(
    el: &Element,
    e: &WebPointerEvent,
) -> Result<(), web_sys::js_sys::JsString> {
    Ok(el.release_pointer_capture(e.pointer_id())?)
}

/// Query whether an element currently has capture for this pointer id.
pub fn has_pointer_capture(el: &Element, e: &WebPointerEvent) -> bool {
    el.has_pointer_capture(e.pointer_id())
}
