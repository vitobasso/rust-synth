# FIX
- crash when pitch too low: 'attempt to multiply with overflow', src/core/music_theory/pitch.rs:72:10

# LINKS
- Midi files: https://freemidi.org/
- Midi documentation
    - programs: https://en.wikipedia.org/wiki/General_MIDI#Percussion
    - meta events: https://www.csie.ntu.edu.tw/~r92092/ref/midi/
- MIDI to CSV on the command line: http://www.fourmilab.ch/webtools/midicsv/

# INSPIRATION
- [TonicAudio](https://github.com/TonicAudio/Tonic). C++ lib.
    - Api design: controllers, generators, proccessors
    - combine oscillators with +, *, ...

# TODO
- newtype instead of type aliases. 
- restrictive types for RecklessFloat, range 0-1 (rhythm)
- move midi -> preset mapping out of rust-synth
- From(usize) for PitchClass, Pitch