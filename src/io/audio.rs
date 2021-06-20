use cpal::{self, Device, SupportedStreamConfig, StreamConfig};
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use cpal::SampleFormat::{F32, U16, I16};
use crate::core::{music_theory::Hz, synth::Sample};

pub struct AudioOut {
    device: Device,
    config: SupportedStreamConfig,
}

impl AudioOut {
    pub fn initialize() -> Result<Self, String> {
        match cpal::default_host().default_output_device() {
            Some(device) =>
                device.default_output_config()
                    .map_err(|e| format!("Failed to get default output format. {:?}", e))
                    .map(|config| AudioOut { device, config }),
            None => Err("Failed to get default output device".to_string()),
        }
    }

    pub fn sample_rate(&self) -> Hz {
        Hz::from(self.config.sample_rate().0)
    }

    pub fn start(&self, next_sample: Box<dyn Fn() -> Sample + Send>) {
        match self.config.sample_format() {
            F32 => self.start_for::<f32>(next_sample),
            I16 => self.start_for::<i16>(next_sample),
            U16 => self.start_for::<u16>(next_sample),
        }
    }

    fn start_for<T>(&self, next_sample: Box<dyn Fn() -> Sample + Send>)
        where T: cpal::Sample,
    {
        let channels = self.config.channels() as usize;
        let stream = self.device.build_output_stream(
            &StreamConfig::from(self.config.clone()),
            move |buffer: &mut [T], _| write(buffer, channels, next_sample),
            |e| panic!("{:?}", e))
            .unwrap_or_else(|e| panic!("{:?}", e));
        stream.play().unwrap_or_else(|e| panic!("{:?}", e));
    }

}

fn write<T>(buffer: &mut [T], channels: usize, next_sample: Box<dyn Fn() -> Sample + Send>)
where T: cpal::Sample
{
    for chunk in buffer.chunks_mut(channels) {
        let sample = next_sample() as f32;
        let cpal_sample: T = cpal::Sample::from(&sample);
        for place in chunk.iter_mut() {
            *place = cpal_sample;
        }
    }
}

