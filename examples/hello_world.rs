extern crate juke;

use juke::*;
use std::time::Duration;

fn main() {
    let res = Engine::new("Hello, World! - ESC to exit", 256, 144, 3).run(
        |e: &mut Engine, delta: Duration| {
            e.buffer.clear();

            println!(
                "Frame Time: {:?}, FPS: {:?}",
                delta,
                1. / delta.as_secs_f32()
            );

            Ok(())
        },
    );

    match res {
        Ok(()) => println!("exited succesfully!"),
        Err(e) => panic!("{}", e),
    }
}
