#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod audio;
mod fft;
pub mod keyboard_loop;
pub mod options;
pub mod tray;
pub mod types;

#[cfg(feature = "window-display")]
pub mod draw;

#[cfg(feature = "demo")]
mod demo;

pub use audio::*;
pub use types::*;

pub const MAX_DB: f32 = -6.;
pub const MIN_DB: f32 = -36.;
pub const FREQ_RANGE: std::ops::RangeInclusive<f32> = 20.0..=16_000.0;

pub const OPTIONS_FILE: &str = "options.json";

// Private imports for main
use options::Options;
use std::mem::ManuallyDrop;

// Exit owns keyboard to drop (clear) on exit
pub enum ExitState {
    Restart,
    Exit(ManuallyDrop<wooting_rgb::RgbKeyboard>),
}

fn main() -> Result<()> {
    #[cfg(not(any(windows, target_os = "linux")))]
    compile_error!("This application only targets Windows or Linux systems");

    let instance = Box::new(single_instance::SingleInstance::new(env!("CARGO_BIN_NAME")).unwrap());
    if !instance.is_single() {
        eprintln!("App Already Running");
        std::process::exit(0)
    }

    if !wooting_rgb::is_wooting_keyboard_connected() {
        eprintln!("Keyboard Not Connected");
        std::process::exit(0)
    }

    let opt = Options::read_from_file();
    let opt = std::sync::Arc::new(std::sync::RwLock::new(opt));
    let opt_clone = opt.clone();

    let (tx, rx) = std::sync::mpsc::sync_channel(1);

    #[cfg(feature = "demo")]
    demo::cycle(tx.clone(), opt.clone());

    std::thread::spawn(move || crate::tray::spawn_tray(opt_clone, tx));

    loop {
        let status = audio::setup(opt.clone(), &rx)?;

        if let ExitState::Exit(k) = status {
            opt.read().unwrap().write_to_file();
            ManuallyDrop::into_inner(k); // Drop
            return Ok(());
        }
    }
}
