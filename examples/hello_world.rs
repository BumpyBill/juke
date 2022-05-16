extern crate juke;

use juke::{gizmos, math::u32::UVec2, *};

fn main() {
    let res = Engine::new("Hello, World! - ESC to exit", 256, 144, 3).run(|e: &mut Engine| {
        let pos = UVec2::new(e.buffer.w as u32 / 2, e.buffer.h as u32 / 2);
        gizmos::circle(Color(255, 0, 255), &pos, 50, &mut e.buffer);

        Ok(())
    });

    match res {
        Ok(()) => println!("exited succesfully!"),
        Err(e) => panic!("{}", e),
    }
}
