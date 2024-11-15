// Graph is generious at this point...

use crossbeam_channel::Receiver;

use super::{
    bus::Bus,
    mixer::{build_mixer, MixerController},
    playable::Playable,
    track::{build_track, TrackController},
};

pub fn build_audio_graph(
    input_receiver: Receiver<(f32, f32)>,
    amount: usize,
    track_size: usize,
) -> (Bus<f32>, Vec<TrackController>, Vec<MixerController>) {
    let mut track_controllers = Vec::with_capacity(amount);
    let mut mixer_controllers = Vec::with_capacity(amount);
    let mut mixers: Vec<Box<dyn Playable<f32> + Send>> = Vec::with_capacity(amount);

    for _ in 0..amount {
        let (new_track_controller, new_track) = build_track(input_receiver.clone(), track_size);

        track_controllers.push(new_track_controller);

        let (new_mixer_controller, new_mixer) = build_mixer(Box::new(new_track));

        mixers.push(Box::new(new_mixer));
        mixer_controllers.push(new_mixer_controller);
    }

    let bus = Bus::new(mixers);

    (bus, track_controllers, mixer_controllers)
}
