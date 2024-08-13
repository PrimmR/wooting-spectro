use crate::{
    options::{ActiveDevice, Options},
    tray::TrayMessage,
    ExitState, Result, SampleVal,
};

use std::sync::{
    mpsc::{sync_channel, Receiver},
    Arc, Mutex, RwLock,
};

use casey::lower;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SizedSample,
};

macro_rules! format_match(
    ( $m:expr,$d:expr,$c:expr,$o:expr,$rx:expr, $( $x:ident ),* ) => {
        match $m {
            $(
                cpal::SampleFormat::$x => run::<lower!($x)>($d, $c, $o, $rx),
            )*
            sample_format => {
                panic!("Unknown format: {}", sample_format)
            }
        }
    };
);

pub fn setup(opt: Arc<RwLock<Options>>, rx: &Receiver<TrayMessage>) -> Result<ExitState> {
    let host = cpal::default_host();

    let device = if let ActiveDevice::Named(ref n) = opt.read().unwrap().device {
        if let Some(d) = host.output_devices()?.find(|x| x.name().map(|y| &y == n).unwrap_or(false)) {
            d
        } else {
            opt.write().unwrap().device = ActiveDevice::Default;
            host.default_output_device().unwrap()
        }
    } else {
        host.default_output_device().unwrap()
    };

    let config = device.default_output_config().expect("Failed to get default input config");
    let form = config.sample_format();

    let config = cpal::StreamConfig {
        channels: 2, ..config.into()

    }
;
    format_match!(form, device, config.into(), opt, rx, I8, I16, I32, F32)
}

pub fn run<T>(
    device: cpal::Device,
    config: cpal::StreamConfig,
    opt: Arc<RwLock<Options>>,
    rx: &Receiver<TrayMessage>,
) -> Result<ExitState>
where
    T: SizedSample + SampleVal,
{
    let buffer: Arc<Mutex<Vec<T>>> = Arc::new(Mutex::new(Vec::new()));
    let buffer_clone = buffer.clone();
    let config_clone = config.clone();

    let (stream_err_tx, stream_err_rx) = sync_channel(1);

    let err_fn = move |err| {
        let _ = stream_err_tx.send(err);
    };

    let stream = device.build_input_stream(
        &config,
        move |data: &[T], _: &_| buffer.lock().unwrap().extend_from_slice(data),
        err_fn,
        None,
    )?;

    stream.play()?;

    #[cfg(feature = "window-display")]
    {
        crate::draw::draw(buffer_clone, &config_clone)
    }
    #[cfg(not(feature = "window-display"))]
    {
        crate::keyboard_loop::draw(buffer_clone, &config_clone, rx, stream_err_rx, opt)
    }
}

pub fn get_devices() -> Result<Vec<String>> {
    let host = cpal::default_host();

    let names = host.output_devices()?.filter_map(|x| x.name().ok()).collect();

    Ok(names)
}
