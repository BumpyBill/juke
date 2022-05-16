<p>
  <a href="https://crates.io/crates/juke" target="_blank">
    <img alt="Version" src="https://img.shields.io/crates/v/juke">
  </a>
  <a href="https://docs.rs/juke" target="_blank">
    <img alt="Documentation" src="https://img.shields.io/docsrs/juke" />
  </a>
</p>

> 🤖 A small engine for prototyping projects

## Usage

```toml
# Cargo.toml
[dependencies]
juke = "0.0.11"
```


## Example
```rs
use juke::{
    gizmos,
    math::{u32::UVec2},
    *,
};

fn main() {
    let res = Engine::new("Hello, World! - ESC to exit", 256, 144, 3).run(|e: &mut Engine| {
        let pos = UVec2::new(e.buffer.w as u32 / 2, e.buffer.h as u32 / 2);
        gizmos::circle(Color(255, 0, 255), &pos.as_vec2(), 50, &mut e.buffer);

        Ok(())
    });

    match res {
        Ok(()) => println!("exited succesfully!"),
        Err(e) => panic!("{}", e),
    }
}

```
