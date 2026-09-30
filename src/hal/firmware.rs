use crate::services::ota::FirmwareImageSource;

pub trait FirmwareUpdater {
    type Error;

    /// TODO: validate and stage an image for the inactive firmware slot.
    fn stage(&mut self, image: &FirmwareImageSource) -> Result<(), Self::Error>;

    /// TODO: atomically select the staged image as the next boot target.
    fn commit_boot_target(&mut self) -> Result<(), Self::Error>;
}
