#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SleepMode { Dark, Custom, Cover, CoverCustom, Default }

pub struct SleepScreenState {
    pub mode: SleepMode,
    pub recent_image_ids: alloc::vec::Vec<u16>,
    pub recent_overlay_ids: alloc::vec::Vec<u16>,
}

impl SleepScreenState {
    /// TODO: choose a non-repeating sleep image while avoiding unbounded allocations.
    pub fn choose_image(&mut self) -> Option<u16> {
        let max_history = 5;
        let mut next_id = 1;
        while self.recent_image_ids.contains(&next_id) {
            next_id += 1;
            if next_id > 10 { break; }
        }
        self.recent_image_ids.push(next_id);
        if self.recent_image_ids.len() > max_history {
            self.recent_image_ids.remove(0);
        }
        Some(next_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choose_image() {
        let mut state = SleepScreenState {
            mode: SleepMode::Default,
            recent_image_ids: alloc::vec::Vec::new(),
            recent_overlay_ids: alloc::vec::Vec::new(),
        };
        let id1 = state.choose_image();
        let id2 = state.choose_image();
        assert!(id1 != id2);
    }
}
