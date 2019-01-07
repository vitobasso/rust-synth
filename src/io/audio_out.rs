extern crate cpal;
use audio_out::cpal::{
    UnknownTypeOutputBuffer::{F32, I16, U16},
    StreamData::Output,
    OutputBuffer, Device, Format, EventLoop
};
use std::sync::mpsc::Receiver;

type Sample = f64;
pub fn loop_forever(device: &Device, format: &Format, sig_in: Receiver<Sample>) {

    let channels = format.channels as usize;
    let event_loop = EventLoop::new();
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id.clone());

    event_loop.run(move |_, data| {
        match data {
            Output { buffer: F32(buffer) } => feed_buffer(buffer, &sig_in, channels),
            Output { buffer: I16(buffer) } => feed_buffer(buffer, &sig_in, channels),
            Output { buffer: U16(buffer) } => feed_buffer(buffer, &sig_in, channels),
            _ => panic!("Unexpected buffer type."),
        }
    });
}

fn feed_buffer<T: SampleFromF64>(mut buffer: OutputBuffer<T>, sig_in: &Receiver<Sample>, channels: usize) {
    for buff_chunks in buffer.chunks_mut(channels) {
        match sig_in.recv() {
            Ok(sample) =>
                for out in buff_chunks.iter_mut() {
                    *out = T::from_f64(sample);
                },
            _ => {
                panic!("Sample channel hang up?");
            }
        }
    }
}

trait SampleFromF64: cpal::Sample {
    fn from_f64(value: f64) -> Self;
}
impl SampleFromF64 for f32 {
    fn from_f64(value: f64) -> Self {
        value as f32
    }
}
impl SampleFromF64 for i16 {
    fn from_f64(value: f64) -> i16 {
        (value * std::i16::MAX as f64) as i16
    }
}
impl SampleFromF64 for u16 {
    fn from_f64(value: f64) -> u16 {
        ((value * 0.5 + 0.5) * std::u16::MAX as f64) as u16
    }
}
