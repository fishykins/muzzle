pub mod bezier;
pub mod friedlander_wave;
mod blast;
mod shock_wave;

pub use blast::*;

pub const SAMPLERATE: u32 = 48000;

#[test]
pub fn test_audio() {
    use hound;
    use std::i16;
    use std::time::Duration;
    use rodio::Source;

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    let _source = rodio::source::SineWave::new(150);
    let muzzleblast = Blast::new(
        Duration::from_millis(36),
        0.3,
        Duration::from_millis(60),
        1.4,
        vec![
            (HarmonicBand {frequency: 49.0, width: 4.0, amplitude: 0.5, weight: 20, diffraction: 2000}, HarmonicGeneration::Cumulative),
            (HarmonicBand {frequency: 120.0, width: 12.0, amplitude: 0.3, weight: 6, diffraction: 1500}, HarmonicGeneration::Cumulative),
            (HarmonicBand {frequency: 600.0, width: 12.0, amplitude: 0.1, weight: 5, diffraction: 250}, HarmonicGeneration::Cumulative),
            (HarmonicBand {frequency: 800.0, width: 5.0, amplitude: 0.2, weight: 2, diffraction: 300}, HarmonicGeneration::Random),
            (HarmonicBand {frequency: 1600.0, width: 4.0, amplitude: 0.2, weight: 2, diffraction: 500}, HarmonicGeneration::Random),
        ],
        32
    );
    
    // Render it!
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 48000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("impact.wav", spec).unwrap();
    let amplitude = i16::MAX as f32;

    for t in muzzleblast.clone().into_iter() {
        writer.write_sample((t * amplitude) as i16).unwrap();
    }

    // Play audio!!
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    sink.append(muzzleblast.clone());
    sink.set_volume(0.5);
    let duration = muzzleblast.total_duration();
    println!("Blast is {:?} long", duration);
    std::thread::sleep(duration.unwrap());
}
