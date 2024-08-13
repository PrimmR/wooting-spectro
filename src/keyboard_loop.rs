use crate::*;
use options::ActiveDevice;
use tray::TrayMessage;

use std::mem::take;
use std::sync::{mpsc::Receiver, Arc, Mutex, RwLock};
use std::thread::sleep;
use std::time::Duration;

use cpal::{StreamConfig, StreamError};

use wooting_rgb::*;
use wooting_rgb_sys::*;

const FPS: u32 = 60;
const DECAY: f32 = 30. / FPS as f32;

pub fn draw<T>(
    buf: Arc<Mutex<Vec<T>>>,
    config: &StreamConfig,
    tray_rx: &Receiver<TrayMessage>,
    stream_err_rx: Receiver<StreamError>,
    opt: Arc<RwLock<Options>>,
) -> Result<ExitState>
where
    T: SampleVal,
{
    let theme = opt.read().unwrap().theme.get_theme();
    let mut keyboard = Keyboard::from_meta_and_theme(get_info(), theme);

    // LOOP
    let keyboard_cols = get_info().max_columns.into();
    let mut spec_mem = SpectrumMemory::new(FreqIntensity::stateless_log_sections(keyboard_cols, FREQ_RANGE), DECAY);

    loop {
        keyboard.display(&spec_mem, opt.read().unwrap().caps_active);

        // Constrain FPS
        sleep(Duration::new(0, 1_000_000_000u32 / FPS));

        // Get new audio frames
        {
            let data_buffer = take(&mut *buf.lock().unwrap());

            let fft = fft::process(data_buffer, config.channels, config.sample_rate.0);
            spec_mem.tick();
            spec_mem.push(fft.log_sections(keyboard_cols, FREQ_RANGE));
        }

        // Events
        match stream_err_rx.try_recv() {
            Err(_) => (),
            Ok(StreamError::DeviceNotAvailable) => {
                opt.write().unwrap().device = ActiveDevice::Default;
                return Ok(ExitState::Restart);
            }
            Ok(StreamError::BackendSpecific { err }) => {
                eprintln!("{err}");
                return Ok(ExitState::Exit(keyboard.close()));
            }
        };

        match tray_rx.try_recv() {
            Err(_) => (),
            Ok(TrayMessage::ThemeReload) => {
                let theme = opt.read().unwrap().theme;
                keyboard.set_theme(theme.get_theme());
            }
            Ok(TrayMessage::Refresh) => {
                return Ok(ExitState::Restart);
            }
            Ok(TrayMessage::Quit) => {
                return Ok(ExitState::Exit(keyboard.close()));
            }
        }
        if !is_wooting_keyboard_connected() {
            return Ok(ExitState::Exit(keyboard.close()));
        }
    }
}

fn get_info() -> WOOTING_USB_META {
    unsafe { *wooting_rgb_sys::wooting_rgb_device_info() }
}
