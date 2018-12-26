extern crate hound;

use std::{i16, f32::consts::PI, io, fs};
use self::hound::{WavSpec, SampleFormat, WavWriter};

pub fn writer(sample_rate: u32, file_name: &str) -> WavWriter<io::BufWriter<fs::File>> {
    let path = "target/".to_owned() + file_name;
    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };
    WavWriter::create(path, spec).unwrap()
}
