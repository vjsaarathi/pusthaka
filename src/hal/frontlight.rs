pub trait Frontlight {
    type Error;
    fn set_brightness_percent(&mut self, value: u8) -> Result<(), Self::Error>;
    fn brightness_percent(&self) -> u8;
    fn supports_warmth(&self) -> bool;
    fn set_warmth_percent(&mut self, value: u8) -> Result<(), Self::Error>;
    fn warmth_percent(&self) -> u8;
}
