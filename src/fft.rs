use crate::FreqIntensity;
use crate::SampleVal;

use easyfft::num_complex::Complex;
use easyfft::prelude::*;

use float_ord::FloatOrd;

const THRESHOLD: f32 = 0.2;

/// Processes a sample into its constituent frequencies
pub fn process<T>(samples: Vec<T>, channels: u16, sample_rate: u32) -> FreqIntensity
where
    T: SampleVal,
{
    let mut freq_intensity: Option<FreqIntensity> = None;

    for channel in outerleave(samples, channels) {
        let fft = fft(&channel, sample_rate);
        freq_intensity = Some(if let Some(v) = freq_intensity { v.combine_mean(fft) } else { fft });
    }

    freq_intensity.expect("Number of channels is 0")
}

#[inline]
fn complex_to_f32(w: &Complex<impl SampleVal>) -> Complex<f32> {
    Complex::new(w.re.to_f32(), w.im.to_f32())
}

/// Performs a Forward Fourier Transform on the sample to create a frequency weighting
fn fft<T>(buffer: &[T], sample_rate: u32) -> FreqIntensity
where
    T: SampleVal,
{
    let buffer: Vec<f32> = buffer.iter().map(T::norm_to_f32).collect();

    let bins = buffer.len();
    let fft = buffer.fft();

    let amp_weights = (0..bins / 2)
        .map(|i| {
            let freq = (i as f32 / bins as f32) * sample_rate as f32;
            // Amplitude is modulus, phase is argument
            let amp = 2. * complex_to_f32(&fft[i]).norm() / bins as f32;
            (freq, amp)
        })
        .collect();

    let filtered_db_weights =
        filter(amp_weights, THRESHOLD).iter().map(|(freq, amp)| (*freq, 10. * amp.log10())).collect();

    FreqIntensity(filtered_db_weights)
}

fn filter(weights: Vec<(f32, f32)>, threshold_prct: f32) -> Vec<(f32, f32)> {
    if let Some((_, max)) = weights.iter().max_by_key(|x| FloatOrd(x.1)) {
        weights.iter().map(|(f, a)| (*f, if max * threshold_prct > *a { 0. } else { *a })).collect()
    } else {
        weights
    }
}

fn outerleave<T>(samples: Vec<T>, channels: impl Into<usize> + Copy) -> Vec<Vec<T>>
where
    T: SampleVal,
{
    let mut out = vec![Vec::new(); channels.into()];

    for chunk in samples.chunks(channels.into()) {
        chunk.iter().enumerate().for_each(|(i, x)| out[i].push(*x))
    }

    out
}
