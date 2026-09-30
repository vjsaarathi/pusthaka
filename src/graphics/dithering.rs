/// Algorithms supported by the ReaderOS image pipeline.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DitherAlgorithm { None, Atkinson, FloydSteinberg, Bayer4x4 }

/// Canonical 4×4 ordered-dither matrix used for deterministic overlays.
pub const BAYER_4X4: [[u8; 4]; 4] = [
    [0, 8, 2, 10],
    [12, 4, 14, 6],
    [3, 11, 1, 9],
    [15, 7, 13, 5],
];

/// TODO: implement bounded-row Atkinson error diffusion.
pub fn atkinson(pixels: &mut [u8], width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let old_pixel = pixels[idx];
            let new_pixel = if old_pixel < 128 { 0 } else { 255 };
            pixels[idx] = new_pixel;
            let err = old_pixel as i16 - new_pixel as i16;
            let err_eighth = err / 8;

            let mut add_err = |x_off: isize, y_off: isize| {
                let nx = x as isize + x_off;
                let ny = y as isize + y_off;
                if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                    let nidx = ny as usize * width + nx as usize;
                    let p = pixels[nidx] as i16;
                    pixels[nidx] = (p + err_eighth).clamp(0, 255) as u8;
                }
            };

            add_err(1, 0);
            add_err(2, 0);
            add_err(-1, 1);
            add_err(0, 1);
            add_err(1, 1);
            add_err(0, 2);
        }
    }
}

/// TODO: implement bounded-row Floyd-Steinberg error diffusion.
pub fn floyd_steinberg(pixels: &mut [u8], width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let old_pixel = pixels[idx];
            let new_pixel = if old_pixel < 128 { 0 } else { 255 };
            pixels[idx] = new_pixel;
            let err = old_pixel as i16 - new_pixel as i16;

            let mut add_err = |x_off: isize, y_off: isize, weight: i16| {
                let nx = x as isize + x_off;
                let ny = y as isize + y_off;
                if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                    let nidx = ny as usize * width + nx as usize;
                    let p = pixels[nidx] as i16;
                    pixels[nidx] = (p + (err * weight) / 16).clamp(0, 255) as u8;
                }
            };

            add_err(1, 0, 7);
            add_err(-1, 1, 3);
            add_err(0, 1, 5);
            add_err(1, 1, 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atkinson() {
        let mut pixels = [128; 4];
        atkinson(&mut pixels, 2, 2);
        assert_eq!(pixels[0], 255);
    }

    #[test]
    fn test_floyd_steinberg() {
        let mut pixels = [128; 4];
        floyd_steinberg(&mut pixels, 2, 2);
        assert_eq!(pixels[0], 255);
    }
}
