//! 🤖 A small engine for prototyping projects

mod gui;
pub mod resources;

use resources::ResourceManager;

use crate::gui::Framework;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use std::time::{Duration, Instant};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

pub use egui;

pub struct Engine {
    window: Window,
    input: WinitInputHelper,
    pixels: Pixels,
    event_loop: EventLoop<()>,
    framework: Framework,

    resources: ResourceManager
}

impl Engine {
    pub fn new(title: &str, w: u32, h: u32, pixel_size: u32) -> Self {
        env_logger::init();
        let event_loop = EventLoop::new();
        let input = WinitInputHelper::new();
        let window = {
            let size = LogicalSize::new((w * pixel_size) as f64, (h * pixel_size) as f64);
            WindowBuilder::new()
                .with_title(title)
                .with_inner_size(size)
                .with_resizable(false)
                .with_maximized(false)
                .build(&event_loop)
                .unwrap()
        };

        let (pixels, framework) = {
            let window_size = window.inner_size();
            let scale_factor = window.scale_factor() as f32;
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            let pixels = Pixels::new(w, h, surface_texture).unwrap();
            let framework =
                Framework::new(window_size.width, window_size.height, scale_factor, &pixels);

            (pixels, framework)
        };

        Self {
            window,
            input,
            pixels,
            framework,
            event_loop,
            resources: ResourceManager::new(),
        }
    }

    pub fn run<F: 'static + Fn(FrameContext, &mut ResourceManager)>(mut self, u: F) {
        let mut frame_count = 0u128;
        let mut previous_time = Instant::now();
        const FRAME_TIME: Duration = Duration::from_micros(16666);

        self.event_loop.run(move |event, _, control_flow| {
            if self.input.update(&event) {
                if self.input.key_pressed(VirtualKeyCode::Escape) || self.input.quit() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                self.window.request_redraw();
            }

            match event {
                Event::WindowEvent { event, .. } => {
                    // Update egui inputs
                    self.framework.handle_event(&event);
                }

                Event::RedrawRequested(_) => {
                    let delta = {
                        let real_delta = previous_time.elapsed();
                        if real_delta < FRAME_TIME {
                            let sleep_time = FRAME_TIME - real_delta;
                            std::thread::sleep(sleep_time);

                            FRAME_TIME
                        } else {
                            real_delta
                        }
                    };

                    frame_count += 1;
                    previous_time = Instant::now();

                    u(FrameContext {
                        frame_count,
                        buffer: self.pixels.get_frame(),
                        delta,
                    }, &mut self.resources);
                    self.framework.prepare(&self.window, &mut self.resources);

                    let render_result =
                        self.pixels.render_with(|encoder, render_target, context| {
                            context.scaling_renderer.render(encoder, render_target);
                            self.framework.render(encoder, render_target, context)?;

                            Ok(())
                        });

                    if render_result
                        .map_err(|e| error!("pixels.render() failed: {}", e))
                        .is_err()
                    {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                _ => (),
            }
        });
    }

    pub fn ui<T: 'static + Fn(&egui::Context, &mut ResourceManager)>(mut self, ui: T) -> Self {
        self.framework.gui = Some(Box::new(ui));
        self
    }
}

pub struct FrameContext<'a> {
    pub frame_count: u128,
    pub buffer: &'a mut [u8],
    pub delta: Duration,
}
