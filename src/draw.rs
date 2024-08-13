use crate::*;

use std::mem::take;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use cpal::StreamConfig;

const FPS: u32 = 30;
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const MULT: f32 = HEIGHT as f32 / (MAX_DB - MIN_DB);
const SECTIONS: u32 = 12;
const DECAY: f32 = 30. / FPS as f32;

pub fn draw<T>(
    buf: Arc<Mutex<Vec<T>>>,
    config: &StreamConfig,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: SampleVal,
{
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Test", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let mut spec_mem = SpectrumMemory::new(
        FreqIntensity::stateless_log_sections(SECTIONS, FREQ_RANGE),
        DECAY,
    );

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 0, 255));

        const SECTION_WIDTH: u32 = WIDTH / SECTIONS;

        for (i, (_freq, weight)) in spec_mem.weights.iter().enumerate() {
            let h = ((weight - crate::MIN_DB) * MULT) as u32;
            let rect = Rect::new(
                i as i32 * SECTION_WIDTH as i32,
                HEIGHT as i32 - h as i32,
                SECTION_WIDTH,
                h,
            );
            canvas.draw_rect(rect).unwrap();
        }

        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));

        {
            let data_buffer = take(&mut *buf.lock().unwrap());

            let fft = fft::process(data_buffer, config.channels, config.sample_rate.0);

            spec_mem.tick();
            spec_mem.push(fft.log_sections(SECTIONS, FREQ_RANGE));
        }
    }

    Ok(())
}
