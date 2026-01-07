// Copyright 2025 the UI Events Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Minimal example displaying pointer events from `ui-events-winit`.

use anyhow::Result;
use std::collections::VecDeque;
use std::sync::Arc;

use ui_events::pointer::PointerEvent;
use ui_events_winit::{WindowEventReducer, WindowEventTranslation};
use vello::kurbo::{Affine, Circle, Stroke};
use vello::peniko::Color;
use vello::util::{RenderContext, RenderSurface};
use vello::{AaConfig, RenderParams, Renderer, RendererOptions, Scene};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

use vello::wgpu;

const BG_COLOR: Color = Color::new([0.066_666_67, 0.066_666_67, 0.066_666_67, 1.0]);
const COALESCED_COLOR: Color = Color::new([0.250_980_4, 0.627_451, 1.0, 0.3]);
const PREDICTED_COLOR: Color = Color::new([1.0, 0.627_451, 0.0, 0.45]);
const CURRENT_COLOR: Color = Color::new([0.250_980_4, 0.627_451, 1.0, 0.9]);
const DOWN_FILL: Color = Color::new([0.0, 0.784_313_74, 0.470_588_24, 0.25]);
const DOWN_STROKE: Color = Color::new([0.0, 0.784_313_74, 0.470_588_24, 0.9]);
const UP_FILL: Color = Color::new([1.0, 0.313_725_5, 0.313_725_5, 0.25]);
const UP_STROKE: Color = Color::new([1.0, 0.313_725_5, 0.313_725_5, 0.9]);

#[derive(Debug)]
enum RenderState {
    Active {
        surface: Box<RenderSurface<'static>>,
        valid_surface: bool,
        window: Arc<Window>,
    },
    Suspended(Option<Arc<Window>>),
}

struct Visualizer {
    current: VecDeque<(f64, f64)>,
    coalesced: VecDeque<(f64, f64)>,
    predicted: VecDeque<(f64, f64)>,
    downs: VecDeque<(f64, f64)>,
    ups: VecDeque<(f64, f64)>,
    dirty: bool,
}

impl Visualizer {
    const CAP: usize = 4096;

    fn new() -> Self {
        Self {
            current: VecDeque::with_capacity(Self::CAP),
            coalesced: VecDeque::with_capacity(Self::CAP),
            predicted: VecDeque::with_capacity(Self::CAP),
            downs: VecDeque::with_capacity(256),
            ups: VecDeque::with_capacity(256),
            dirty: true,
        }
    }

    fn push_cap<T>(buf: &mut VecDeque<T>, val: T, cap: usize) {
        if buf.len() >= cap {
            buf.pop_front();
        }
        buf.push_back(val);
    }

    fn handle_pointer(&mut self, pe: &PointerEvent) {
        use ui_events::pointer::PointerEvent as PE;
        match pe {
            PE::Move(update) => {
                for s in &update.coalesced {
                    Self::push_cap(&mut self.coalesced, (s.position.x, s.position.y), Self::CAP);
                }
                for s in &update.predicted {
                    Self::push_cap(&mut self.predicted, (s.position.x, s.position.y), Self::CAP);
                }
                let s = &update.current;
                Self::push_cap(&mut self.current, (s.position.x, s.position.y), Self::CAP);
                self.dirty = true;
            }
            PE::Down(btn) => {
                let s = &btn.state;
                Self::push_cap(&mut self.downs, (s.position.x, s.position.y), 512);
                self.dirty = true;
            }
            PE::Up(btn) => {
                let s = &btn.state;
                Self::push_cap(&mut self.ups, (s.position.x, s.position.y), 512);
                self.dirty = true;
            }
            _ => {}
        }
    }

    fn rebuild_scene(&mut self, scene: &mut Scene) {
        if !self.dirty {
            return;
        }

        scene.reset();

        // Coalesced.
        for &(x, y) in &self.coalesced {
            let dot = Circle::new((x, y), 2.0);
            scene.fill(
                vello::peniko::Fill::NonZero,
                Affine::IDENTITY,
                COALESCED_COLOR,
                None,
                &dot,
            );
        }

        // Predicted.
        for &(x, y) in &self.predicted {
            let dot = Circle::new((x, y), 3.0);
            scene.fill(
                vello::peniko::Fill::NonZero,
                Affine::IDENTITY,
                PREDICTED_COLOR,
                None,
                &dot,
            );
        }

        // Current trail.
        let dot_stroke = Stroke::new(2.0);
        for &(x, y) in &self.current {
            let dot = Circle::new((x, y), 3.0);
            scene.fill(
                vello::peniko::Fill::NonZero,
                Affine::IDENTITY,
                CURRENT_COLOR,
                None,
                &dot,
            );
            scene.stroke(&dot_stroke, Affine::IDENTITY, CURRENT_COLOR, None, &dot);
        }

        // Down rings.
        for &(x, y) in &self.downs {
            let ring = Circle::new((x, y), 9.0);
            scene.fill(
                vello::peniko::Fill::NonZero,
                Affine::IDENTITY,
                DOWN_FILL,
                None,
                &ring,
            );
            scene.stroke(&dot_stroke, Affine::IDENTITY, DOWN_STROKE, None, &ring);
        }

        // Up rings.
        for &(x, y) in &self.ups {
            let ring = Circle::new((x, y), 7.0);
            scene.fill(
                vello::peniko::Fill::NonZero,
                Affine::IDENTITY,
                UP_FILL,
                None,
                &ring,
            );
            scene.stroke(&dot_stroke, Affine::IDENTITY, UP_STROKE, None, &ring);
        }

        self.dirty = false;
    }
}

fn create_vello_renderer(render_cx: &RenderContext, surface: &RenderSurface<'_>) -> Renderer {
    Renderer::new(
        &render_cx.devices[surface.dev_id].device,
        RendererOptions::default(),
    )
    .expect("Couldn't create renderer")
}

fn render_frame(
    context: &RenderContext,
    renderers: &mut [Option<Renderer>],
    scene: &mut Scene,
    viz: &mut Visualizer,
    window: &Window,
    surface: &mut RenderSurface<'_>,
) {
    viz.rebuild_scene(scene);

    let width = surface.config.width;
    let height = surface.config.height;
    let device_handle = &context.devices[surface.dev_id];

    renderers[surface.dev_id]
        .as_mut()
        .expect("renderer missing")
        .render_to_texture(
            &device_handle.device,
            &device_handle.queue,
            scene,
            &surface.target_view,
            &RenderParams {
                base_color: BG_COLOR,
                width,
                height,
                antialiasing_method: AaConfig::Msaa16,
            },
        )
        .expect("render_to_texture failed");

    let surface_texture = surface
        .surface
        .get_current_texture()
        .expect("failed to get surface texture");

    let mut encoder =
        device_handle
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Surface Blit"),
            });
    surface.blitter.copy(
        &device_handle.device,
        &mut encoder,
        &surface.target_view,
        &surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default()),
    );
    device_handle.queue.submit([encoder.finish()]);

    window.pre_present_notify();
    surface_texture.present();

    device_handle.device.poll(wgpu::PollType::Poll).ok();
}

struct SimpleWinitApp {
    context: RenderContext,
    renderers: Vec<Option<Renderer>>,
    state: RenderState,
    reducer: WindowEventReducer,
    scene: Scene,
    viz: Visualizer,
}

impl SimpleWinitApp {
    fn new() -> Self {
        Self {
            context: RenderContext::new(),
            renderers: vec![],
            state: RenderState::Suspended(None),
            reducer: WindowEventReducer::default(),
            scene: Scene::new(),
            viz: Visualizer::new(),
        }
    }
}

impl ApplicationHandler for SimpleWinitApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let RenderState::Suspended(cached_window) = &mut self.state else {
            return;
        };

        let window = cached_window.take().unwrap_or_else(|| {
            let attr = Window::default_attributes()
                .with_inner_size(LogicalSize::new(384_u32, 384_u32))
                .with_resizable(true)
                .with_title("ui-events: simple_winit");
            Arc::new(event_loop.create_window(attr).expect("create_window"))
        });

        let size = window.inner_size();
        let surface_future = self.context.create_surface(
            window.clone(),
            size.width,
            size.height,
            wgpu::PresentMode::AutoVsync,
        );
        let surface = pollster::block_on(surface_future).expect("create_surface");

        self.renderers
            .resize_with(self.context.devices.len(), || None);
        self.renderers[surface.dev_id]
            .get_or_insert_with(|| create_vello_renderer(&self.context, &surface));

        self.state = RenderState::Active {
            surface: Box::new(surface),
            valid_surface: size.width != 0 && size.height != 0,
            window,
        };

        if let RenderState::Active { window, .. } = &self.state {
            window.request_redraw();
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        if let RenderState::Active { window, .. } = &self.state {
            self.state = RenderState::Suspended(Some(window.clone()));
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let window = match &self.state {
            RenderState::Active { window, .. } if window.id() == window_id => window.clone(),
            _ => return,
        };

        if let Some(WindowEventTranslation::Pointer(pe)) =
            self.reducer.reduce(window.scale_factor(), &event)
        {
            self.viz.handle_pointer(&pe);
            window.request_redraw();
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                let RenderState::Active {
                    surface,
                    valid_surface,
                    ..
                } = &mut self.state
                else {
                    return;
                };

                if size.width == 0 || size.height == 0 {
                    *valid_surface = false;
                    return;
                }
                self.context
                    .resize_surface(surface, size.width, size.height);
                *valid_surface = true;
                self.viz.dirty = true;
                window.request_redraw();
            }
            WindowEvent::ScaleFactorChanged { .. } => {
                let RenderState::Active {
                    surface,
                    valid_surface,
                    ..
                } = &mut self.state
                else {
                    return;
                };

                let size = window.inner_size();
                if size.width == 0 || size.height == 0 {
                    *valid_surface = false;
                    return;
                }
                self.context
                    .resize_surface(surface, size.width, size.height);
                *valid_surface = true;
                self.viz.dirty = true;
                window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                let RenderState::Active {
                    surface,
                    valid_surface,
                    ..
                } = &mut self.state
                else {
                    return;
                };
                if !*valid_surface {
                    return;
                }

                render_frame(
                    &self.context,
                    &mut self.renderers,
                    &mut self.scene,
                    &mut self.viz,
                    window.as_ref(),
                    surface,
                );
            }
            _ => {}
        }
    }
}

fn main() -> Result<()> {
    let mut app = SimpleWinitApp::new();
    let event_loop = EventLoop::new()?;
    event_loop.run_app(&mut app)?;
    Ok(())
}
