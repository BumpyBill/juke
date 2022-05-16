use crate::*;
use glam::u32::UVec2;

/// Draws a circle onto the frame buffer.
///
/// # Examples
///
/// ```
/// gizmos::circle(Color(255, 0, 255), &pos, 50, &mut e.buffer);
/// ```
pub fn circle(color: Color, pos: &UVec2, radius: u32, buffer: &mut Buffer) {
    for angle in 0..360 {
        let angle = angle as f32 * PI / 180f32;

        let x = radius as f32 * angle.cos();
        let y = radius as f32 * angle.sin();

        let x = x + pos.x as f32;
        let y = y + pos.y as f32;

        pixel(color.clone(), UVec2::new(x as u32, y as u32), buffer);
    }
}

/// Draws a line onto the frame buffer.
///
/// # Examples
///
/// ```
/// gizmos::line(Color(255, 0, 0), &p1, &p2, &mut e.buffer);
/// ```
pub fn line(color: Color, p1: &UVec2, p2: &UVec2, buffer: &mut Buffer) {
    let d = p2.as_vec2() - p1.as_vec2();
    let step = if d.x.abs() >= d.y.abs() {
        d.x.abs()
    } else {
        d.y.abs()
    };
    let mut p = p1.to_owned();
    let d = (d / step).as_uvec2();

    for _ in 0..step.ceil() as i32 {
        pixel(color.clone(), p, buffer);
        p += d;
    }
}

/// Draws a single pixel onto the frame buffer.
///
/// # Examples
///
/// ```
/// gizmos::pixel(Color(255, 255, 255), &pos, &mut e.buffer);
/// ```
pub fn pixel(color: Color, pos: UVec2, buffer: &mut Buffer) {
    let w = buffer.w;
    buffer[pos.y as usize * w + pos.x as usize] = color.into();
}
