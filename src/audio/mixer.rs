use super::{playable::Playable, reverb::Freeverb};

pub struct Mixer<T> {
    source: Box<dyn Playable<T> + Send>,
    reverb: Freeverb
}
impl<T> Mixer<T>{
    pub fn new(source: Box<dyn Playable<T> + Send>) -> Self {
        Self {
            source,
            reverb: Freeverb::new(44_100)
        }
    }
    pub fn set_reverb_wet(&mut self, wet: f32){
        self.reverb.set_wet(wet);
    }
}
impl Playable<f32> for Mixer<f32> {
    fn tick(&mut self) -> Option<(f32, f32)> {
        match self.source.tick(){
            Some(sample) => Some(self.reverb.tick(sample)),
            None => None
        }
    }
}