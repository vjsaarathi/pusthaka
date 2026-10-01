use crate::kernel::time::MonotonicInstant;

/// Physical input state supplied by a device driver.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RawButtonState {
    pub index: u8,
    pub pressed: bool,
}

/// Physical touch sample supplied by a device driver.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RawTouchSample {
    pub x: i16,
    pub y: i16,
    pub pressed: bool,
}

/// Low-level input source contract.
pub trait InputSource {
    type Error;
    fn poll(&mut self, now: MonotonicInstant) -> Result<(), Self::Error>;
    fn button_state(&self, index: u8) -> bool;
}

/// Logical controls consumed by UI and reader state machines.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LogicalButton {
    Back,
    Confirm,
    Left,
    Right,
    Up,
    Down,
    Power,
    PageBack,
    PageForward,
    NavNext,
    NavPrevious,
    ScreenLeft,
    ScreenRight,
    ScreenUp,
    ScreenDown,
}

/// Derived input events after debounce/remapping/orientation processing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputEvent {
    ButtonPressed(LogicalButton),
    ButtonReleased(LogicalButton),
    Tap { x: i16, y: i16 },
    LongPress { x: i16, y: i16 },
    Drag { x: i16, y: i16, dx: i16, dy: i16 },
    EdgeSwipe { edge: Edge, inward_dx: i16, inward_dy: i16 },
    Character(char),
    Scroll { x: i16, y: i16, dx: i16, dy: i16 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Edge { Top, Right, Bottom, Left }

/// Stateful mapper responsible for debounce, physical remapping, touch gestures and
/// orientation-aware navigation semantics.
pub struct InputMapper {
    pub debounce_ms: u32,
    pub front_button_back: u8,
    pub front_button_confirm: u8,
    pub front_button_left: u8,
    pub front_button_right: u8,
    pub side_button_page_forward: bool,
    pub long_press_ms: u32,
}

impl Default for InputMapper {
    fn default() -> Self {
        Self {
            debounce_ms: 15,
            front_button_back: 0,
            front_button_confirm: 1,
            front_button_left: 2,
            front_button_right: 3,
            side_button_page_forward: true,
            long_press_ms: 750,
        }
    }
}

impl InputMapper {
    /// TODO: consume the raw device state, emit debounced logical events, and ensure
    /// that each one-shot event is consumed at most once by the activity loop.
    pub fn update(&mut self, _now: MonotonicInstant) -> crate::Result<()> {
        Ok(())
    }
}
