use crate::MIN_DB;

use std::ops::RangeInclusive;

use float_ord::FloatOrd;

/// Struct to hold the Frequency-DB pairs of a frame of audio
#[derive(Debug, Default)]
pub struct FreqIntensity(pub Vec<(f32, f32)>);

impl FreqIntensity {
    /// Gets the Frequency-DB pair with the frequency closest to a given value, calculated arithmetically
    pub fn get_nearest_freq(&self, freq: f32) -> (f32, f32) {
        *self.0.iter().min_by_key(|(f, _)| FloatOrd((f - freq).abs())).unwrap_or(&(MIN_DB, MIN_DB))
    }

    /// Gets the Frequency-DB pair with the frequency closest to a given value, calculated arithmetically
    pub fn get_nearest_freq_log(&self, freq: f32) -> (f32, f32) {
        *self.0.iter().min_by_key(|(f, _)| FloatOrd((f.log2() - freq.log2()).abs())).unwrap_or(&(MIN_DB, MIN_DB))
    }

    /// Gets the Frequency-DB pair with the highest intensity
    pub fn max(&self) -> &(f32, f32) {
        self.0.iter().max_by_key(|x| FloatOrd(x.1)).unwrap_or(&(MIN_DB, MIN_DB))
    }

    /// Returns the number of Frequency-DB pairs contained
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if [Self::len] is 0
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Combines two [FreqIntensity] structs by calculating the mean weight for each pair
    pub fn combine_mean(&self, other: Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(a, b)| (a.0, neg_geo_mean(a.1.max(MIN_DB), b.1.max(MIN_DB))))
                .collect(),
        )
    }

    /// Calculates the (geometric) mean weight and frequency within a given frequency range
    pub fn range_mean_weight(&self, range: std::ops::Range<f32>) -> (f32, f32) {
        let within: Vec<f32> = self.0.iter().filter(|x| range.contains(&x.0)).map(|x| x.1).collect();
        if within.is_empty() {
            self.get_nearest_freq_log(geo_mean(range.start, range.end))
        } else {
            (geo_mean(range.start, range.end), *within.iter().max_by_key(|x| FloatOrd(**x)).unwrap_or(&MIN_DB))
        }
    }

    /// Returns the bounding values to split a range into a certain number of sections using a logarithmic scale
    fn get_fenceposts(sections: u32, range: RangeInclusive<f32>) -> Vec<f32> {
        let (log_start, log_end) = (range.start().log2(), range.end().log2());
        let step = (log_end - log_start) / sections as f32;

        (0..=sections).map(|i| (log_start + step * i as f32).exp2()).collect()
    }

    /// Returns Frequency-DB pairs corresponding to a given number of sections within a range
    /// The size of each section increases exponentially
    pub fn log_sections(&self, sections: u32, range: RangeInclusive<f32>) -> Vec<(f32, f32)> {
        let lin_fenceposts: Vec<f32> = Self::get_fenceposts(sections, range);

        lin_fenceposts.windows(2).map(|x| self.range_mean_weight(x[0]..x[1])).collect()
    }

    /// Static method that does the same as [Self::log_sections], but only outputs the frequency component
    pub fn stateless_log_sections(sections: u32, range: RangeInclusive<f32>) -> Vec<f32> {
        let lin_fenceposts: Vec<f32> = Self::get_fenceposts(sections, range);

        lin_fenceposts.windows(2).map(|x| geo_mean(x[0], x[1])).collect()
    }
}

/// Calculate the geometric (or of logs) mean of two floats
fn geo_mean(a: f32, b: f32) -> f32 {
    (a * b).sqrt()
}

fn neg_geo_mean(a: f32, b: f32) -> f32 {
    -(-a * -b).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean() {
        assert_eq!(3., geo_mean(1., 9.))
    }
}
