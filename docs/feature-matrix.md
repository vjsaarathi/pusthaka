# ReaderOS Feature/Architecture Matrix

This matrix maps the full feature surface identified in the CrossPoint analysis to the new ReaderOS architecture.

| Reference feature | ReaderOS construct | Profile | Implementation status |
|---|---|---|---|
| Boot/wakeup routing | `kernel::boot::BootManager` | Core | Contract defined |
| Crash-loop protection | `RuntimeState.reader_activity_load_count` | Core | Contract defined |
| Activity lifecycle | `ui::activity::Activity` | Core | Contract defined |
| Stack navigation | `ui::navigation::ActivityManager` | Core | Contract defined |
| Deferred transitions | `PendingTransition` | Core | Contract defined |
| Logical button mapping | `hal::input::InputMapper` | Core | Contract defined |
| Touch gestures | `TouchSource` + `InputEvent` | Core/Extended | Contract defined |
| Orientation | `graphics::geometry::Rotation` | Core | Contract defined |
| 1-bit framebuffer | `graphics::framebuffer::FrameBuffer` | Core | Contract defined |
| Grayscale planes | `RenderMode` + display capabilities | Extended | Contract defined |
| E-ink refresh policy | `RefreshRequest` | Core | Contract defined |
| Bresenham lines | `Renderer::draw_line` | Core | Contract defined |
| Polygon/circle primitives | `Renderer` | Core | Contract defined |
| Clipping | `Renderer::set_clip` | Core | Contract defined |
| Strip rendering | renderer/display contract | Extended | Contract defined |
| 12.4 fixed-point text | `FixedPoint4` | Core | Contract defined |
| Atkinson/Floyd/Bayer | `graphics::dithering` | Extended | Contract defined |
| BMP | `BitmapReader` | Core/Extended | Contract defined |
| PNG/JPEG | `ImageDecoder` | Extended | Contract defined |
| Built-in fonts | `FontProvider` / `FontSystem` | Core | Contract defined |
| `.cpfont` v4 | `formats::cpfont` | Core/Extended | Format contract defined |
| SD font LRU cache | `FontCache` | Core/Extended | Contract defined |
| TrueType vector fonts | `FontSource::TrueType` | Extended | Contract defined |
| Unicode bidi | `reader::shaping::BidiResolver` | Extended | Contract defined |
| Arabic shaping | `ArabicShaper` | Extended | Contract defined |
| EPUB ZIP streaming | `reader::epub::zip::ZipReader` | Core | Contract defined |
| Container.xml | `ContainerParser` | Core | Contract defined |
| OPF | `ContentOpfParser` | Core | Contract defined |
| EPUB3 nav | `TocNavParser` | Core | Contract defined |
| EPUB2 NCX | `TocNcxParser` | Core | Contract defined |
| HTML block model | `ChapterHtmlParser` + `Block` | Core | Contract defined |
| CSS subset | `CssParser`, `CssSelector`, `CssStyle` | Core | Contract defined |
| CSS cache | `CSS_CACHE_VERSION`, format layer | Core | Contract defined |
| Incremental pagination | `PaginationEngine` | Core | Contract defined |
| Disk-backed section cache | `SectionCacheFormat` | Core | Contract defined |
| Word-store bump allocation | `WordStore` | Core | Contract defined |
| Liang hyphenation | `LiangHyphenator` | Core | Contract defined |
| CJK ruby | HTML parser tags | Extended | Contract defined |
| Tables + stacked fallback | HTML/layout contract | Core | Contract defined |
| Footnotes | `ReaderSession.footnote_stack` | Core | Contract defined |
| EPUB cover extraction | `EpubEngine::extract_cover` | Core | Contract defined |
| XTC/XTCH | `formats::xtc`, document format | Extended | Byte layout intentionally not invented |
| EPUB/TXT/XTC/BMP selection | `DocumentFormat` | Core | Contract defined |
| Reader overlays | `ReaderOverlay` | Core | Contract defined |
| Automatic page turn | `ReaderSession` | Core | Contract defined |
| Percentage navigation | `ReaderPosition` / reader navigation | Core | Contract defined |
| Bookmarks | `BookmarkEntry` / `BookmarkStore` | Core | Contract defined |
| Recent books | `RecentBooksStore` | Core | Contract defined |
| Library index | `LibraryIndex` | Core | Contract defined |
| Natural sort/search | `NaturalSort` / `LibraryIndex` | Core | Contract defined |
| Settings | `services::persistence::Settings` | Core | Contract defined |
| Runtime state | `RuntimeState` | Core | Contract defined |
| Sleep screens | `ui::sleep::SleepScreenState` | Core/Extended | Contract defined |
| Frontlight | `hal::frontlight::Frontlight` | Extended | Contract defined |
| RTC/timezone | `hal::clock::Clock` | Extended | Contract defined |
| Battery | `hal::battery::BatteryGauge` | Extended | Contract defined |
| Dictionary/StarDict | `reader::dictionary`, `formats::stardict` | Extended | Contract defined |
| DictZip | `DictZipReader` | Extended | Contract defined |
| i18n | `I18nService` | Core | Contract defined |
| QR | `QrEncoder` | Connected | Contract defined |
| Wi-Fi | `NetworkInterface` | Connected | HAL contract only |
| HTTP/WebSocket | `WebServerService` | Connected | Contract defined |
| WebDAV | `WebDavService` | Connected | Contract defined |
| OPDS | `OpdsClient` | Connected | Contract defined |
| OTA | `OtaService` / `FirmwareUpdater` | Connected | Contract defined |
| KOReader sync | `KOSyncClient` / `XPathResolver` | Connected | Contract defined |
| USB mass storage | `Capability::UsbMassStorage` + future device service | Connected | Capability reserved |

The absence of implementation code is deliberate: the scaffold defines the stable architecture before algorithms/drivers are implemented.
