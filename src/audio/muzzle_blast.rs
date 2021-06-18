use std::f32::consts::PI;
use std::time::Duration;

use rodio::Source;

use super::bezier::{CubicBezier, Node};

const SAMPLERATE: u32 = 48000;

#[derive(Clone)]
pub struct MuzzleBlast {
    delay: Duration,
    attack: Duration,
    positive_phase: Duration,
    negative_phase: Duration,
    tail: Duration,

    length: Duration,
    sample: u64,
    bezier: CubicBezier,
}

impl MuzzleBlast {
    pub fn new(length: Duration) -> Self {
        let delay = Duration::from_secs_f64(length.as_secs_f64() * 0.05);
        let attack = Duration::from_secs_f64(length.as_secs_f64() * 0.05);
        let positive_phase = Duration::from_secs_f64(length.as_secs_f64() * 0.5);
        let negative_phase = Duration::from_secs_f64(length.as_secs_f64() * 0.3);
        let tail = Duration::from_secs_f64(length.as_secs_f64() * 0.1);

        Self {
            delay,
            attack,
            positive_phase,
            negative_phase,
            tail,

            length,
            sample: 0,
            bezier: CubicBezier::new(
                Duration::from_secs_f64(length.as_secs_f64() * 0.0),
                Duration::from_secs_f64(length.as_secs_f64() * 1.0),
                Node::new(Duration::from_secs_f64(length.as_secs_f64() * 0.05), 0.0),
                Node::new(Duration::from_secs_f64(length.as_secs_f64() * 0.06), 0.9),
                Node::new(Duration::from_secs_f64(length.as_secs_f64() * 0.7), -0.1),
                Node::new(Duration::from_secs_f64(length.as_secs_f64() * 0.9), 0.0),
            ),
        }
    }

    fn synthesize(&self, time: Duration) -> Option<f32> {
        if time >= self.length {
            return None;
        }

        let a = self.bezier.amplitude(time);
        Some(a as f32)
    }
}

impl Iterator for MuzzleBlast {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.sample = self.sample.wrapping_add(1);
        let time = Duration::from_secs_f64((self.sample as f64) / (SAMPLERATE as f64));
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
        SAMPLERATE
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        Some(self.delay + self.attack + self.positive_phase + self.negative_phase + self.tail)
    }
}
