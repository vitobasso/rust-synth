This is a synthesizer library written from scratch in Rust. 

It works by receiving commands, e.g. `NoteOn`, `NoteOff`, `SetPatch`, 
and producing a sound signal that can be sent to your computer's audio device to produce sound.


[rust-synth-gui](https://github.com/vitobasso/rust-synth-gui) lets you play it with your computer keyboard 
and is also an example on how to use the library.  

## Features
This is the progress so far
- [x] Oscillators
    - Sine, Saw, Square, Pulse, Supersaw
    - [ ] Noise
- [x] Filters
    - Biquad LPF, HPF, BPF, Notch
- Modulation
    - [x] ADSR
        - [ ] Filter ADSR
    - [x] LFO's
    - [x] Wire modulation to parameters
- Effects
    - [ ] Compression
    - [ ] Distortion
    - [ ] Delay
- [x] Polyphony
- [x] Arpeggiator
    - [x] Set beat
- [x] Loop recorder
    - [ ] Snap to measures
- [ ] Drums
- [x] Read Midi
