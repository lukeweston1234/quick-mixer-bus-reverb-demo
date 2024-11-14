pub struct AudioSample<T> {
    samples: Vec<T>,
    is_mono: bool,
    sample_rate: u32,
}
impl<T> AudioSample<T>
where
    T: Copy,
{
    pub fn new(samples: Vec<T>, is_mono: bool, sample_rate: u32) -> Self {
        Self {
            samples,
            is_mono,
            sample_rate,
        }
    }
    pub fn get_sample(&self, left_index: usize) -> Option<(T, T)> {
        let left_sample = *self.samples.get(left_index)?;
        let right_sample = if self.is_mono {
            left_sample
        } else {
            *self.samples.get(left_index + 1)?
        };
        Some((left_sample, right_sample))
    }
    pub fn set_samples(&mut self, new_sample: Vec<T>) {
        self.samples = new_sample;
    }
    pub fn get_sample_size(&self) -> usize {
        self.samples.len()
    }
}
