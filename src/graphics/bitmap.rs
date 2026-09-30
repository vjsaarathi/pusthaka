use super::dithering::DitherAlgorithm;
use super::framebuffer::FrameBuffer;
use super::geometry::Rect;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImageFormat { Bmp, Png, Jpeg }

/// Streaming image decoder boundary. Decoders must emit rows/tiles rather than requiring
/// the entire source image and decoded image to coexist in RAM.
pub trait ImageDecoder {
    type Error;
    fn decode_into(&mut self, target: &mut FrameBuffer<'_>, dst: Rect, dither: DitherAlgorithm) -> Result<(), Self::Error>;
}

pub struct BitmapMetadata {
    pub width: u32,
    pub height: u32,
    pub bits_per_pixel: u16,
}

/// TODO: preserve support for top-down BMPs and row padding while keeping reads streaming.
pub trait BitmapReader {
    type Error;
    fn metadata(&mut self) -> Result<BitmapMetadata, Self::Error>;
}
