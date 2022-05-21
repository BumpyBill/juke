extern crate juke;

use juke::*;
use std::time::Duration;

fn main() {
    Engine::new("Hello, World! - ESC to exit", 256, 144, 4).run(|f: FrameContext| {
        for pixel in f.buffer.chunks_exact_mut(4) {
            pixel[0] = 0xff; // R
            pixel[1] = 0x00; // G
            pixel[2] = 0xff; // B
            pixel[3] = 0xff; // A
        }

        println!(
            "Frame Time: {:?}, FPS: {}",
            f.delta,
            1. / f.delta.as_secs_f32()
        );
    });
    //     |e: &mut Engine, delta: Duration| {
    //         println!(
    //             "Frame Time: {:?}, FPS: {:?}",
    //             delta,
    //             1. / delta.as_secs_f32()
    //         );
    //
    //         Ok(())
    //     },
    // );
    //
    // match res {
    //     Ok(()) => println!("exited succesfully!"),
    //     Err(e) => panic!("{}", e),
    // }
}
