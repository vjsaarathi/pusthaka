# ReaderOS Minimum Device Contract v0.1

A physical device is eligible to run the firmware only if it can satisfy this contract.

## Minimum requirements

| Capability | Minimum requirement |
|---|---|
| CPU | 32-bit MCU |
| RAM | 512 KiB SRAM |
| Firmware flash | 4 MiB persistent flash |
| User storage | Removable or externally accessible persistent storage |
| Display | E-ink/electrophoretic |
| Pixel format | At least 1-bit monochrome |
| Framebuffer | Device resolution must fit the ReaderOS framebuffer memory budget |
| Refresh | Full-screen refresh + completion/busy indication |
| Input | At least one user input mechanism |
| Configuration storage | Small persistent non-volatile storage |
| Monotonic time | Hardware/software monotonic timer |
| Reset | Software/hardware reset capability |
| Low power | Sleep/low-power mode |
| Storage throughput | Must pass the ReaderOS storage benchmark before certification |
| UTF-8 | Firmware must be able to process UTF-8 text |

## Optional hardware

Touch, grayscale, frontlight, warm frontlight, RTC, battery gauge, accelerometer, Wi-Fi, USB mass storage and OTA support are optional capabilities.

## Design choices

The 512 KiB RAM and 4 MiB firmware figures are architecture gates selected for the new OS. They are not claims about CrossPoint's own minimum requirements.

The first concrete hardware target is ESP32-C3. Later targets must implement the same HAL contracts rather than creating a second reader architecture.
