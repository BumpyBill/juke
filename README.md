<p>
  <a href="https://crates.io/crates/juke" target="_blank">
    <img alt="Version" src="https://img.shields.io/crates/v/juke">
  </a>
  <a href="https://docs./juke" target="_blank">
    <img alt="Documentation" src="https://img.shields.io/docsrs/juke" />
  </a>
</p>

> 🤖 A small engine for prototyping projects

## Usage

```toml
# Cargo.toml
[dependencies]
minifb = "0.23"
```


## Example
```rs
use juke::{gizmos, math::vector::*, *};

fn main() {
    let res = Engine::new("Hello, World! - ESC to exit", 320, 180, 3).run(|e: &mut Engine| {
        let pos = Vector2::new(e.buffer.w / 2, e.buffer.h / 2);
        gizmos::circle(Color(255, 0, 255), &pos, 50, &mut e.buffer);

        Ok(())
    });

    match res {
        Ok(()) => println!("exited succesfully!"),
        Err(e) => panic!("{}", e),
    }
}
```
