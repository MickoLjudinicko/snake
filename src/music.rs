use crate::sound;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// Standard note durations in milliseconds (assuming a tempo of 120 BPM)
pub const WHOLE: u64 = 1000;
pub const QUARTER: u64 = 250;
pub const EIGHTH: u64 = 125;
pub const SIXTEENTH: u64 = 63;

// Triplet note durations
pub const QUARTER_TRIPLET: u64 = (QUARTER as f64 * (2.0 / 3.0)) as u64;
pub const SIXTEENTH_TRIPLET: u64 = (SIXTEENTH as f64 * (2.0 / 3.0)) as u64;

/// Represents the name of a musical note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NoteName {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

/// Represents a musical note with a name, octave, and duration.
#[derive(Debug, Clone, Copy)]
pub struct Note {
    pub name: NoteName,
    pub octave: i8,    // Octave number, e.g., 4 for the 4th octave
    pub duration: u64, // Duration in milliseconds
}

impl Note {
    pub fn frequency(&self) -> f32 {
        let a4_frequency = 440.0;
        let a4_note_number = 69;

        let note_number = self.midi_note_number();

        a4_frequency * 2f32.powf((note_number as f32 - a4_note_number as f32) / 12.0)
    }

    fn midi_note_number(&self) -> i32 {
        let note_index = match self.name {
            NoteName::C => 0,
            NoteName::CSharp => 1,
            NoteName::D => 2,
            NoteName::DSharp => 3,
            NoteName::E => 4,
            NoteName::F => 5,
            NoteName::FSharp => 6,
            NoteName::G => 7,
            NoteName::GSharp => 8,
            NoteName::A => 9,
            NoteName::ASharp => 10,
            NoteName::B => 11,
        };

        let octave_offset = (self.octave as i32 + 1) * 12; // MIDI octave starts from -1
        note_index + octave_offset
    }
}

// Represents an element in the music sequence: either a Note or a Rest
pub enum MusicElement {
    Note(Note),
    Rest { duration: u64 },
}

/// Plays a sequence of music elements in a loop until the stop signal is received.
pub fn play_music(elements: Vec<MusicElement>, stop_signal: Arc<AtomicBool>) {
    while !stop_signal.load(Ordering::SeqCst) {
        for element in &elements {
            if stop_signal.load(Ordering::SeqCst) {
                break;
            }
            match element {
                MusicElement::Note(note) => {
                    let frequency = note.frequency();

                    sound::play_tone(frequency as u32, note.duration)
                }
                MusicElement::Rest { duration } => {
                    thread::sleep(Duration::from_millis(*duration));
                }
            }
        }
    }
}

/// Returns a vector of music elements representing the game's theme music.
pub fn game_theme() -> Vec<MusicElement> {
    vec![
        // 1st Measure
        MusicElement::Note(Note {
            name: NoteName::C,
            octave: 4,
            duration: SIXTEENTH,
        }),
        MusicElement::Note(Note {
            name: NoteName::C,
            octave: 5,
            duration: SIXTEENTH,
        }),
        MusicElement::Note(Note {
            name: NoteName::A,
            octave: 3,
            duration: SIXTEENTH,
        }),
        MusicElement::Note(Note {
            name: NoteName::A,
            octave: 4,
            duration: SIXTEENTH,
        }),
        MusicElement::Note(Note {
            name: NoteName::ASharp,
            octave: 3,
            duration: SIXTEENTH,
        }),
        MusicElement::Note(Note {
            name: NoteName::ASharp,
            octave: 4,
            duration: SIXTEENTH,
        }),
        MusicElement::Rest { duration: EIGHTH },
        MusicElement::Rest { duration: QUARTER },
        // 2nd Measure
        MusicElement::Note(Note {
            name: NoteName::C,
            octave: 4,
            duration: SIXTEENTH,
        }),
        MusicElement::Note(Note {
            name: NoteName::C,
            octave: 5,
            duration: SIXTEENTH,
        }),
        MusicElement::Note(Note {
            name: NoteName::A,
            octave: 3,
            duration: SIXTEENTH,
        }),
        MusicElement::Note(Note {
            name: NoteName::A,
            octave: 4,
            duration: SIXTEENTH,
        }),
        MusicElement::Note(Note {
            name: NoteName::ASharp,
            octave: 3,
            duration: SIXTEENTH,
        }),
        MusicElement::Note(Note {
            name: NoteName::ASharp,
            octave: 4,
            duration: SIXTEENTH,
        }),
        MusicElement::Rest { duration: EIGHTH },
        MusicElement::Rest { duration: QUARTER },
        // 3rd Measure
        MusicElement::Note(Note {
            name: NoteName::F,
            octave: 3,
            duration: SIXTEENTH,
        }),
        MusicElement::Note(Note {
            name: NoteName::F,
            octave: 4,
            duration: SIXTEENTH,
        }),
        MusicElement::Note(Note {
            name: NoteName::D,
            octave: 3,
            duration: SIXTEENTH,
        }),
        MusicElement::Note(Note {
            name: NoteName::D,
            octave: 4,
            duration: SIXTEENTH,
        }),
        MusicElement::Note(Note {
            name: NoteName::DSharp,
            octave: 3,
            duration: SIXTEENTH,
        }),
        MusicElement::Note(Note {
            name: NoteName::DSharp,
            octave: 4,
            duration: SIXTEENTH,
        }),
        MusicElement::Rest { duration: EIGHTH },
        MusicElement::Rest { duration: QUARTER },
        // 4th Measure
        MusicElement::Note(Note {
            name: NoteName::F,
            octave: 3,
            duration: SIXTEENTH,
        }),
        MusicElement::Note(Note {
            name: NoteName::F,
            octave: 4,
            duration: SIXTEENTH,
        }),
        MusicElement::Note(Note {
            name: NoteName::D,
            octave: 3,
            duration: SIXTEENTH,
        }),
        MusicElement::Note(Note {
            name: NoteName::D,
            octave: 4,
            duration: SIXTEENTH,
        }),
        MusicElement::Note(Note {
            name: NoteName::DSharp,
            octave: 3,
            duration: SIXTEENTH,
        }),
        MusicElement::Note(Note {
            name: NoteName::DSharp,
            octave: 4,
            duration: SIXTEENTH,
        }),
        MusicElement::Rest { duration: EIGHTH },
        MusicElement::Rest { duration: QUARTER },
        MusicElement::Note(Note {
            name: NoteName::DSharp,
            octave: 4,
            duration: SIXTEENTH_TRIPLET,
        }),
        MusicElement::Note(Note {
            name: NoteName::D,
            octave: 4,
            duration: SIXTEENTH_TRIPLET,
        }),
        MusicElement::Note(Note {
            name: NoteName::CSharp,
            octave: 4,
            duration: SIXTEENTH_TRIPLET,
        }),
        // 5th Measure
        MusicElement::Note(Note {
            name: NoteName::C,
            octave: 4,
            duration: QUARTER,
        }),
        MusicElement::Note(Note {
            name: NoteName::DSharp,
            octave: 4,
            duration: QUARTER,
        }),
        MusicElement::Note(Note {
            name: NoteName::D,
            octave: 4,
            duration: QUARTER,
        }),
        MusicElement::Note(Note {
            name: NoteName::GSharp,
            octave: 3,
            duration: QUARTER,
        }),
        MusicElement::Note(Note {
            name: NoteName::G,
            octave: 3,
            duration: QUARTER,
        }),
        MusicElement::Note(Note {
            name: NoteName::CSharp,
            octave: 4,
            duration: QUARTER,
        }),
        // 6th Measure
        MusicElement::Note(Note {
            name: NoteName::C,
            octave: 4,
            duration: SIXTEENTH_TRIPLET,
        }),
        MusicElement::Note(Note {
            name: NoteName::FSharp,
            octave: 4,
            duration: SIXTEENTH_TRIPLET,
        }),
        MusicElement::Note(Note {
            name: NoteName::F,
            octave: 4,
            duration: SIXTEENTH_TRIPLET,
        }),
        MusicElement::Note(Note {
            name: NoteName::E,
            octave: 4,
            duration: SIXTEENTH_TRIPLET,
        }),
        MusicElement::Note(Note {
            name: NoteName::ASharp,
            octave: 4,
            duration: SIXTEENTH_TRIPLET,
        }),
        MusicElement::Note(Note {
            name: NoteName::A,
            octave: 4,
            duration: SIXTEENTH_TRIPLET,
        }),
        MusicElement::Note(Note {
            name: NoteName::GSharp,
            octave: 4,
            duration: QUARTER_TRIPLET,
        }),
        MusicElement::Note(Note {
            name: NoteName::DSharp,
            octave: 4,
            duration: QUARTER_TRIPLET,
        }),
        MusicElement::Note(Note {
            name: NoteName::B,
            octave: 3,
            duration: QUARTER_TRIPLET,
        }),
        MusicElement::Note(Note {
            name: NoteName::ASharp,
            octave: 3,
            duration: QUARTER_TRIPLET,
        }),
        MusicElement::Note(Note {
            name: NoteName::A,
            octave: 3,
            duration: QUARTER_TRIPLET,
        }),
        MusicElement::Note(Note {
            name: NoteName::GSharp,
            octave: 3,
            duration: QUARTER_TRIPLET,
        }),
        MusicElement::Rest { duration: WHOLE },
    ]
}
