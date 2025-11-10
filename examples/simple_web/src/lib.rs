// Copyright 2025 the UI Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Minimal web example wiring DOM events to `ui-events` via `ui-events-web`.
//!
//! Open the browser devtools console to see the logged events.

#![expect(
    missing_docs,
    reason = "Example crate: keep code compact without exhaustive docs"
)]

use std::cell::RefCell;
use std::collections::{BTreeSet, VecDeque};
use std::rc::Rc;

use ui_events_web::pointer::Options as PtrOptions;
use web_sys::wasm_bindgen::{closure::Closure, prelude::*, JsCast, JsValue};
use web_sys::{
    CanvasRenderingContext2d, Document, Event, EventTarget, HtmlCanvasElement, HtmlElement,
    HtmlInputElement, KeyboardEvent, Window,
};

thread_local! {
    // Keep closures alive for the lifetime of the page.
    #[expect(
        clippy::type_complexity,
        reason = "Store closures to keep listeners alive; verbose type is acceptable here"
    )]
    static _LISTENERS: RefCell<Vec<Closure<dyn FnMut(Event)>>> = Default::default();
}

fn window() -> Window {
    web_sys::window().expect("no window")
}

fn document() -> Document {
    window().document().expect("no document")
}

fn add_listener(target: &EventTarget, ty: &str, mut f: impl FnMut(Event) + 'static) {
    let cb = Closure::wrap(Box::new(move |e: Event| f(e)) as Box<dyn FnMut(Event)>);
    target
        .add_event_listener_with_callback(ty, cb.as_ref().unchecked_ref())
        .expect("add_event_listener");
    _LISTENERS.with(|v| v.borrow_mut().push(cb));
}

fn log_line(msg: &str) {
    web_sys::console::log_1(&JsValue::from_str(msg));
}

fn setup_canvas(win: &Window) -> (HtmlCanvasElement, CanvasRenderingContext2d) {
    let doc = win.document().expect("no document");
    let canvas: HtmlCanvasElement = doc.create_element("canvas").unwrap().dyn_into().unwrap();
    let canvas_style = concat!(
        "position:fixed;",
        "left:0;",
        "top:0;",
        "width:100vw;",
        "height:100vh;",
        "touch-action:none;",
        "z-index:0;",
        "background:#111;",
    );
    canvas.set_attribute("style", canvas_style).ok();
    let body = doc.body().unwrap();
    body.append_child(&canvas).unwrap();
    let ctx: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();
    resize_canvas(&canvas, &ctx, win);
    (canvas, ctx)
}

fn resize_canvas(canvas: &HtmlCanvasElement, ctx: &CanvasRenderingContext2d, win: &Window) {
    let dpr = win.device_pixel_ratio();
    let w = win.inner_width().unwrap().as_f64().unwrap();
    let h = win.inner_height().unwrap().as_f64().unwrap();
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Canvas backing store size intentionally truncated to u32 pixels"
    )]
    let bw = (w * dpr) as u32;
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Canvas backing store size intentionally truncated to u32 pixels"
    )]
    let bh = (h * dpr) as u32;
    canvas.set_width(bw);
    canvas.set_height(bh);
    ctx.set_fill_style_str("rgba(255,255,255,0.85)");
    ctx.set_stroke_style_str("#fff");
    ctx.set_line_width(2.0);
}

fn setup_hud(win: &Window) -> HtmlElement {
    let doc = win.document().unwrap();
    let hud: HtmlElement = doc.create_element("div").unwrap().dyn_into().unwrap();
    let hud_style = concat!(
        "position:fixed;",
        "left:8px;",
        "top:8px;",
        "z-index:1;",
        "color:#eee;",
        "background:rgba(0,0,0,0.55);",
        "padding:6px 8px;",
        "border-radius:6px;",
        "font:12px/1.3 system-ui,san-serif;",
    );
    hud.set_attribute("style", hud_style).ok();
    let hud_html = r#"
      <div style="margin-bottom:6px">
        <button id="clear">Clear</button>
        <label><input type="checkbox" id="show_coalesced" checked> Coalesced</label>
        <label><input type="checkbox" id="show_predicted" checked> Predicted</label>
      </div>
      <div><b>Pressed:</b> <span id="pressed"></span></div>
      <div><b>Last:</b> <span id="last"></span></div>
    "#;
    hud.set_inner_html(hud_html);
    let body = doc.body().unwrap();
    body.append_child(&hud).unwrap();
    hud
}

fn draw_dot(
    ctx: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    r: f64,
    fill: &str,
    stroke: Option<&str>,
) {
    ctx.begin_path();
    ctx.arc(x, y, r, 0.0, std::f64::consts::PI * 2.0).ok();
    if let Some(s) = stroke {
        ctx.set_stroke_style_str(s);
    }
    ctx.set_fill_style_str(fill);
    ctx.fill();
    if stroke.is_some() {
        ctx.stroke();
    }
}

struct Visualizer {
    win: Window,
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    pressed: BTreeSet<String>,
    show_coalesced: bool,
    show_predicted: bool,
    current: VecDeque<(f64, f64)>,
    coalesced: VecDeque<(f64, f64)>,
    predicted: VecDeque<(f64, f64)>,
    downs: VecDeque<(f64, f64)>,
    ups: VecDeque<(f64, f64)>,
    dirty: bool,
}

impl Visualizer {
    const CAP: usize = 4096;

    fn new(win: Window) -> Rc<RefCell<Self>> {
        let (canvas, ctx) = setup_canvas(&win);
        let _hud = setup_hud(&win);
        let viz = Rc::new(RefCell::new(Self {
            win,
            canvas,
            ctx,
            pressed: BTreeSet::new(),
            show_coalesced: true,
            show_predicted: true,
            current: VecDeque::with_capacity(Self::CAP),
            coalesced: VecDeque::with_capacity(Self::CAP),
            predicted: VecDeque::with_capacity(Self::CAP),
            downs: VecDeque::with_capacity(256),
            ups: VecDeque::with_capacity(256),
            dirty: true,
        }));

        // Hook up HUD controls
        Self::install_controls(viz.clone());

        viz
    }

    fn install_controls(viz: Rc<RefCell<Self>>) {
        let doc = viz.borrow().win.document().unwrap();
        // Clear button
        if let Some(el) = doc.get_element_by_id("clear") {
            let v = viz.clone();
            let cb = Closure::wrap(Box::new(move |_e: Event| {
                v.borrow_mut().clear();
            }) as Box<dyn FnMut(Event)>);
            el.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())
                .ok();
            _LISTENERS.with(|ls| ls.borrow_mut().push(cb));
        }
        // Coalesced toggle
        if let Some(el) = doc.get_element_by_id("show_coalesced") {
            if let Ok(input) = el.dyn_into::<HtmlInputElement>() {
                input.set_checked(true);
                let v = viz.clone();
                let cb = Closure::wrap(Box::new(move |e: Event| {
                    let target: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
                    v.borrow_mut().show_coalesced = target.checked();
                    v.borrow_mut().schedule_draw();
                }) as Box<dyn FnMut(Event)>);
                input
                    .add_event_listener_with_callback("change", cb.as_ref().unchecked_ref())
                    .ok();
                _LISTENERS.with(|ls| ls.borrow_mut().push(cb));
            }
        }
        // Predicted toggle
        if let Some(el) = doc.get_element_by_id("show_predicted") {
            if let Ok(input) = el.dyn_into::<HtmlInputElement>() {
                input.set_checked(true);
                let v = viz.clone();
                let cb = Closure::wrap(Box::new(move |e: Event| {
                    let target: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
                    v.borrow_mut().show_predicted = target.checked();
                    v.borrow_mut().schedule_draw();
                }) as Box<dyn FnMut(Event)>);
                input
                    .add_event_listener_with_callback("change", cb.as_ref().unchecked_ref())
                    .ok();
                _LISTENERS.with(|ls| ls.borrow_mut().push(cb));
            }
        }
    }

    fn resize(&mut self) {
        resize_canvas(&self.canvas, &self.ctx, &self.win);
        self.dirty = true;
        self.schedule_draw();
    }

    fn clear(&mut self) {
        self.current.clear();
        self.coalesced.clear();
        self.predicted.clear();
        self.downs.clear();
        self.ups.clear();
        // Clear the canvas
        let w = self.canvas.width() as f64;
        let h = self.canvas.height() as f64;
        self.ctx.clear_rect(0.0, 0.0, w, h);
        self.dirty = false;
    }

    fn push_cap<T>(buf: &mut VecDeque<T>, val: T, cap: usize) {
        if buf.len() >= cap {
            buf.pop_front();
        }
        buf.push_back(val);
    }

    fn handle_pointer(&mut self, pe: &ui_events::pointer::PointerEvent) {
        use ui_events::pointer::PointerEvent as PE;
        match pe {
            PE::Move(update) => {
                for s in &update.coalesced {
                    if self.show_coalesced {
                        Self::push_cap(
                            &mut self.coalesced,
                            (s.position.x, s.position.y),
                            Self::CAP,
                        );
                    }
                }
                for s in &update.predicted {
                    if self.show_predicted {
                        Self::push_cap(
                            &mut self.predicted,
                            (s.position.x, s.position.y),
                            Self::CAP,
                        );
                    }
                }
                let s = &update.current;
                Self::push_cap(&mut self.current, (s.position.x, s.position.y), Self::CAP);
                self.dirty = true;
                self.schedule_draw();
            }
            PE::Down(btn) => {
                let s = &btn.state;
                Self::push_cap(&mut self.downs, (s.position.x, s.position.y), 512);
                self.dirty = true;
                self.schedule_draw();
            }
            PE::Up(btn) => {
                let s = &btn.state;
                Self::push_cap(&mut self.ups, (s.position.x, s.position.y), 512);
                self.dirty = true;
                self.schedule_draw();
            }
            _ => {}
        }
    }

    fn handle_keyboard(&mut self, ke: &ui_events::keyboard::KeyboardEvent) {
        let label = if !matches!(ke.code, ui_events::keyboard::Code::Unidentified) {
            format!("{:?}", ke.code)
        } else {
            match &ke.key {
                ui_events::keyboard::Key::Character(s) => s.clone(),
                ui_events::keyboard::Key::Named(n) => format!("{:?}", n),
            }
        };
        match ke.state {
            ui_events::keyboard::KeyState::Down => {
                self.pressed.insert(label);
            }
            ui_events::keyboard::KeyState::Up => {
                self.pressed.remove(&label);
            }
        }
        self.update_hud(&format!(
            "state={:?} code={:?} repeat={} composing={}",
            ke.state, ke.code, ke.repeat, ke.is_composing
        ));
    }

    fn update_hud(&self, last: &str) {
        let doc = self.win.document().unwrap();
        if let Some(el) = doc.get_element_by_id("pressed") {
            let text = self.pressed.iter().cloned().collect::<Vec<_>>().join(", ");
            el.set_text_content(Some(&text));
        }
        if let Some(el) = doc.get_element_by_id("last") {
            el.set_text_content(Some(last));
        }
    }

    fn schedule_draw(&mut self) {
        // For simplicity in the demo, draw synchronously.
        self.draw_frame();
    }

    fn draw_frame(&mut self) {
        if !self.dirty {
            return;
        }
        let w = self.canvas.width() as f64;
        let h = self.canvas.height() as f64;
        self.ctx.clear_rect(0.0, 0.0, w, h);
        // Coalesced
        if self.show_coalesced {
            self.ctx.set_fill_style_str("rgba(64,160,255,0.3)");
            for &(x, y) in &self.coalesced {
                draw_dot(&self.ctx, x, y, 2.0, "rgba(64,160,255,0.3)", None);
            }
        }
        // Predicted
        if self.show_predicted {
            for &(x, y) in &self.predicted {
                draw_dot(&self.ctx, x, y, 3.0, "rgba(255,160,0,0.45)", None);
            }
        }
        // Current trail
        for &(x, y) in &self.current {
            draw_dot(
                &self.ctx,
                x,
                y,
                3.0,
                "rgba(64,160,255,0.9)",
                Some("rgba(64,160,255,0.9)"),
            );
        }
        // Down rings
        for &(x, y) in &self.downs {
            draw_dot(
                &self.ctx,
                x,
                y,
                9.0,
                "rgba(0,200,120,0.25)",
                Some("rgba(0,200,120,0.9)"),
            );
        }
        // Up rings
        for &(x, y) in &self.ups {
            draw_dot(
                &self.ctx,
                x,
                y,
                7.0,
                "rgba(255,80,80,0.25)",
                Some("rgba(255,80,80,0.9)"),
            );
        }
        self.dirty = false;
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    log_line("simple_web: starting; open the console to see events");

    let doc = document();
    let body: EventTarget = doc.body().expect("body").into();
    let win = window();
    let viz = Visualizer::new(win.clone());

    // Resize handler to keep canvas crisp on DPR/size change.
    {
        let win_ev: EventTarget = win.clone().into();
        let v = viz.clone();
        add_listener(&win_ev, "resize", move |_ev: Event| {
            v.borrow_mut().resize();
        });
    }

    // Pointer + wheel only. We avoid adding mouse events alongside pointer events
    // because browsers also synthesize compatibility mouse events, which would
    // double-report the same interaction in this example.
    for ty in [
        "pointerdown",
        "pointerup",
        "pointermove",
        "pointerenter",
        "pointerleave",
        "pointercancel",
        "wheel",
    ] {
        let v = viz.clone();
        add_listener(&body, ty, move |ev: Event| {
            let scale = window().device_pixel_ratio();
            let (show_coalesced, show_predicted) = {
                let vb = v.borrow();
                (vb.show_coalesced, vb.show_predicted)
            };
            let opts = PtrOptions::default()
                .with_scale(scale)
                .with_coalesced(show_coalesced)
                .with_predicted(show_predicted);
            if let Some(pe) = ui_events_web::pointer::pointer_event_from_dom_event(&ev, &opts) {
                log_line(&format!("pointer: {:?}", pe));
                v.borrow_mut().handle_pointer(&pe);
            }
        });
    }

    // Keyboard
    for ty in ["keydown", "keyup"] {
        let v = viz.clone();
        add_listener(&doc, ty, move |ev: Event| {
            let Some(k) = ev.dyn_ref::<KeyboardEvent>() else {
                return;
            };
            let ke = ui_events_web::keyboard::from_web_keyboard_event(k);
            v.borrow_mut().handle_keyboard(&ke);
        });
    }

    Ok(())
}
