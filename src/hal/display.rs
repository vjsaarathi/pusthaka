use crate::graphics::framebuffer::FrameBufferView;

/// Display features that may be selected at runtime.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DisplayCapabilities {
    pub grayscale_levels: u8,
    pub partial_refresh: bool,
    pub asynchronous_refresh: bool,
    pub waveform_control: bool,
}

/// Supported refresh qualities. Exact electrical waveforms belong in device drivers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RefreshMode {
    Fast,
    Half,
    Full,
}

/// Describes one display update operation without exposing controller-specific commands.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RefreshRequest {
    pub mode: RefreshMode,
    pub grayscale: bool,
    pub asynchronous: bool,
}

/// The e-ink contract shared by all qualifying devices.
pub trait EinkDisplay {
    type Error;

    fn capabilities(&self) -> DisplayCapabilities;

    /// Submits a complete framebuffer view to the physical panel.
    ///
    /// TODO: device implementations must translate the generic request to the
    /// controller waveform/update protocol, including any required grayscale
    /// preconditioning and board-specific resynchronization.
    fn refresh(&mut self, frame: &FrameBufferView<'_>, request: RefreshRequest) -> Result<(), Self::Error>;

    fn is_busy(&self) -> bool;

    /// TODO: put the panel into the minimum safe low-power state supported by the device.
    fn sleep(&mut self) -> Result<(), Self::Error>;
}
