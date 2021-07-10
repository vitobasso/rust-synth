use crate::util::range_map::CyclicRangeMap;
use crate::core::music_theory::rhythm::{Note, NoteDuration};
use crate::core::tools::arpeggiator::builder;
use crate::core::sheet_music::sheet_music::MeasurePosition;

/// Range is valid between 0 and 1. TODO restrictive newtype
/// Values outside this range will result in an empty `Vec`.
#[derive(Clone, PartialEq, Default, Debug)]
pub struct Phrase {
    map: CyclicRangeMap<Note>,
}

const MEASURE_DURATION: f64 = NoteDuration::Whole as u8 as f64;

impl Phrase {

    pub fn from_specs(specs: builder::Specs) -> Phrase {
        Phrase::new(&builder::notes(specs))
    }

    pub fn new(notes: &[Note]) -> Self {
        let pairs: Vec<(f64, Note)> = notes.iter()
            .scan(0., |progress, note| {
                let index = *progress;
                *progress += note.duration as u8 as f64 / MEASURE_DURATION;
                Some((index, *note))
            }).collect();
        let total_duration: f64 = notes.iter().map(|n| n.duration as u8 as f64).sum();
        let total_measures: f64 = total_duration / MEASURE_DURATION;
        Phrase { map: CyclicRangeMap::new(pairs, total_measures) }
    }

    pub fn range(&self, from: f64, to: f64) -> Vec<Note> {
        self.map.range(from, to).into_iter().cloned().collect()
    }

    pub fn view(&self) -> View {
        View {
            notes: self.map.full_cycle().into_iter().cloned().collect(),
            length: self.map.end(),
        }

    }
}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct View {
    pub notes: Vec<Note>,
    pub length: MeasurePosition,
}
