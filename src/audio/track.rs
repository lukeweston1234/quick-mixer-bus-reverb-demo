use crossbeam_channel::{Receiver};

use super::{audio_sample::AudioSample, playable::Playable};
use crate::audio::sampler::Sampler;

#[derive(PartialEq)]
pub enum TrackState {
    Playing,
    Paused,
    Stopped,
    OnlyInput,
    Recording,
}

pub struct Track<T> where T: Copy + Send {
    state: TrackState,
    sampler: Sampler<T>,
    input_receiver: Receiver<(T,T)>,
    initial_vec_size: usize,
    recording_buffer: Option<Vec<T>>,
}
impl<T> Track<T> 
where T: Copy + Send {
    pub fn new(input_receiver: Receiver<(T,T)>, initial_vec_size: usize)
     -> Self {
        Self {
            state: TrackState::Stopped,
            sampler: Sampler::new(None),
            input_receiver,
            initial_vec_size,
            recording_buffer: Some(Vec::with_capacity(initial_vec_size))
        }
    }
    pub fn play(&mut self){
        self.state = TrackState::Playing;
    }
    pub fn pause(&mut self){
        self.state = TrackState::Paused;
    }
    pub fn stop(&mut self){
        self.sampler.reset_position();
        self.state = TrackState::Stopped;
    }
    pub fn only_feedback(&mut self){
        self.state = TrackState::OnlyInput;
    }
    pub fn record(&mut self){
        if let Some(recording_buffer) = self.recording_buffer.as_mut() {
            recording_buffer.clear();
        }
        self.state = TrackState::Recording;
    }
    pub fn clear_sample(&mut self){
        self.sampler.clear_sample();
    }
    fn add_clip(&mut self){
        let final_clip = self.recording_buffer.take().unwrap();
        self.sampler.set_sample(AudioSample::new(final_clip, false, 44_100));
        self.state = TrackState::Playing;
        self.sampler.play();
    }
    fn handle_recording(&mut self) -> Option<(T,T)> {
        if let Ok(sample) = self.input_receiver.try_recv(){

            let clip = self.recording_buffer.get_or_insert_with(|| Vec::with_capacity(self.initial_vec_size));

            clip.push(sample.0);
            clip.push(sample.1);

            if clip.len() >= self.initial_vec_size {
                self.add_clip();
            }
            return Some(sample);
        }
        None
    }
    fn handle_feedback(&mut self) -> Option<(T,T)> {
        match self.input_receiver.try_recv() {
            Ok(sample) => Some(sample),
            Err(_) => None
        }
    }
}
impl<T> Playable<T> for Track<T>
where T: Copy + Send {
    fn tick(&mut self) -> Option<(T, T)> {
        match self.state {
            TrackState::Playing =>  self.sampler.tick(),
            TrackState::Recording => self.handle_recording(),
            TrackState::OnlyInput => self.handle_feedback(),
            TrackState::Paused => None,
            TrackState::Stopped => None
        }
    }
}

