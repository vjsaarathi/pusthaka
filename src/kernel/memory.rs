/// Memory-pressure classification used by cache managers and background work.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MemoryPressure {
    Normal,
    Constrained,
    Critical,
}

/// Runtime memory budget supplied to components that may allocate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryBudget {
    pub total_bytes: usize,
    pub available_bytes: usize,
    pub reserved_bytes: usize,
}

impl MemoryBudget {
    pub fn pressure(&self) -> MemoryPressure {
        if self.available_bytes <= self.reserved_bytes {
            MemoryPressure::Critical
        } else if self.available_bytes < self.total_bytes / 10 {
            MemoryPressure::Constrained
        } else {
            MemoryPressure::Normal
        }
    }
}

/// TODO: implement bounded allocation helpers for firmware paths. The implementation
/// must fail cleanly instead of turning allocation failure into an uncontrolled panic.
pub fn try_allocate<T>(_value: T) -> crate::Result<T> {
    Ok(_value)
}
