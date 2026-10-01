# Deliberately Unfixed Design Decisions

These decisions are intentionally represented as explicit TODOs rather than invented by the scaffold.

1. Exact maximum framebuffer allocation beyond the minimum memory contract.
2. Exact storage-throughput certification benchmark.
3. Exact XTC/XTCH binary wire format.
4. Final network transport implementation and dependency pairing.
5. Final PNG/JPEG decoder selection after memory/throughput benchmarking.
6. Exact TTF rasterizer choice after glyph/pixel comparison against the required quality bar.
7. Exact cache format versioning for new ReaderOS-native section caches.
8. Exact bootloader/partition layout for production OTA.

These are not invitations to change the architectural boundaries. They are implementation gates inside those boundaries.
