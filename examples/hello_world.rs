extern crate juke;

use juke::{gizmos, math::vector::*, *};

fn main() {
    let res = Engine::new("Hello, World! - ESC to exit", 320, 180, 3).run(|e: &mut Engine| {
        let pos = Vector2::new(e.buffer.w / 2, e.buffer.h / 2);
        gizmos::circle(Color(255, 0, 255), &pos, 50, &mut e.buffer);

        Ok(())
    });

    match res {
        Ok(()) => println!("exited successfully!"),
        Err(e) => panic!("{}", e),
    }
}
