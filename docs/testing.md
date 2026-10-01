# Testing Contract

The repository is designed for test-first implementation.

## Unit tests

Every parser, serializer, state machine, geometry transform, text-layout primitive and protocol parser must have unit tests next to the implementation or in an adjacent test module.

## Property tests

Use host-side property testing where practical for:

- path normalization and cache-path generation,
- natural sort,
- UTF-8 chunk-boundary handling,
- ZIP/XML/HTML parser chunk boundaries,
- fixed-point arithmetic,
- orientation transforms,
- cache decoding bounds checks.

## Golden tests

Maintain golden fixtures for:

- framebuffer pixels,
- dithering output,
- glyph bitmaps,
- pagination/page layouts,
- EPUB metadata/TOC extraction,
- StarDict lookup results,
- XPath mapping.

## Differential testing

Where the reference CrossPoint implementation is available as an executable oracle, feed the same fixture into both implementations and compare the agreed representation: bytes, structure, coordinates or rendered pixels.

The Rust architecture must not make such testing dependent on an ESP32 device. The host simulator is the first oracle target.
