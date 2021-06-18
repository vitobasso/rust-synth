use cpal;
use self::cpal::{
    UnknownTypeOutputBuffer::{F32, I16, U16},
    StreamData::Output,
    OutputBuffer, Device, Format, EventLoop
};
use std::sync::mpsc::Receiver;
use crate::core::{music_theory::Hz, synth::Sample, control::Millis};

const LATENCY: Millis = 250;

pub struct Out {
    device: Device,
    format: Format,
}

impl Out {
    pub fn initialize() -> Result<Self, String> {
        match cpal::default_output_device() {
            Some(device) =>
                device.default_output_format()
                    .map_err(|e| format!("Failed to get default output format. {:?}", e))
                    .map(|format| Out { device, format }),
            None => Err("Failed to get default output device".to_string()),
        }
    }

    pub fn sample_rate(&self) -> Hz {
        Hz::from(self.format.sample_rate.0)
    }

    pub fn buffer_size(&self) -> usize {
        self.sample_rate() as usize / LATENCY as usize
    }

    pub fn start(&self, sound_in: Receiver<Sample>) {
        start(&self.device, &self.format, sound_in)
    }
}

fn start(device: &Device, format: &Format, sound_in: Receiver<Sample>) {
    let channels = format.channels as usize;
    let event_loop = EventLoop::new();
    let stream_id = event_loop.build_output_stream(device, format).unwrap();
    event_loop.play_stream(stream_id);

    event_loop.run(move |_, data| {
        match data {
            Output { buffer: F32(buffer) } => feed_buffer(buffer, &sound_in, channels),
            Output { buffer: I16(buffer) } => feed_buffer(buffer, &sound_in, channels),
            Output { buffer: U16(buffer) } => feed_buffer(buffer, &sound_in, channels),
            _ => panic!("Unexpected buffer type."),
        }
    });
}

fn feed_buffer<T: SampleFromF64>(mut buffer: OutputBuffer<'_, T>, sig_in: &Receiver<Sample>, channels: usize) {
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
        (value * f64::from(f64::MAX)) as i16
    }
}
impl SampleFromF64 for u16 {
    fn from_f64(value: f64) -> u16 {
        ((value * 0.5 + 0.5) * f64::from(u16::MAX)) as u16
    }
}
