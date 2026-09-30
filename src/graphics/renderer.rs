use crate::graphics::framebuffer::FrameBuffer;
use crate::graphics::geometry::{Point, Rect, Rotation};
use crate::graphics::text::{FontId, TextStyle};
use crate::hal::display::{EinkDisplay, RefreshRequest};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RenderMode { Monochrome, GrayscaleLsb, GrayscaleMsb }

/// Rendering engine owned by the application runtime. It contains no device-controller logic.
pub struct Renderer<'a> {
    pub frame: FrameBuffer<'a>,
    pub rotation: Rotation,
    pub mode: RenderMode,
    pub clip: Option<Rect>,
}

impl<'a> Renderer<'a> {
    pub fn new(frame: FrameBuffer<'a>) -> Self {
        Self { frame, rotation: Rotation::Portrait, mode: RenderMode::Monochrome, clip: None }
    }

    pub fn set_rotation(&mut self, rotation: Rotation) { self.rotation = rotation; }

    /// TODO: implement the exact logical→physical transforms for all four rotations.
    pub fn rotate_point(&self, point: Point) -> Point {
        let (w, h) = (self.frame.size().width as i32, self.frame.size().height as i32);
        match self.rotation {
            Rotation::Portrait => point,
            Rotation::LandscapeClockwise => Point { x: h - 1 - point.y, y: point.x },
            Rotation::PortraitInverted => Point { x: w - 1 - point.x, y: h - 1 - point.y },
            Rotation::LandscapeCounterClockwise => Point { x: point.y, y: w - 1 - point.x },
        }
    }

    pub fn set_clip(&mut self, rect: Rect) { self.clip = Some(rect); }
    pub fn clear_clip(&mut self) { self.clip = None; }

    /// TODO: implement integer Bresenham drawing.
    pub fn draw_line(&mut self, a: Point, b: Point) {
        let mut x0 = a.x;
        let mut y0 = a.y;
        let x1 = b.x;
        let y1 = b.y;

        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            let p = self.rotate_point(Point { x: x0, y: y0 });
            self.frame.set_pixel(p, crate::graphics::color::Pixel::Black);
            if x0 == x1 && y0 == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    /// TODO: implement scanline polygon fill with bounded temporary storage.
    pub fn fill_polygon(&mut self, points: &[Point]) {
        if points.len() < 3 { return; }
        let mut min_x = points[0].x;
        let mut max_x = points[0].x;
        let mut min_y = points[0].y;
        let mut max_y = points[0].y;
        for p in points {
            if p.x < min_x { min_x = p.x; }
            if p.x > max_x { max_x = p.x; }
            if p.y < min_y { min_y = p.y; }
            if p.y > max_y { max_y = p.y; }
        }
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let mut inside = false;
                let mut j = points.len() - 1;
                for i in 0..points.len() {
                    let pi = points[i];
                    let pj = points[j];
                    if (pi.y > y) != (pj.y > y) {
                        let intersect_x = (pj.x - pi.x) * (y - pi.y) / (pj.y - pi.y) + pi.x;
                        if x < intersect_x {
                            inside = !inside;
                        }
                    }
                    j = i;
                }
                if inside {
                    let p = self.rotate_point(Point { x, y });
                    self.frame.set_pixel(p, crate::graphics::color::Pixel::Black);
                }
            }
        }
    }

    /// TODO: implement circle primitives.
    pub fn fill_circle(&mut self, center: Point, radius: u16) {
        let r = radius as i32;
        let mut x = 0;
        let mut y = r;
        let mut d = 3 - 2 * r;

        let mut draw_horizontal_line = |x_start: i32, x_end: i32, y_pos: i32| {
            let (start, end) = if x_start < x_end { (x_start, x_end) } else { (x_end, x_start) };
            for xi in start..=end {
                let p = self.rotate_point(Point { x: xi, y: y_pos });
                self.frame.set_pixel(p, crate::graphics::color::Pixel::Black);
            }
        };

        while x <= y {
            draw_horizontal_line(center.x - x, center.x + x, center.y + y);
            draw_horizontal_line(center.x - x, center.x + x, center.y - y);
            draw_horizontal_line(center.x - y, center.x + y, center.y + x);
            draw_horizontal_line(center.x - y, center.x + y, center.y - x);

            if d < 0 {
                d += 4 * x + 6;
            } else {
                d += 4 * (x - y) + 10;
                y -= 1;
            }
            x += 1;
        }
    }

    pub fn draw_text(&mut self, provider: &mut dyn crate::graphics::text::FontProvider<Error = crate::CoreError>, font: FontId, origin: Point, text: &str, style: TextStyle) {
        let mut current_x = origin.x;
        let scale_f32 = style.point_size as f32;

        for c in text.chars() {
            if let Ok((glyph_id, _metrics)) = provider.glyph_for_codepoint(font, c as u32) {
                if let Ok(bitmap_data) = provider.rasterize_glyph(font, glyph_id, scale_f32) {
                    if bitmap_data.len() >= 20 {
                        let width = u32::from_le_bytes(bitmap_data[0..4].try_into().unwrap()) as i32;
                        let height = u32::from_le_bytes(bitmap_data[4..8].try_into().unwrap()) as i32;
                        let bearing_x = i32::from_le_bytes(bitmap_data[8..12].try_into().unwrap());
                        let bearing_y = i32::from_le_bytes(bitmap_data[12..16].try_into().unwrap());
                        let adv_width = u32::from_le_bytes(bitmap_data[16..20].try_into().unwrap()) as i32;
                        
                        let pixels = &bitmap_data[20..];
                        
                        for y in 0..height {
                            for x in 0..width {
                                let val = pixels[(y * width + x) as usize];
                                if val > 0 {
                                    let px = current_x + bearing_x + x;
                                    let py = origin.y + bearing_y + y;
                                    let p = self.rotate_point(Point { x: px, y: py });
                                    if self.frame.format() == crate::graphics::framebuffer::PixelFormat::Grayscale8 {
                                        self.frame.set_pixel_value(p, val);
                                    } else {
                                        if val > 128 {
                                            self.frame.set_pixel(p, crate::graphics::color::Pixel::Black);
                                        }
                                    }
                                }
                            }
                        }
                        
                        current_x += adv_width;
                    }
                }
            }
        }
    }

    /// Submits the current frame to a physical display. The display implementation owns
    /// waveform/controller details, while the renderer owns pixel generation.
    pub fn submit<D: EinkDisplay>(&self, display: &mut D, request: RefreshRequest) -> Result<(), D::Error> {
        display.refresh(&self.frame.view(), request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::framebuffer::{FrameBuffer, PixelFormat};
    use crate::graphics::geometry::{Size, Point};

    #[test]
    fn test_renderer_basic() {
        let mut buf = [0xFF; 8];
        let fb = FrameBuffer::new(&mut buf, Size { width: 8, height: 8 }, PixelFormat::Mono1MsbFirst).unwrap();
        let mut renderer = Renderer::new(fb);
        
        // draw a line
        renderer.draw_line(Point { x: 0, y: 0 }, Point { x: 7, y: 7 });
        assert_eq!(renderer.frame.bytes()[0], 0b01111111);
        
        // rotate
        renderer.set_rotation(Rotation::LandscapeClockwise);
        let p = renderer.rotate_point(Point { x: 0, y: 0 });
        assert_eq!(p.x, 7);
        assert_eq!(p.y, 0);
    }
}
