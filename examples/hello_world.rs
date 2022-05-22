extern crate juke;

use juke::{egui::Context, *};
use std::time::Duration;

static mut FRAME_TIME: Duration = Duration::ZERO;

fn main() {
    Engine::new("Hello, World! - ESC to exit", 256, 144, 4)
        .ui(ui)
        .run(update);
}

fn update(f: FrameContext) {
    for pixel in f.buffer.chunks_exact_mut(4) {
        pixel[0] = 0xff; // R
        pixel[1] = 0x00; // G
        pixel[2] = 0xff; // B
        pixel[3] = 0xff; // A
    }

    unsafe { FRAME_TIME = f.delta };
}

fn ui(ctx: &Context) {
    egui::Window::new("Hello, egui!").show(ctx, |ui| {
        ui.label("This example demonstrates using egui with juke.");

        ui.separator();

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x /= 2.0;
            unsafe {
                ui.label(format!("Frame Time: {:?}", FRAME_TIME));
            }
            unsafe {
                ui.label(format!("FPS: {}", 1. / FRAME_TIME.as_secs_f32()));
            }
        });
    });
}
