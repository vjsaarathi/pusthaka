#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Pixel { Black, White }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GrayscaleLevel {
    Black,
    DarkGray,
    LightGray,
    White,
}

impl GrayscaleLevel {
    pub const fn ordinal(self) -> u8 {
        match self { Self::Black => 0, Self::DarkGray => 1, Self::LightGray => 2, Self::White => 3 }
    }
}
