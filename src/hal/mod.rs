//! Hardware contracts for e-reader devices.
//!
//! This module defines the boundary between the portable operating environment and
//! concrete devices. Nothing in the reader core should know about ESP32, GPIO
//! numbers, vendor display controllers, or board names.

pub mod battery;
pub mod capabilities;
pub mod clock;
pub mod device;
pub mod display;
pub mod firmware;
pub mod frontlight;
pub mod input;
pub mod network;
pub mod power;
pub mod storage;
pub mod touch;

pub use capabilities::{Capability, CapabilitySet, DeviceCapabilities, DeviceDescriptor, DisplayDescriptor, FeatureProfile, InputDescriptor, MemoryProfile, StorageDescriptor};
pub use device::{Device, DeviceServices};
pub use display::{DisplayCapabilities, EinkDisplay, RefreshMode, RefreshRequest};
