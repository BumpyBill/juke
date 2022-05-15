//! 🤖 A small engine for prototyping projects

pub mod gizmos;
pub mod math;

use math::vector::Vector2;
use minifb::{Key, Result, Window, WindowOptions};
use std::f64::consts::PI;

/// A wrapper for window and frame management
///
/// # Examples
///
/// ```
/// let mut engine = Engine::new("Hello, World! - ESC to exit", 320, 180, 3);
/// ```
///
/// # Panics
/// Panics if the initialization of the window fails
pub struct Engine {
    pub window: Window,
    pub buffer: Buffer,
}

impl Engine {
    pub fn new(title: &str, w: usize, h: usize, pixel_size: usize) -> Self {
        Self {
            window: Window::new(
                title,
                w * pixel_size,
                h * pixel_size,
                WindowOptions::default(),
            )
            .unwrap(),
            buffer: Buffer::new(w, h),
        }
    }

    /// The main loop
    ///
    /// # Examples
    ///
    /// ```
    /// Engine::new("Hello, World! - ESC to exit", 320, 180, 3).run(|e: &mut Engine| {
    ///    // ...
    //     Ok(())
    ///});
    /// ```
    ///
    /// # Errors
    /// Returns an error if the closure returns one or there was an issue updating the frame buffer
    pub fn run<F: Fn(&mut Engine) -> Result<()>>(&mut self, u: F) -> Result<()> {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            u(self)?;

            self.window
                .update_with_buffer(self.buffer.raw(), self.buffer.w, self.buffer.h)?;
        }
        Ok(())
    }
}

pub struct Buffer {
    pub w: usize,
    pub h: usize,

    val: Vec<u32>,
}

impl Buffer {
    pub fn new(w: usize, h: usize) -> Self {
        Buffer {
            w,
            h,
            val: vec![0; w * h],
        }
    }

    pub fn raw(&self) -> &[u32] {
        self.val.as_slice()
    }

    pub fn raw_mut(&mut self) -> &mut [u32] {
        self.val.as_mut_slice()
    }
}

impl std::ops::Index<usize> for Buffer {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.val[index]
    }
}

impl std::ops::IndexMut<usize> for Buffer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.val[index]
    }
}

/// RGB color.
///
/// # Examples
///
/// ```
/// Color(255, 0, 255)
/// ```
#[derive(Clone)]
pub struct Color(pub u8, pub u8, pub u8);

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        let rgb = color.0 as u32;
        let rgb = (rgb << 8) + color.1 as u32;
        (rgb << 8) + color.2 as u32
    }
}
