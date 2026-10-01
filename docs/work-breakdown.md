# Implementation Order

This document records the implementation order implied by the scaffold.

## Phase 0 — Contract and host oracle

Implement the capability model, device descriptors, host virtual device, framebuffer, event model, scheduler budgets, and renderer test surface.

Exit criterion: a host program can create a virtual e-reader and render deterministic pages without any ESP dependency.

## Phase 1 — ESP32-C3 bring-up

Implement the concrete device package, allocator, boot entry point, GPIO, SPI, display controller, storage, power and monotonic clock.

Exit criterion: ESP32-C3 renders a framebuffer supplied by the hardware-independent renderer and accepts at least one input source.

## Phase 2 — Graphics/text

Implement geometry/orientation, monochrome framebuffer operations, dithering, bitmap decoding, fonts, kerning and fixed-point text placement.

## Phase 3 — Reader core

Implement streaming ZIP, XML/HTML/CSS parsers, block model, text storage, pagination, caching, hyphenation, image conversion and XTC.

## Phase 4 — UI/services

Implement activities/navigation, library, persistence, bookmarks, settings, sleep and dictionary services.

## Phase 5 — Connected profile

Implement Wi-Fi, HTTP/WebSocket server, WebDAV, OPDS, OTA, sync and QR helpers without allowing these concerns into the reader core.

## Phase 6 — Compatibility certification

Create golden/differential fixtures and require structural/byte/pixel compatibility where the chosen format/protocol contract demands it.
