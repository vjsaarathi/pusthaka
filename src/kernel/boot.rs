use alloc::string::String;

/// Cold/wake/restart routing state used before the first activity is selected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BootResume { Splash, SplashlessWake, Silent }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WakeReason { PowerButton, Timer, Other, Unknown }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecoveryMode { None, FirmwareUpdate }

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BootTarget { Home, Reader, Settings, CrashReport, Recovery }

/// RTC/no-init style restart information without exposing a specific MCU mechanism.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SilentRebootRequest {
    pub target: BootTarget,
    pub payload: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BootFailure { StorageUnavailable, InvalidDevice, InitializationFailed }

/// Hardware-neutral boot decision state machine.
pub struct BootManager {
    pub resume: BootResume,
    pub wake_reason: WakeReason,
    pub recovery: RecoveryMode,
    pub previous_panic: bool,
}

impl BootManager {
    pub fn target(&self, has_open_book: bool, crash_loop: bool) -> BootTarget {
        if self.recovery == RecoveryMode::FirmwareUpdate { BootTarget::Recovery }
        else if self.previous_panic { BootTarget::CrashReport }
        else if crash_loop { BootTarget::Home }
        else if has_open_book || self.resume == BootResume::Silent { BootTarget::Reader }
        else { BootTarget::Home }
    }
}
