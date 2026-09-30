#![no_std]

extern crate alloc;

pub mod devices;
pub mod formats;
pub mod graphics;
pub mod hal;
pub mod kernel;
pub mod reader;
pub mod services;
pub mod ui;

mod error;
pub use error::{CoreError, Result};
