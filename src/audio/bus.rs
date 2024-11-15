use super::{playable::Playable, reverb::Freeverb};

pub struct Bus<T> {
    sources: Vec<Box<dyn Playable<T> + Send>>,
    reverb: Freeverb,
}
impl<T> Bus<T> {
    pub fn new(sources: Vec<Box<dyn Playable<T> + Send>>) -> Self {
        let mut reverb = Freeverb::new(44_100);
        reverb.set_wet(0.0);
        Self {
            sources,
            reverb: reverb,
        }
    }
    pub fn set_reverb_wet(&mut self, wet: f32) {
        self.reverb.set_wet(wet);
    }
}
impl Playable<f32> for Bus<f32> {
    fn tick(&mut self) -> Option<(f32, f32)> {
        let mut total_l = 0.0;
        let mut total_r = 0.0;
        for source in self.sources.iter_mut() {
            if let Some(sample) = source.tick() {
                total_l += sample.0;
                total_r += sample.1
            }
        }
        Some((total_l, total_r))
    }
}
