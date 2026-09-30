#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point { pub x: i32, pub y: i32 }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Size { pub width: u16, pub height: u16 }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rect { pub x: i32, pub y: i32, pub width: u16, pub height: u16 }

impl Rect {
    pub const fn right(self) -> i32 { self.x + self.width as i32 }
    pub const fn bottom(self) -> i32 { self.y + self.height as i32 }
}

/// Logical-to-physical display orientation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rotation { Portrait, LandscapeClockwise, PortraitInverted, LandscapeCounterClockwise }
