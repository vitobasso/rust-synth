use crate::core::music_theory::rhythm::{NoteDuration, Note};
use crate::core::music_theory::diatonic_scale::ScaleDegree::{self, *};
use crate::core::music_theory::diatonic_scale::{OctaveShift, RelativePitch};
use num_traits::FromPrimitive;

#[derive(Clone, PartialEq, Debug)]
pub enum Chord {
    Octaves, Triad, Fantasy, Tetra, Penta
}

#[derive(Clone, PartialEq, Debug)]
pub enum Direction {
    Up, Down, UpDown
}

#[derive(Clone, PartialEq, Debug)]
pub struct Specs {
    pub chord: Chord,
    pub direction: Direction,
    pub octave_min: OctaveShift,
    pub octave_max: OctaveShift,
    pub duration: NoteDuration,
}

pub fn notes(specs: Specs) -> Vec<Note> {
    let rising = notes_rising(&specs);
    let towards_direction_once = match specs.direction {
        Direction::Up => rising,
        Direction::Down => rising.into_iter().rev().collect(),
        Direction::UpDown => rising.clone().into_iter()
            .chain(rising.into_iter().skip(1).rev().skip(1)).collect(),
    };
    towards_direction_once.into_iter().collect()
}

fn notes_rising(specs: &Specs) -> Vec<Note> {
    let chord_degrees = specs.chord.notes();
    let pitches: Vec<RelativePitch> = octaves(specs.octave_min, specs.octave_max).into_iter()
        .flat_map(|octave| chord_degrees.iter().map(move |degree| (octave, *degree)))
        .collect();
    pitches.into_iter()
        .map(|pitch| Note { duration: specs.duration, pitch })
        .collect()
}

impl Chord {
    fn notes(&self) -> Vec<ScaleDegree> {
        match self {
            Chord::Octaves => vec![I1],
            Chord::Triad => vec![I1, I3, I5],
            Chord::Fantasy => vec![I1, I2, I3, I5],
            Chord::Tetra => vec![I1, I3, I5, I7],
            Chord::Penta => vec![I1, I2, I3, I5, I6],
        }
    }
}

fn octaves(min: OctaveShift, max: OctaveShift) -> Vec<OctaveShift> {
    (min as isize ..= max as isize)
        .map(|num| FromPrimitive::from_isize(num)).flatten().collect()
}

impl Default for Chord {
    fn default() -> Self {
        Chord::Octaves
    }
}
impl Default for Direction {
    fn default() -> Self {
        Direction::Up
    }
}

impl Default for Specs {
    fn default() -> Self {
        Specs {
            chord: Default::default(),
            direction: Default::default(),
            octave_min: OctaveShift::Down1,
            octave_max: OctaveShift::Up1,
            duration: Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use NoteDuration::*;
    use OctaveShift::*;

    #[test]
    fn octaves_up() {
        let specs = Specs {
            chord: Chord::Octaves,
            direction: Direction::Up,
            octave_min: Same,
            octave_max: Up2,
            duration: Whole
        };

        assert_eq!(notes(specs), vec!(
            Note::new(Whole, Same, I1),
            Note::new(Whole, Up1, I1),
            Note::new(Whole, Up2, I1),
        ))
    }

    #[test]
    fn triad() {
        let specs = Specs {
            chord: Chord::Triad,
            direction: Direction::Up,
            octave_min: Same,
            octave_max: Up2,
            duration: Whole
        };

        assert_eq!(notes(specs), vec!(
            Note::new(Whole, Same, I1),
            Note::new(Whole, Same, I3),
            Note::new(Whole, Same, I5),
            Note::new(Whole, Up1, I1),
            Note::new(Whole, Up1, I3),
            Note::new(Whole, Up1, I5),
            Note::new(Whole, Up2, I1),
            Note::new(Whole, Up2, I3),
            Note::new(Whole, Up2, I5),
        ))
    }

    #[test]
    fn octave_down() {
        let specs = Specs {
            chord: Chord::Octaves,
            direction: Direction::Up,
            octave_min: Down1,
            octave_max: Up1,
            duration: Whole
        };

        assert_eq!(notes(specs), vec!(
            Note::new(Whole, Down1, I1),
            Note::new(Whole, Same, I1),
            Note::new(Whole, Up1, I1),
        ))
    }

    #[test]
    fn direction_down() {
        let specs = Specs {
            chord: Chord::Octaves,
            direction: Direction::Down,
            octave_min: Same,
            octave_max: Up2,
            duration: Whole
        };

        assert_eq!(notes(specs), vec!(
            Note::new(Whole, Up2, I1),
            Note::new(Whole, Up1, I1),
            Note::new(Whole, Same, I1),
        ))
    }

    #[test]
    fn direction_up_down() {
        let specs = Specs {
            chord: Chord::Octaves,
            direction: Direction::UpDown,
            octave_min: Same,
            octave_max: Up2,
            duration: Whole
        };

        assert_eq!(notes(specs), vec!(
            Note::new(Whole, Same, I1),
            Note::new(Whole, Up1, I1),
            Note::new(Whole, Up2, I1),
            Note::new(Whole, Up1, I1),
        ))
    }

    #[test]
    fn duration_half_more_octaves() {
        let specs = Specs {
            chord: Chord::Octaves,
            direction: Direction::UpDown,
            octave_min: Down1,
            octave_max: Up2,
            duration: Half
        };

        assert_eq!(notes(specs), vec!(
            Note::new(Half, Down1, I1),
            Note::new(Half, Same, I1),
            Note::new(Half, Up1, I1),
            Note::new(Half, Up2, I1),
            Note::new(Half, Up1, I1),
            Note::new(Half, Same, I1),
        ))
    }

    #[test]
    fn mixed() {
        let specs = Specs {
            chord: Chord::Triad,
            direction: Direction::UpDown,
            octave_min: Down1,
            octave_max: Same,
            duration: Half
        };

        assert_eq!(notes(specs), vec!(
            Note::new(Half, Down1, I1),
            Note::new(Half, Down1, I3),
            Note::new(Half, Down1, I5),
            Note::new(Half, Same, I1),
            Note::new(Half, Same, I3),
            Note::new(Half, Same, I5),
            Note::new(Half, Same, I3),
            Note::new(Half, Same, I1),
            Note::new(Half, Down1, I5),
            Note::new(Half, Down1, I3),
        ))
    }


}