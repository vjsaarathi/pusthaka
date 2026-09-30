//! Hardware-independent runtime primitives.

pub mod boot;
pub mod events;
pub mod memory;
pub mod scheduler;
pub mod services;
pub mod time;

pub use boot::{BootManager, BootResume, BootTarget, RecoveryMode, SilentRebootRequest, WakeReason};
pub use events::{Event, EventQueue};
pub use scheduler::{PollResult, Service, WorkBudget};
pub use time::{Duration, MonotonicInstant, TimeSource};
