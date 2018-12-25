use super::Sample;
use std::{collections::HashMap, mem};

pub struct LoopManager {
    loops: HashMap<usize, Loop>,
    playing_loops: HashMap<usize, LoopPlayback>,
    recording_loop: Option<LoopRecorder>,
}
impl LoopManager {
    pub fn new() -> LoopManager {
        LoopManager { loops: HashMap::new(), playing_loops: HashMap::new(), recording_loop: None }
    }
    pub fn toggle_recording(&mut self, index: usize) {
        if let Some(recorder) = mem::replace(&mut self.recording_loop, None) {
            let new_loop = recorder.stop_recording();
            self.loops.insert(index, new_loop);
        } else {
            self.recording_loop = Some(LoopRecorder::new())
        }
    }
    pub fn toggle_playback(&mut self, index: usize) {
        if self.playing_loops.remove(&index).is_none() {
            if let Some(loop_to_play) = self.loops.get(&index) {
                self.playing_loops.insert(index, loop_to_play.start_playback());
            }
        }
    }
    pub fn get_recorder(&mut self) -> Option<&mut LoopRecorder> {
        self.recording_loop.as_mut()
    }
    pub fn next_sample(&mut self) -> Sample {
        self.playing_loops.values_mut()
            .filter_map(|l| l.next())
            .sum()
    }
}

pub struct Loop {
    samples: Vec<Sample>
}
impl Loop {
    pub fn start_playback(&self) -> LoopPlayback {
        LoopPlayback::new(self.samples.to_vec())
    }
}

pub struct LoopRecorder {
    samples: Vec<Sample>,
}
impl LoopRecorder {
    fn new() -> LoopRecorder {
        LoopRecorder { samples: vec![] }
    }
    pub fn write(&mut self, sample: Sample) {
        self.samples.push(sample)
    }
    fn stop_recording(self) -> Loop {
        Loop { samples: self.samples }
    }
}

pub struct LoopPlayback {
    position: usize,
    samples: Vec<Sample>,
}
impl LoopPlayback {
    fn new(samples: Vec<Sample>) -> LoopPlayback {
        LoopPlayback { position: 0, samples }
    }
}
impl Iterator for LoopPlayback {
    type Item = Sample;
    fn next(&mut self) -> Option<Sample> {
        let position = self.position;
        self.position = (self.position + 1) % self.samples.len();
        self.samples.get(position).cloned()
    }
}