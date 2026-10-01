# ReaderOS Architectural Laws

These are design laws for the implementation, not suggestions.

## 1. Hardware independence

`src/reader`, `src/graphics`, `src/ui`, and the non-device portions of `src/services` must not depend on ESP32-specific types.

The dependency direction is:

```text
Device implementation
        ↓
       HAL
        ↓
     Services
        ↓
   Reader / UI
```

Hardware-specific behaviour flows downward only.

## 2. Device capability contract

The firmware selects behaviour from `CapabilitySet` and descriptors. It must not accumulate board-name checks such as `if x4_pro` inside reader or UI code.

## 3. no_std firmware + std tooling

Firmware-facing library code is `no_std` and may use `alloc` where required. Host simulator and test tooling may use `std`.

## 4. Cooperative long-running work

EPUB XML parsing, HTML parsing, CSS processing, pagination, indexing and other expensive operations must be resumable. Each unit exposes a step/poll API with an explicit work budget.

## 5. Storage as working memory

The reader may spill large intermediate structures, caches and manifests to persistent storage. RAM usage must be bounded and intentional.

## 6. Deterministic rendering

The renderer owns exact coordinate transforms, framebuffer bit layout, fixed-point text placement, clipping, dithering and display-update semantics. Higher layers do not directly manipulate device controller registers.

## 7. Explicit ownership

A framebuffer borrow/lease must be represented by Rust ownership/borrowing wherever possible. No subsystem should retain a raw framebuffer pointer across an unrelated operation.

## 8. No global application singletons

Construct services and pass references/dependencies explicitly. Global constants are acceptable; global mutable service singletons are not.

## 9. Optional capabilities

Touch, grayscale, RTC, frontlight, warm frontlight, battery gauge, accelerometer, Wi-Fi, USB and OTA are optional. Core reader functionality must remain valid without them unless the selected application explicitly requires them.

## 10. Profiles

- `Core`: local reading essentials.
- `Extended`: richer e-reader hardware/features such as grayscale, frontlight, RTC, sensors, dictionaries and XTC.
- `Connected`: Wi-Fi, HTTP/WebDAV, OPDS, OTA and synchronization.

## 11. First validation target

The first complete vertical slice must execute on both:

```text
host simulator
ESP32-C3
```

The same core reader logic must be used in both environments.
