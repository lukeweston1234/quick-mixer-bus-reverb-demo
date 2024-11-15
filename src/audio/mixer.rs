use crossbeam_channel::{bounded, Receiver, Sender};

use super::{playable::Playable, reverb::Freeverb};

pub enum MixerCommand {
    SetReverbWet(f32),
    SetGain(f32),
}

#[derive(Clone)]
pub struct MixerController {
    sender: Sender<MixerCommand>,
}
impl MixerController {
    fn new(sender: Sender<MixerCommand>) -> Self {
        Self { sender }
    }
    fn set_gain(&self, gain: f32) {
        let _ = self.sender.send(MixerCommand::SetGain(gain));
    }
    fn set_reverb_wet(&self, wet: f32) {
        let _ = self.sender.send(MixerCommand::SetReverbWet(wet));
    }
}

pub struct Mixer<T> {
    source: Box<dyn Playable<T> + Send>,
    controller_receiver: Receiver<MixerCommand>,
    reverb: Freeverb,
    gain: f32, // For the time being, no fancy DB or anything, just multipling
}
impl<T> Mixer<T> {
    pub fn new(
        source: Box<dyn Playable<T> + Send>,
        controller_receiver: Receiver<MixerCommand>,
    ) -> Self {
        Self {
            source,
            controller_receiver: controller_receiver,
            reverb: Freeverb::new(44_100),
            gain: 1.0,
        }
    }
    pub fn set_reverb_wet(&mut self, wet: f32) {
        self.reverb.set_wet(f32::min(1.0, f32::max(0.0, wet)));
    }
    pub fn set_gain(&mut self, gain: f32) {
        self.gain = f32::min(1.5, f32::max(0.0, gain));
    }
}
impl Playable<f32> for Mixer<f32> {
    fn tick(&mut self) -> Option<(f32, f32)> {
        if let Ok(msg) = self.controller_receiver.try_recv() {
            match msg {
                MixerCommand::SetGain(gain) => self.set_gain(gain),
                MixerCommand::SetReverbWet(wet) => self.set_reverb_wet(wet),
            }
        }
        match self.source.tick() {
            Some(sample) => {
                let (wet_l, wet_r) = self.reverb.tick(sample);
                Some((wet_l * self.gain, wet_r * self.gain))
            }
            None => None,
        }
    }
}

pub fn build_mixer<T>(source: Box<dyn Playable<T> + Send>) -> (MixerController, Mixer<T>)
where
    T: Send + Copy,
{
    let (controller_sender, controller_receiver) = bounded(10);

    let mixer_controller = MixerController::new(controller_sender);

    let mixer = Mixer::new(source, controller_receiver);

    (mixer_controller, mixer)
}
