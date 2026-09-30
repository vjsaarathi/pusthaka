/// Explicit work budget supplied to cooperative services.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorkBudget {
    pub max_steps: u32,
    pub max_millis: u32,
}

impl WorkBudget {
    pub const fn steps(max_steps: u32) -> Self {
        Self { max_steps, max_millis: u32::MAX }
    }

    pub const fn millis(max_millis: u32) -> Self {
        Self { max_steps: u32::MAX, max_millis }
    }

    pub const fn bounded(max_steps: u32, max_millis: u32) -> Self {
        Self { max_steps, max_millis }
    }
}

/// Result returned by a resumable service after one cooperative slice.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PollResult {
    /// More work remains; the scheduler may call the service again later.
    Pending,
    /// The operation completed.
    Complete,
    /// The service cannot progress until an external dependency changes.
    Waiting,
}

/// Common contract for expensive state machines.
pub trait Service {
    type Error;
    fn poll(&mut self, budget: WorkBudget) -> Result<PollResult, Self::Error>;
}
