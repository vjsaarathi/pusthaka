/// Hardware power-management contract.
pub trait PowerController {
    type Error;
    fn hold_power_rails(&mut self) -> Result<(), Self::Error>;
    fn enter_sleep(&mut self) -> Result<(), Self::Error>;
    fn reset(&mut self) -> Result<(), Self::Error>;
    fn prevent_sleep(&mut self, prevent: bool) -> Result<(), Self::Error>;
}
