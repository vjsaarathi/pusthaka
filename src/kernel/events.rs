use alloc::collections::VecDeque;

use crate::hal::input::InputEvent;
use crate::services::networking::NetworkEvent;

/// Application-wide event vocabulary. Events intentionally carry stable data rather
/// than references into transient driver buffers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    Input(InputEvent),
    Network(NetworkEvent),
    Timer(TimerEvent),
    Storage(StorageEvent),
    Power(PowerEvent),
    RenderRequested,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimerEvent {
    Tick,
    SleepTimeout,
    PeriodicRefresh,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StorageEvent {
    Mounted,
    Unavailable,
    Changed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PowerEvent {
    Wake,
    SleepRequested,
    ResetRequested,
}

/// Heap-backed queue used by the host implementation first. A firmware backend may
/// later replace this with a fixed-capacity queue without changing event semantics.
pub struct EventQueue {
    queue: VecDeque<Event>,
    capacity: usize,
}

impl EventQueue {
    pub fn new(capacity: usize) -> Self {
        Self { queue: VecDeque::new(), capacity }
    }

    pub fn push(&mut self, event: Event) -> Result<(), Event> {
        if self.queue.len() >= self.capacity {
            return Err(event);
        }
        self.queue.push_back(event);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<Event> { self.queue.pop_front() }
    pub fn is_empty(&self) -> bool { self.queue.is_empty() }
}
