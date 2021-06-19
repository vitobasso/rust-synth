use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;
use cpal::{self, Device, SupportedStreamConfig, StreamConfig};
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use cpal::SampleFormat::{F32, U16, I16};
use crate::core::{music_theory::Hz, synth::Sample, tools::Millis};

const LATENCY: Millis = 250;

pub struct Out {
    device: Device,
    config: SupportedStreamConfig,
}

impl Out {
    pub fn initialize() -> Result<Self, String> {
        match cpal::default_host().default_output_device() {
            Some(device) =>
                device.default_output_config()
                    .map_err(|e| format!("Failed to get default output format. {:?}", e))
                    .map(|config| Out { device, config }),
            None => Err("Failed to get default output device".to_string()),
        }
    }

    pub fn sample_rate(&self) -> Hz {
        Hz::from(self.config.sample_rate().0)
    }

    pub fn buffer_size(&self) -> usize {
        self.sample_rate() as usize / LATENCY as usize
    }

    pub fn start(&self, sound_in: Receiver<Sample>) {
        match self.config.sample_format() {
            F32 => self.start_for::<f32>(sound_in),
            I16 => self.start_for::<i16>(sound_in),
            U16 => self.start_for::<u16>(sound_in),
        }
    }

    fn start_for<T>(&self, sound_in: Receiver<Sample>)
        where T: cpal::Sample
    {
        let channels = self.config.channels() as usize;
        let stream = self.device.build_output_stream(
            &StreamConfig::from(self.config.clone()),
            move |buffer: &mut [T], _| write(buffer, channels, &sound_in),
            |e| panic!("{:?}", e))
            .unwrap_or_else(|e| panic!("{:?}", e));
        stream.play().unwrap_or_else(|e| panic!("{:?}", e));
        thread::sleep(Duration::new(u64::MAX, 0))
        //TODO instead return stream without thread ?
        //TODO so if I don't need a thread maybe I don't need a Receiver anymore ..
    }

}

fn write<T>(buffer: &mut [T], channels: usize, sound_in: &Receiver<Sample>)
where T: cpal::Sample
{
    for chunk in buffer.chunks_mut(channels) {
        let sample = sound_in.recv().unwrap_or_else(|e| panic!("{}", e)) as f32;
        let cpal_sample: T = cpal::Sample::from(&sample);
        for place in chunk.iter_mut() {
            *place = cpal_sample;
        }
    }
}

