use crate::ui::activity::ActivityId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ListViewport { pub first_index: usize, pub visible_count: usize }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ListState { pub selected_index: usize, pub scroll_offset: usize, pub item_count: usize }

impl ListState {
    pub fn new(item_count: usize) -> Self { Self { selected_index: 0, scroll_offset: 0, item_count } }
}

/// Shared behaviour expected of settings/library/list activities.
pub trait SelectableList {
    fn selected(&self) -> usize;
    fn move_next(&mut self);
    fn move_previous(&mut self);
    fn activate(&mut self) -> Option<ActivityId>;
}
