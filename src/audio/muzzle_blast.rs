use std::f32::consts::PI;
use std::time::Duration;

use rodio::Source;

use super::bezier::{CubicBezier, Node};

#[derive(Clone)]
pub struct MuzzleBlast {
    freq: f32, // Base frequency
    ppd: u64,  // Positive phase duration
    npd: u64,  // Negative phase duration
    peak: f32, // Peak overpressure
    low: f32,
    delay: u64, // Initial delay before blast
    attack: u64, 
    tail: u64,  // How long the blast takes to end
    speed: f64, // Playback speed (1 is realtime, 0.5 is half-speed)

    //Baked values
    sample: usize,     // Current sample
    length: u64,       // Total duration,
    time_factor: f64,  // bitrate / speed
    pitch_factor: f32, // bitrate / speed
    pi_freq_time: f32, // PI * self.freq * self.time_factor
    bezier: CubicBezier,
}

impl MuzzleBlast {
    pub fn new(
        base_frequency: f32,
        delay: u64,
        attack: u64,
        positive_phase_duration: u64,
        negative_phase_duration: u64,
        tail: u64,
        speed: f64,
    ) -> Self {
        let time_factor = 48.0 / speed;

        Self {
            freq: base_frequency,
            ppd: positive_phase_duration,
            npd: negative_phase_duration,
            attack,
            peak: 0.8,
            low: -0.2,
            delay,
            tail,
            speed,

            sample: 0,
            length: delay + attack + positive_phase_duration + negative_phase_duration + tail,
            time_factor,
            pitch_factor: 48000.0,
            pi_freq_time: PI * base_frequency / time_factor as f32,
            bezier: CubicBezier::new(
                delay,
                delay + attack + positive_phase_duration + negative_phase_duration,
                Node::new(delay, 0.0),
                Node::new(delay + attack, 0.75),
                Node::new(delay + attack + positive_phase_duration, -0.2),
                Node::new(delay + attack + positive_phase_duration + negative_phase_duration,0.0)
            ),
        }
    }

    fn base_sine_generator(&self, freq: f32) -> f32 {
        let mut sine: f32 = 0.0;
        let mut base = freq;

        // Base sin synthesis- the core fundemental frequencies to be used
        for i in 1..4 {
            base = base * PI * i as f32;
            let freq = base as f32 * (self.sample + i) as f32 * self.pi_freq_time;
            sine += freq.sin() * 0.2f32.powi(i as i32);
        }
        return sine;
    }

    fn synthesize(&self, time: f64) -> Option<f32> {
        if (time as u64) < self.length {
            let amp = self.bezier.amplitude(Duration::from_secs_f64(time));
            Some(amp as f32)
        } else {
            None
        }
    }

    fn synthesize_old(&self, time: f64) -> Option<f32> {
        let sine = self.base_sine_generator(0.000104);

        if (time as u64) < self.delay {
            // Pre-blast
            return Some(0.0);
        } else if (time as u64) < self.delay + self.ppd {
            // Positive phase
            let percent = 1.0 - (time as f32 - self.delay as f32) / self.ppd as f32;
            let lerped = self.low + (self.peak - self.low) * percent;

            return Some(lerped + sine);
        } else if (time as u64) < self.delay + self.ppd + self.npd {
            // Negative phase

            let percent = (time as f32 - self.delay as f32 - self.ppd as f32) / self.npd as f32;
            let lerped = self.low - self.low * percent;
            let lerped_sine_amplitude = (1.0 - percent) * sine * 0.5;

            return Some(lerped + lerped_sine_amplitude);
        } else if (time as u64) < self.delay + self.ppd + self.npd + self.tail {
            return Some(sine * 0.0002);
        } else {
            println!("Last frame = {:?}", time);
            return None;
        }
    }
}

impl Iterator for MuzzleBlast {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.sample = self.sample.wrapping_add(1);
        let time = self.sample as f64 / self.time_factor;
        return self.synthesize(time);
    }
}

impl Source for MuzzleBlast {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        48000
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        let time = (self.length) as f64 / self.speed;
        let duration = Duration::from_millis(time as u64);
        Some(duration)
    }
}
