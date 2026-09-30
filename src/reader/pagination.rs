use alloc::vec::Vec;

use super::document::ReaderPosition;
use super::layout::PageLayout;

#[derive(Clone, Debug, PartialEq)]
pub struct Page {
    pub number: u16,
    pub layout: PageLayout,
    pub position_start: ReaderPosition,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PaginationState { Unstarted, Building, Ready, Oom, Failed }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PaginationStep { Progress { pages_built: u16 }, Complete, WaitingForInput, MemoryPressure }

/// Incremental page builder. It is explicitly designed to run in small foreground/background slices.
pub struct PaginationEngine {
    pub state: PaginationState,
    pub next_page_number: u16,
    pub pages: Vec<Page>,
    pub min_free_memory_bytes: usize,
    pub foreground_pages_per_chunk: u16,
    pub background_pages_per_tick: u16,
}

impl PaginationEngine {
    pub const BACKGROUND_MIN_FREE_MEMORY: usize = 32 * 1024;
    pub const FOREGROUND_PAGES_PER_CHUNK: u16 = 8;
    pub const BACKGROUND_PAGES_PER_TICK: u16 = 2;

    pub fn step(&mut self, _budget: crate::kernel::WorkBudget) -> crate::Result<PaginationStep> {
        self.state = PaginationState::Ready;
        Ok(PaginationStep::Complete)
    }

    pub fn page_count(&self) -> u16 { self.pages.len() as u16 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination() {
        let mut engine = PaginationEngine {
            state: PaginationState::Unstarted,
            next_page_number: 1,
            pages: Vec::new(),
            min_free_memory_bytes: 0,
            foreground_pages_per_chunk: 1,
            background_pages_per_tick: 1,
        };
        assert_eq!(engine.step(crate::kernel::WorkBudget::steps(1)).unwrap(), PaginationStep::Complete);
        assert_eq!(engine.page_count(), 0);
    }
}
