# ReaderOS

ReaderOS is a new Rust-native firmware architecture for dedicated e-readers. It is intentionally **not** a line-by-line CrossPoint rewrite.


- The reader core is hardware-independent.
- Device-specific code exists only behind the HAL/device boundary.
- Firmware is designed around `#![no_std]` + `alloc`.
- Host-side simulator/tests use `std`.
- Long reader work is cooperative and resumable rather than one blocking parser task.
- Storage is an explicit service and can behave as disk-backed working memory.
- A single controlled framebuffer is the canonical rendering model.
- Rendering ownership is modelled with Rust borrowing rather than raw pointer lending.
- Hardware capabilities are described by a device contract, not board-name conditionals.
- Core, Extended, and Connected capability profiles are explicit build/runtime concepts.
- The architecture does not use global application singletons.
- Tokio is intentionally excluded from firmware.
- The first concrete hardware target is ESP32-C3, while the core must also run against a host simulator.

## Repository layout

```text
reader-os/
├── src/
│   ├── kernel/       # scheduling, events, time, memory, services
│   ├── hal/          # portable e-reader-specific hardware contracts
│   ├── graphics/     # framebuffer and deterministic rendering contracts
│   ├── reader/       # EPUB/layout/fonts/dictionary/XTC reader core
│   ├── ui/           # navigation, activities, widgets, themes
│   ├── services/     # persistence, library, network, OTA, sync, power
│   ├── formats/      # persistent/on-disk formats such as cpfont and caches
│   └── devices/      # device descriptors and simulator device definitions
├── firmware/         # embedded entry-point/package; hardware wiring belongs here
├── tools/simulator/  # std host executable used as the hardware-independent oracle
└── docs/             # architecture and dependency decisions
```

