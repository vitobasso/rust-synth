
extern crate cpal;

use beep::cpal::{
    UnknownTypeOutputBuffer::{F32, I16, U16},
    StreamData::Output,
    OutputBuffer,
    Sample
};

pub trait WaveGen {
    fn next_sample(&self, clock: f32) -> f32;
}

pub fn beep<A: WaveGen + Sync>(wave: A) {
    let device = cpal::default_output_device().expect("Failed to get default output device");
    let format = device.default_output_format().expect("Failed to get default output format");
    let event_loop = cpal::EventLoop::new();
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id.clone());

    let sample_rate = format.sample_rate.0 as f32;
    let channels = format.channels as usize;

    let mut clock: f32 = 0f32;
    let mut next_sample = || {
        clock = (clock + 1.0) % sample_rate;
        let normalized_clock = clock/sample_rate;
        wave.next_sample(normalized_clock)
    };

    event_loop.run(move |_, data| {
        match data {
            Output { buffer: F32(buffer) } => feed_buffer(buffer, &mut next_sample, channels),
            Output { buffer: I16(buffer) } => feed_buffer(buffer, &mut next_sample, channels),
            Output { buffer: U16(buffer) } => feed_buffer(buffer, &mut next_sample, channels),
            _ => panic!("Unexpected buffer type."),
        }
    });
}

fn feed_buffer<T: SampleFromF32, F>(mut buffer: OutputBuffer<T>, next_sample: &mut F, channels: usize) -> ()
    where F: FnMut() -> f32 {
    for buff_chunks in buffer.chunks_mut(channels) {
        let value: T = T::from_f32(next_sample());
        for out in buff_chunks.iter_mut() {
            *out = value;
        }
    }
}

trait SampleFromF32: Sample {
    fn from_f32(value: f32) -> Self;
}
impl SampleFromF32 for f32 {
    fn from_f32(value: f32) -> Self {
        value
    }
}
impl SampleFromF32 for i16 {
    fn from_f32(f: f32) -> i16 {
        (f * std::i16::MAX as f32) as i16
    }
}
impl SampleFromF32 for u16 {
    fn from_f32(value: f32) -> u16 {
        ((value * 0.5 + 0.5) * std::u16::MAX as f32) as u16
    }
}
