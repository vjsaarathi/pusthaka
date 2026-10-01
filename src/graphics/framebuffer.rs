use super::color::Pixel;
use super::geometry::{Point, Size};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PixelFormat { Mono1MsbFirst, Grayscale8 }

/// Borrowed framebuffer view. The backing allocation is owned by the runtime/device
/// layer; renderers never allocate an independent full-screen buffer behind the scenes.
pub struct FrameBuffer<'a> {
    bytes: &'a mut [u8],
    size: Size,
    format: PixelFormat,
}

#[derive(Clone, Copy)]
pub struct FrameBufferView<'a> {
    pub bytes: &'a [u8],
    pub size: Size,
    pub format: PixelFormat,
}

impl<'a> FrameBuffer<'a> {
    pub fn new(bytes: &'a mut [u8], size: Size, format: PixelFormat) -> crate::Result<Self> {
        let required = match format {
            PixelFormat::Mono1MsbFirst => (size.width as usize * size.height as usize).div_ceil(8),
            PixelFormat::Grayscale8 => size.width as usize * size.height as usize,
        };
        if bytes.len() < required { return Err(crate::CoreError::InvalidInput); }
        Ok(Self { bytes, size, format })
    }

    pub fn view(&self) -> FrameBufferView<'_> {
        FrameBufferView { bytes: self.bytes, size: self.size, format: self.format }
    }

    pub fn size(&self) -> Size { self.size }
    
    pub fn format(&self) -> PixelFormat { self.format }

    pub fn clear(&mut self, pixel: Pixel) {
        let value = match (self.format, pixel) {
            (PixelFormat::Mono1MsbFirst, Pixel::Black) => 0x00,
            (PixelFormat::Mono1MsbFirst, Pixel::White) => 0xFF,
            (PixelFormat::Grayscale8, Pixel::Black) => 255,
            (PixelFormat::Grayscale8, Pixel::White) => 0,
        };
        self.bytes.fill(value);
    }

    /// TODO: preserve the MSB-first bit convention and add clipping/orientation-free pixel access.
    pub fn set_pixel(&mut self, point: Point, pixel: Pixel) {
        if point.x < 0 || point.y < 0 || point.x >= self.size.width as i32 || point.y >= self.size.height as i32 {
            return;
        }
        match self.format {
            PixelFormat::Mono1MsbFirst => {
                let index = (point.y as usize * self.size.width as usize + point.x as usize) / 8;
                let bit = 7 - ((point.x as usize) % 8);
                match pixel {
                    Pixel::Black => self.bytes[index] &= !(1 << bit),
                    Pixel::White => self.bytes[index] |= 1 << bit,
                }
            }
            PixelFormat::Grayscale8 => {
                let index = point.y as usize * self.size.width as usize + point.x as usize;
                match pixel {
                    Pixel::Black => self.bytes[index] = 255,
                    Pixel::White => self.bytes[index] = 0,
                }
            }
        }
    }

    pub fn blend_pixel(&mut self, point: Point, coverage: u8, target: Pixel) {
        if point.x < 0 || point.y < 0 || point.x >= self.size.width as i32 || point.y >= self.size.height as i32 {
            return;
        }
        match self.format {
            PixelFormat::Mono1MsbFirst => {
                let index = (point.y as usize * self.size.width as usize + point.x as usize) / 8;
                let bit = 7 - ((point.x as usize) % 8);
                if coverage > 128 {
                    match target {
                        Pixel::Black => self.bytes[index] &= !(1 << bit),
                        Pixel::White => self.bytes[index] |= 1 << bit,
                    }
                }
            }
            PixelFormat::Grayscale8 => {
                let index = point.y as usize * self.size.width as usize + point.x as usize;
                let existing = self.bytes[index] as u16;
                let alpha = coverage as u16;
                let target_val = match target { Pixel::Black => 255, Pixel::White => 0 };
                let new_val = (alpha * target_val + (255 - alpha) * existing) / 255;
                self.bytes[index] = new_val as u8;
            }
        }
    }

    pub fn bytes(&self) -> &[u8] { self.bytes }
    pub fn bytes_mut(&mut self) -> &mut [u8] { self.bytes }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_pixel() {
        let mut buf = [0xFF; 8];
        let mut fb = FrameBuffer::new(&mut buf, Size { width: 8, height: 8 }, PixelFormat::Mono1MsbFirst).unwrap();
        fb.set_pixel(Point { x: 0, y: 0 }, Pixel::Black);
        assert_eq!(fb.bytes()[0], 0b01111111);
        fb.set_pixel(Point { x: 7, y: 0 }, Pixel::Black);
        assert_eq!(fb.bytes()[0], 0b01111110);
    }
}
