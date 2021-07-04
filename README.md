I'm writing this synthesizer library from scratch as an excuse to learn Rust.  
It's definitely not stable. There's quite a variety of rough features I've been slowly polishing and completing. 

It works by receiving commands, e.g. `NoteOn`, `NoteOff`, `SetPatch`, 
and producing a sound signal that can be sent to your computer's audio output device.

An example of usage and runnable demo can be found in [rust-synth-gui](https://github.com/vitobasso/rust-synth-gui).  

## Features
This is the progress so far:
- Synth
  - Oscillators
      - [x] Sine, Saw, Square, Pulse
      - [x] Mix of detuned oscillators
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
- [x] State accessible for visualization
