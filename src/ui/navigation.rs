use alloc::boxed::Box;

use super::activity::{Activity, ActivityContext, ActivityId, ActivityResult, ActivityResultHandler};

/// A deferred ownership-moving navigation request.
///
/// This type deliberately does not implement `Copy`, `Clone`, `Debug`, or equality
/// because its `Box<dyn Activity>` payload is an owned trait object.
pub enum PendingTransition { Push(Box<dyn Activity>), Pop, Replace(Box<dyn Activity>) }

/// Stack-based activity controller with deferred transitions.
pub struct ActivityManager {
    pub stack: alloc::vec::Vec<Box<dyn Activity>>,
    pub current: Option<Box<dyn Activity>>,
    pub pending: Option<PendingTransition>,
    pub result_handler: Option<ActivityResultHandler>,
    pub update_requested: bool,
}

impl Default for ActivityManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ActivityManager {
    pub fn new() -> Self {
        Self { stack: alloc::vec::Vec::new(), current: None, pending: None, result_handler: None, update_requested: false }
    }

    /// TODO: apply Push/Pop/Replace only at a safe scheduler boundary, preserving lifecycle ordering.
    pub fn apply_pending(&mut self, ctx: &mut ActivityContext<'_, '_>) -> crate::Result<()> {
        if let Some(pending) = self.pending.take() {
            match pending {
                PendingTransition::Push(mut new_act) => {
                    if let Some(mut curr) = self.current.take() {
                        curr.on_exit(ctx)?;
                        self.stack.push(curr);
                    }
                    new_act.on_enter(ctx)?;
                    self.current = Some(new_act);
                }
                PendingTransition::Pop => {
                    if let Some(mut curr) = self.current.take() {
                        curr.on_exit(ctx)?;
                    }
                    if let Some(mut prev) = self.stack.pop() {
                        prev.on_enter(ctx)?;
                        self.current = Some(prev);
                    }
                }
                PendingTransition::Replace(mut new_act) => {
                    if let Some(mut curr) = self.current.take() {
                        curr.on_exit(ctx)?;
                    }
                    new_act.on_enter(ctx)?;
                    self.current = Some(new_act);
                }
            }
            self.request_render();
        }
        Ok(())
    }

    pub fn request_render(&mut self) { self.update_requested = true; }

    pub fn current_id(&self) -> Option<ActivityId> { self.current.as_ref().map(|a| a.id()) }

    /// TODO: implement parent-child result propagation.
    pub fn finish_current(&mut self, result: ActivityResult) {
        self.pending = Some(PendingTransition::Pop);
        if let Some(mut handler) = self.result_handler.take() {
            handler(result);
        }
    }

    /// TODO: these helpers should instantiate the application-specific activity types.
    pub fn go_home(&mut self) {
        self.stack.clear();
        self.pending = Some(PendingTransition::Replace(Box::new(crate::ui::activities::HomeActivity)));
    }
    pub fn go_to_reader(&mut self) {
        self.stack.clear();
        self.pending = Some(PendingTransition::Replace(Box::new(crate::ui::activities::EpubReaderActivity)));
    }
    pub fn go_to_settings(&mut self) {
        self.pending = Some(PendingTransition::Push(Box::new(crate::ui::activities::SettingsActivity)));
    }
}
