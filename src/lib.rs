#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod devices;
pub mod formats;
pub mod graphics;
pub mod hal;
pub mod kernel;
pub mod reader;
pub mod services;

mod error;
pub use error::{CoreError, Result};
