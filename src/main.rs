
mod beep;
use beep::WaveGen;

fn main() {

    struct Sine;
    impl WaveGen for Sine {
        fn next_sample(&self, clock: f32) -> f32 {
            (clock * 440.0 * 2.0 * std::f32::consts::PI).sin()
        }
    }

    struct Saw;
    impl WaveGen for Saw {
        fn next_sample(&self, clock: f32) -> f32 {
            ((clock * 440.0) % 1.0)
        }
    }

    beep::beep(Sine)
//    beep::beep(Saw)
}
