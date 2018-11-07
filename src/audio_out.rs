extern crate cpal;
use audio_out::cpal::{
    UnknownTypeOutputBuffer::{F32, I16, U16},
    StreamData::Output,
    OutputBuffer, Sample, Device, Format, EventLoop
};
use std::sync::mpsc::Receiver;

type MySample = f32;
pub fn play(device: &Device, format: &Format, sig_in: Receiver<MySample>) {

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

fn feed_buffer<T: SampleFromF32>(mut buffer: OutputBuffer<T>, sig_in: &Receiver<MySample>, channels: usize) -> () {
    for buff_chunks in buffer.chunks_mut(channels) {
        match sig_in.recv() {
            Ok(sample) =>
                for out in buff_chunks.iter_mut() {
                    *out = T::from_f32(sample);
                },
            _ => {
                println!("no samples to feed");
            }
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
