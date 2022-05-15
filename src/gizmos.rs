use crate::*;

/// Draws a circle onto the frame buffer.
///
/// # Examples
///
/// ```
/// gizmos::circle(Color(255, 0, 255), &pos, 50, &mut e.buffer);
/// ```
pub fn circle(color: Color, pos: &Vector2, radius: u32, buffer: &mut Buffer) {
    for angle in 0..360 {
        let angle = angle as f64 * PI / 180f64;

        let x = radius as f64 * angle.cos();
        let y = radius as f64 * angle.sin();

        let x = x as f32 + pos.x;
        let y = y as f32 + pos.y;

        pixel(color.clone(), Vector2::new(x, y), buffer);
    }
}

/// Draws a line onto the frame buffer.
///
/// # Examples
///
/// ```
/// gizmos::line(Color(255, 0, 0), &p1, &p2, &mut e.buffer);
/// ```
pub fn line(color: Color, p1: &Vector2, p2: &Vector2, buffer: &mut Buffer) {
    let d = p2.to_owned() - p1.to_owned();
    let step = if d.x.abs() >= d.y.abs() {
        d.x.abs()
    } else {
        d.y.abs()
    };
    let mut x = p1.x;
    let mut y = p1.y;
    let d = Vector2::new(d.x / step, d.y / step);

    for _ in 0..step.ceil() as i32 {
        pixel(color.clone(), Vector2::new(x, y), buffer);
        x += d.x;
        y += d.y;
    }
}

/// Draws a single pixel onto the frame buffer.
///
/// # Examples
///
/// ```
/// gizmos::pixel(Color(255, 255, 255), &pos, &mut e.buffer);
/// ```
pub fn pixel(color: Color, pos: Vector2, buffer: &mut Buffer) {
    let w = buffer.w;
    buffer[pos.y as usize * w + pos.x as usize] = color.into();
}
