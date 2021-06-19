pub mod bezier;
mod blast_profile;
mod blast_wave;
pub mod friedlander_wave;
mod shock_wave;

pub use blast_profile::BlastProfile;
pub use blast_wave::*;

pub const SAMPLERATE: u32 = 48000;

#[test]
pub fn test_blast() {
    use hound;
    use std::i16;

    let blast_profile_heavy = BlastProfile::from_file("blast_profile_heavy");
    let blast_profile_light = BlastProfile::from_file("blast_profile_light");
    // Render it!
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 48000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    // Write to file
    let mut writer = hound::WavWriter::create("impact.wav", spec).unwrap();
    let amplitude = i16::MAX as f32;
    for t in BlastWave::new(&blast_profile_heavy).into_iter() {
        writer.write_sample((t * amplitude) as i16).unwrap();
    }

    // Play audio from buffer, which does not require the above "write to file" segment
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    sink.append(BlastWave::new(&blast_profile_heavy));
    sink.append(BlastWave::new(&blast_profile_light));
    sink.append(BlastWave::new(&blast_profile_light));
    sink.append(BlastWave::new(&blast_profile_light));
    sink.append(BlastWave::new(&blast_profile_heavy));
    sink.append(BlastWave::new(&blast_profile_heavy));
    sink.append(BlastWave::new(&blast_profile_light));
    sink.append(BlastWave::new(&blast_profile_heavy));
    sink.set_volume(0.5);
    sink.sleep_until_end();
}
