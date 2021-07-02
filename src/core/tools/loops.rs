use crate::core::synth::Sample;
use std::{collections::HashMap, mem};

#[derive(Clone, Copy)]
pub enum Command { TogglePlayback(usize), ToggleRecording(usize) }

#[derive(Default)]
pub struct Manager {
    loops: HashMap<usize, Loop>,
    playing_loops: HashMap<usize, Playback>,
    recording_loop: Option<Recorder>,
}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct View {
    pub playing_loops: HashMap<usize, bool>,
    pub recording_loop: Option<usize>,
}

impl Manager {

    pub fn interpret(&mut self, command: Command) {
        match command {
            Command::TogglePlayback(i) => self.toggle_playback(i),
            Command::ToggleRecording(i) => self.toggle_recording(i),
        }
    }

    fn toggle_recording(&mut self, index: usize) {
        if let Some(recorder) = mem::replace(&mut self.recording_loop, None) {
            let new_loop = recorder.stop_recording();
            self.loops.insert(index, new_loop);
        } else {
            self.recording_loop = Some(Recorder::new(index))
        }
    }

    fn toggle_playback(&mut self, index: usize) {
        if self.playing_loops.remove(&index).is_none() {
            if let Some(loop_to_play) = self.loops.get(&index) {
                self.playing_loops.insert(index, loop_to_play.start_playback());
            }
        }
    }

    pub fn write(&mut self, sample: Sample) {
        if let Some(rec) = self.recording_loop.as_mut() {
            rec.write(sample)
        }
    }

    pub fn next_sample(&mut self) -> Sample {
        self.playing_loops.values_mut()
            .filter_map(|l| l.next())
            .sum()
    }

    pub fn view(&self) -> View {
        View {
            playing_loops: self.loops.keys().map(|k| (*k, self.playing_loops.contains_key(k))).collect(),
            recording_loop: self.recording_loop.as_ref().map(|l| l.position),
        }
    }
}

struct Loop {
    samples: Vec<Sample>
}
impl Loop {
    fn start_playback(&self) -> Playback {
        Playback::new(self.samples.to_vec())
    }
}

struct Recorder {
    position: usize,
    samples: Vec<Sample>,
}
impl Recorder {
    fn new(position: usize) -> Recorder {
        Recorder { position, samples: vec![] }
    }
    fn write(&mut self, sample: Sample) {
        self.samples.push(sample)
    }
    fn stop_recording(self) -> Loop {
        Loop { samples: self.samples }
    }
}

struct Playback {
    position: usize,
    samples: Vec<Sample>,
}
impl Playback {
    fn new(samples: Vec<Sample>) -> Playback {
        Playback { position: 0, samples }
    }
}
impl Iterator for Playback {
    type Item = Sample;
    fn next(&mut self) -> Option<Sample> {
        let position = self.position;
        self.position = (self.position + 1) % self.samples.len();
        self.samples.get(position).cloned()
    }
}