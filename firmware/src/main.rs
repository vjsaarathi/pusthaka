#![cfg_attr(target_arch = "riscv32", no_std)]
#![cfg_attr(target_arch = "riscv32", no_main)]

//! Embedded entry-point scaffold.
//!
//! This package deliberately contains only the first hardware wiring boundary. The actual
//! device adapter belongs here (or under a future device-specific package), while the reader
//! library remains completely ESP-independent.

#[cfg(target_arch = "riscv32")]
mod esp32c3 {
    /// TODO: initialize clocks, heap, GPIO, SPI/I²C, concrete e-ink controller, storage,
    /// input and power services; construct the first `reader_os::kernel::System`.
    pub fn boot() -> ! {
        loop { core::hint::spin_loop(); }
    }
}

#[cfg(target_arch = "riscv32")]
#[esp_hal::main]
fn main() -> ! {
    esp32c3::boot()
}

#[cfg(not(target_arch = "riscv32"))]
fn main() {
    // Host compilation intentionally succeeds; the real target entry point is selected only
    // when the ESP32-C3 target is explicitly requested.
}
