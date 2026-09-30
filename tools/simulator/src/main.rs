mod device;

use device::SimulatorDevice;
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use reader_os::graphics::color::Pixel;
use reader_os::graphics::framebuffer::{FrameBuffer, FrameBufferView, PixelFormat};
use reader_os::graphics::geometry::Size;
use reader_os::graphics::renderer::Renderer;
use reader_os::hal::display::{EinkDisplay, RefreshMode, RefreshRequest};
use reader_os::hal::input::{InputEvent, LogicalButton};
use reader_os::kernel::services::System;
use reader_os::ui::activity::ActivityContext;
use reader_os::ui::navigation::{ActivityManager, PendingTransition};

fn get_activity(idx: usize) -> Option<Box<dyn reader_os::ui::activity::Activity>> {
    match idx {
        0 => Some(Box::new(reader_os::ui::activities::SplashActivity)),
        1 => Some(Box::new(reader_os::ui::activities::EpubReaderActivity)),
        2 => Some(Box::new(reader_os::ui::activities::XtcReaderActivity)),
        3 => Some(Box::new(reader_os::ui::activities::CrashReportActivity)),
        4 => Some(Box::new(reader_os::ui::activities::LibraryListActivity)),
        5 => Some(Box::new(reader_os::ui::activities::FileBrowserActivity)),
        6 => Some(Box::new(reader_os::ui::activities::SettingsActivity)),
        7 => Some(Box::new(reader_os::ui::activities::TextSettingsActivity)),
        8 => Some(Box::new(reader_os::ui::activities::ButtonRemapActivity)),
        9 => Some(Box::new(reader_os::ui::activities::ClockSettingsActivity)),
        10 => Some(Box::new(reader_os::ui::activities::ClockSyncActivity)),
        11 => Some(Box::new(reader_os::ui::activities::TimezonePickerActivity)),
        12 => Some(Box::new(reader_os::ui::activities::LanguageSelectActivity)),
        13 => Some(Box::new(
            reader_os::ui::activities::StatusBarSettingsActivity,
        )),
        14 => Some(Box::new(
            reader_os::ui::activities::HomeButtonSettingsActivity,
        )),
        15 => Some(Box::new(reader_os::ui::activities::KeyboardLayoutsActivity)),
        16 => Some(Box::new(reader_os::ui::activities::OtaUpdateActivity)),
        17 => Some(Box::new(
            reader_os::ui::activities::SdFirmwareUpdateActivity,
        )),
        18 => Some(Box::new(reader_os::ui::activities::FontDownloadActivity)),
        19 => Some(Box::new(reader_os::ui::activities::ClearCacheActivity)),
        20 => Some(Box::new(reader_os::ui::activities::AboutActivity)),
        21 => Some(Box::new(reader_os::ui::activities::FrontlightPanelActivity)),
        22 => Some(Box::new(reader_os::ui::activities::KOReaderAuthActivity)),
        23 => Some(Box::new(
            reader_os::ui::activities::KOReaderSettingsActivity,
        )),
        24 => Some(Box::new(reader_os::ui::activities::OpdsServerListActivity)),
        25 => Some(Box::new(reader_os::ui::activities::OpdsSettingsActivity)),
        26 => Some(Box::new(reader_os::ui::activities::WifiSelectionActivity)),
        27 => Some(Box::new(
            reader_os::ui::activities::CrossPointWebServerActivity,
        )),
        28 => Some(Box::new(reader_os::ui::activities::UsbDriveActivity)),
        29 => Some(Box::new(reader_os::ui::activities::OpdsBookBrowserActivity)),
        30 => Some(Box::new(reader_os::ui::activities::OpdsDownloadActivity)),
        31 => Some(Box::new(reader_os::ui::activities::KeyboardEntryActivity)),
        32 => Some(Box::new(reader_os::ui::activities::ConfirmationActivity)),
        33 => Some(Box::new(
            reader_os::ui::activities::FullScreenMessageActivity,
        )),
        34 => Some(Box::new(
            reader_os::ui::activities::IntervalSelectionActivity,
        )),
        35 => Some(Box::new(reader_os::ui::activities::BmpViewerActivity)),
        36 => Some(Box::new(reader_os::ui::activities::SleepActivity)),
        _ => None,
    }
}

fn main() {
    let width = 400;
    let height = 800;
    let sim_device = SimulatorDevice::new(width, height);
    let mut system = System::new(sim_device);
    let mut activity_manager = ActivityManager::new();

    let font_data =
        std::fs::read("/usr/share/fonts/adwaita-mono-fonts/AdwaitaMono-Bold.ttf").unwrap();
    let mut font_system = reader_os::reader::fonts::FontSystem::new();
    font_system
        .add_font(reader_os::graphics::text::FontId(0), &font_data)
        .unwrap();

    activity_manager.go_home();

    let mut window = Window::new(
        "ReaderOS Simulator",
        width as usize,
        height as usize,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_target_fps(60);

    let mut minifb_buffer: std::vec::Vec<u32> = vec![0; width as usize * height as usize];
    let mut was_mouse_down = false;
    let mut current_activity_idx = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let is_mouse_down = window.get_mouse_down(MouseButton::Left);
        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Discard) {
            if is_mouse_down && !was_mouse_down {
                system.device.input.pending_events.push(InputEvent::Tap {
                    x: mouse_x as i16,
                    y: mouse_y as i16,
                });
                activity_manager.request_render();
            }
        }
        was_mouse_down = is_mouse_down;

        window
            .get_keys_pressed(minifb::KeyRepeat::No)
            .iter()
            .for_each(|key| {
                if *key == Key::Right {
                    if let Some(act) = get_activity(current_activity_idx) {
                        activity_manager.pending = Some(PendingTransition::Push(act));
                        activity_manager.request_render();
                        current_activity_idx += 1;
                    }
                } else if *key == Key::Left {
                    activity_manager.pending = Some(PendingTransition::Pop);
                    activity_manager.request_render();
                    if current_activity_idx > 0 {
                        current_activity_idx -= 1;
                    }
                } else {
                    let logical_button = match key {
                        Key::Up => Some(LogicalButton::Up),
                        Key::Down => Some(LogicalButton::Down),
                        Key::Enter => Some(LogicalButton::Confirm),
                        Key::Backspace => Some(LogicalButton::Back),
                        Key::PageUp => Some(LogicalButton::PageBack),
                        Key::PageDown => Some(LogicalButton::PageForward),
                        _ => None,
                    };
                    if let Some(btn) = logical_button {
                        system
                            .device
                            .input
                            .pending_events
                            .push(InputEvent::ButtonPressed(btn));
                        activity_manager.request_render();
                    }
                }
            });

        {
            let mut dummy_fb_bytes = vec![0u8; width as usize * height as usize];
            let dummy_fb = FrameBuffer::new(
                &mut dummy_fb_bytes,
                Size { width, height },
                PixelFormat::Grayscale8,
            )
            .unwrap();
            let mut dummy_renderer = Renderer::new(dummy_fb);
            let mut ctx = ActivityContext {
                renderer: &mut dummy_renderer,
                font_provider: &mut font_system,
            };

            let _ = activity_manager.apply_pending(&mut ctx);
        }

        while let Some(event) = system.device.input.pending_events.pop() {
            if let Some(mut current) = activity_manager.current.take() {
                let mut dummy_fb_bytes = vec![0u8; width as usize * height as usize];
                let dummy_fb = FrameBuffer::new(
                    &mut dummy_fb_bytes,
                    Size { width, height },
                    PixelFormat::Grayscale8,
                )
                .unwrap();
                let mut dummy_renderer = Renderer::new(dummy_fb);
                let mut ctx = ActivityContext {
                    renderer: &mut dummy_renderer,
                    font_provider: &mut font_system,
                };

                let _ = current.poll(&[event], &mut ctx);
                activity_manager.current = Some(current);
            }
        }

        if activity_manager.update_requested {
            if let Some(mut current) = activity_manager.current.take() {
                let mut fb_bytes = vec![0u8; width as usize * height as usize];
                let size = Size { width, height };
                let mut fb =
                    FrameBuffer::new(&mut fb_bytes, size, PixelFormat::Grayscale8).unwrap();
                fb.clear(Pixel::White);

                {
                    let mut renderer = Renderer::new(fb);
                    let mut ctx = ActivityContext {
                        renderer: &mut renderer,
                        font_provider: &mut font_system,
                    };

                    let _ = current.render(&mut ctx);
                }

                let view = FrameBufferView {
                    bytes: &fb_bytes,
                    size,
                    format: PixelFormat::Grayscale8,
                };
                system
                    .device
                    .display
                    .refresh(
                        &view,
                        RefreshRequest {
                            mode: RefreshMode::Fast,
                            grayscale: true,
                            asynchronous: false,
                        },
                    )
                    .unwrap();
                activity_manager.current = Some(current);
            }
            activity_manager.update_requested = false;
        }

        let fb_bytes = &system.device.display.fb_bytes;
        // In the simulator we need to map Grayscale8 from fb_bytes if we know it was updated
        // Wait, system.device.display is simulated, it might still just copy bytes over.
        // We actually want to render from our own fb buffer, or assume system.device.display.fb_bytes has the Grayscale8 data.
        // Assuming system.device.display.fb_bytes now contains Grayscale8 data since we passed it in refresh()
        for y in 0..height {
            for x in 0..width {
                let index = y as usize * width as usize + x as usize;
                if index < fb_bytes.len() {
                    let val = fb_bytes[index];
                    // 0 is white, 255 is black
                    // ARGB format: A=FF, R=255-val, G=255-val, B=255-val
                    let c = 255 - val as u32;
                    let color = (0xFF << 24) | (c << 16) | (c << 8) | c;

                    minifb_buffer[index] = color;
                }
            }
        }

        window
            .update_with_buffer(&minifb_buffer, width as usize, height as usize)
            .unwrap();
    }
}
