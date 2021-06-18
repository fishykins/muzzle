mod muzzle_blast;
mod shock_wave;
pub mod bezier;
pub mod friedlander_wave;

pub use muzzle_blast::MuzzleBlast;

use hound;
use std::time::Duration;
use rodio::Source;
use std::f32::consts::PI;
use std::i16;

#[test]
pub fn test_audio() {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    let _source = rodio::source::SineWave::new(150);
    let muzzleblast = MuzzleBlast::new(Duration::from_millis(1000));
    let duration = muzzleblast.total_duration().unwrap();
    println!("muzzle length is {:?}", duration);
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    sink.append(muzzleblast.clone()); //beep!
    sink.set_volume(0.5); 

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 48000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("gunshot.wav", spec).unwrap();
    let amplitude = i16::MAX as f32;
    for t in muzzleblast.into_iter() {
        writer.write_sample((t * amplitude) as i16).unwrap();
    }
    std::thread::sleep(duration);
}
