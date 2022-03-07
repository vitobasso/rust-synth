A sound synthesiser built from scratch. It implements oscillators, filters, an arpeggiator and a looper. It's written in Rust, a systems programming language, due to the performance critical nature of producing sound in real time.

https://user-images.githubusercontent.com/1895014/157036647-2696a3bb-1ed7-4cfc-8a3c-401b13763e3d.mp4


It works by receiving commands such as `NoteOn`, `NoteOff`and `SetPatch` which could originate from user interaction or MIDI protocol instructions, for example,
and producing a sound signal that can be sent to an audio output device.

An example of usage and runnable demo can be found in [rust-synth-gui](https://github.com/vitobasso/rust-synth-gui). 

## Progress
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
