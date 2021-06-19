This is a synthesizer library written from scratch in Rust. 

It works by receiving commands, e.g. `NoteOn`, `NoteOff`, `SetPatch`, 
and producing a sound signal that can be sent to your computer's audio device to produce sound.

An example of usage can be found in [rust-synth-gui](https://github.com/vitobasso/rust-synth-gui).  

## Features
This is the progress so far
- Synth
  - Oscillators
      - [x] Sine, Saw, Square, Pulse, Supersaw
      - [ ] Noise
  - Filters
      - [x] Biquad LPF, HPF, BPF, Notch
  - Modulation
      - [x] ADSR
      - [x] LFO's
      - [x] Wire modulation to parameters
  - [x] Polyphony
- Effects
    - [ ] Compression
    - [ ] Distortion
    - [ ] Delay
- Tools
  - [x] Arpeggiator
      - [x] Tap tempo
  - [x] Loop recorder
      - [ ] Snap to measures
- [ ] Drums
- [x] Read Midi
