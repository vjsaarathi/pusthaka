use alloc::string::String;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyboardLayout { Qwerty, Azerty }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyboardLayer { Lower, Upper, Symbols }

pub struct KeyboardState {
    pub layout: KeyboardLayout,
    pub layer: KeyboardLayer,
    pub cursor: usize,
    pub text: String,
}

impl Default for KeyboardState {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyboardState {
    pub fn new() -> Self { Self { layout: KeyboardLayout::Qwerty, layer: KeyboardLayer::Lower, cursor: 0, text: String::new() } }
}
