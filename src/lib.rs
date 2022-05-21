//! 🤖 A small engine for prototyping projects

pub use glam as math;

use minifb::{Key, Result, Window, WindowOptions};
use std::time::{Duration, Instant};

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

    pub frame_count: u128,
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
            frame_count: 0,
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
    pub fn run<F: Fn(&mut Engine, Duration) -> Result<()>>(&mut self, u: F) -> Result<()> {
        let mut prev_time = Instant::now();
        let target_rate = Duration::from_micros(16666);

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            let delta = {
                let real_delta = prev_time.elapsed();
                if real_delta < target_rate {
                    let sleep_time = target_rate - real_delta;
                    std::thread::sleep(sleep_time);

                    real_delta + target_rate
                } else {
                    real_delta
                }
            };

            self.window
                .update_with_buffer(self.buffer.raw(), self.buffer.w, self.buffer.h)?;

            prev_time = Instant::now();
            self.frame_count += 1;

            u(self, delta)?;
        }
        Ok(())
    }
}

pub struct Buffer<const LENGTH: usize> {
    pub w: usize,
    pub h: usize,

    pub val: [u32; LENGTH],
}

impl<const LENGTH: usize> Buffer<LENGTH> {
    pub fn new(w: usize, h: usize) -> Self {
        Buffer {
            w,
            h,
            val: [0; LENGTH],
        }
    }

    pub fn raw(&self) -> &[u32; LENGTH] {
        &self.val
    }

    pub fn raw_mut(&mut self) -> &mut [u32; LENGTH] {
        &mut self.val
    }

    pub fn clear(&mut self, val: u32) {
        self.val = [1; LENGTH]
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
        let (r, g, b) = (color.0 as u32, color.1 as u32, color.2 as u32);
        (r << 16) | (g << 8) | b
    }
}
