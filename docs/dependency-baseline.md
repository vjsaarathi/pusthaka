# Dependency Baseline

Verified against current package metadata on 2026-09-30.

| Component | Baseline | Role | Status |
|---|---|---|---|
| `esp-hal` | 1.2.2 | Bare-metal ESP32 HAL | Selected for the first ESP32-C3 target |
| `esp-alloc` | 0.11.0 | Espressif no_std heap allocator | Selected for firmware bring-up |
| `esp-backtrace` | 0.20.0 | panic/backtrace support | Selected for firmware bring-up |
| `embassy-executor` | 0.10.0 | possible embedded async executor backend | Allowed, not leaked into core |
| `embassy-time` | 0.5.1 | embedded monotonic time/async timers | Allowed, not leaked into core |
| `esp-radio` | 0.18.0 stable line | Wi-Fi/BLE/ESP-NOW | Deferred until Connected profile implementation; do not make it a core dependency |

The current ESP HAL is explicitly a `no_std` HAL for ESP32 devices and exposes an ESP32-C3 feature. The selected `esp-radio` stable line has a compatibility relationship with the ESP HAL and should therefore be added only after the network stack is implemented and version-pinned as a tested pair.
