use std::time::{Duration, Instant};
use crate::core::control::synth::Command;
use crate::core::sheet_music::sheet_music::*;

pub struct PlayingMusic {
    sections: Vec<Section>,
    voices: Vec<PlayingVoice>,
    begin: Instant,
    #[allow(dead_code)] end: Instant,
    current_section_index: usize,
}
impl PlayingMusic {

    pub fn new(sheet_music: SheetMusic) -> Self {
        let begin = Instant::now();
        PlayingMusic {
            begin,
            end: begin + music_duration(&sheet_music),
            sections: sheet_music.sections,
            voices: sheet_music.voices.into_iter().map(PlayingVoice::new).collect(),
            current_section_index: 0,
        }
    }

    pub fn next(&mut self) -> Reading {
        let time_elapsed = Instant::now() - self.begin;
        self.update_current_section_index(time_elapsed);
        let section = self.sections.get(self.current_section_index)
            .unwrap_or_else(|| panic!("No current section"));
        let commands =self.voices.iter_mut()
            .flat_map(|t| t.next_targeted(time_elapsed, section))
            .collect();
        Reading { commands, time_elapsed, section }
    }

    fn update_current_section_index(&mut self, elapsed_time: Duration) {
        self.current_section_index = (self.current_section_index..self.sections.len())
            .take_while(|i| self.sections.get(*i).map(|section| section.begin_time <= elapsed_time) == Some(true))
            .last().unwrap_or_else(|| panic!("Failed to find section"));
    }

    pub fn tick_duration(&self) -> Duration {
        self.sections.get(self.current_section_index)
            .unwrap_or_else(|| panic!("No current section"))
            .tick_duration
    }
}

pub type TargetedCommand = (Command, ChannelId);

struct PlayingVoice {
    voice: Voice,
    current_event_index: usize,
}
impl PlayingVoice {

    fn new(voice: Voice) -> Self {
        PlayingVoice { voice, current_event_index: 0 }
    }

    fn next_targeted(&mut self, elapsed_time: Duration, section: &Section) -> Vec<TargetedCommand> {
        self.next(elapsed_time, section).into_iter()
            .map(|c| (c, self.voice.instrument_id))
            .collect()
    }

    fn next(&mut self, elapsed_time: Duration, section: &Section) -> Vec<Command> {
        let begin = self.current_event_index;
        let result: Vec<Command> = self.voice.events.iter()
            .skip(begin)
            .take_while(|(_, t)| get_time(*t, section) <= elapsed_time)
            .map(|(cmd, _)| cmd.clone())
            .collect();
        self.current_event_index += result.len();
        result
    }

}

fn music_duration(music: &SheetMusic) -> Duration {
    let default_section = Section::default();
    let last_section = music.sections.last().unwrap_or(&default_section);
    get_time(music.end, last_section)
}

fn get_time(tick: Tick, section: &Section) -> Duration {
    let ticks = tick - section.begin_tick;
    let nanos = ticks as u64 * section.tick_duration.as_nanos() as u64;
    section.begin_time + Duration::from_nanos(nanos)
}

pub struct Reading<'a> {
    pub commands: Vec<TargetedCommand>,
    pub time_elapsed: Duration,
    pub section: &'a Section,
}
impl <'a> Reading<'a>  {
    pub fn measure(&self) -> MeasurePosition {
        self.section.measure_at_time(self.time_elapsed)
    }
}