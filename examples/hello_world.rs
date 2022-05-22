extern crate juke;

use juke::{egui::Context, resources::ResourceManager, *};
use std::time::Duration;

fn main() {
    Engine::new("Hello, World! - ESC to exit", 256, 144, 4)
        .ui(ui)
        .run(update);
}

fn update(f: FrameContext, resources: &mut ResourceManager) {
    for pixel in f.buffer.chunks_exact_mut(4) {
        pixel[0] = 0xff; // R
        pixel[1] = 0x00; // G
        pixel[2] = 0xff; // B
        pixel[3] = 0xff; // A
    }

    resources.set("frame_time", f.delta);
}

fn ui(ctx: &Context, resources: &mut ResourceManager) {
    egui::Window::new("Hello, egui!").show(ctx, |ui| {
        ui.label("This example demonstrates using egui with juke.");

        ui.separator();

        ui.horizontal(|ui| {
            let delta = resources.get::<Duration>("frame_time").unwrap();

            ui.spacing_mut().item_spacing.x /= 2.0;
            ui.label(format!("Frame Time: {:?}", delta));
            ui.label(format!("FPS: {}", 1. / delta.as_secs_f32()));
        });
    });
}
