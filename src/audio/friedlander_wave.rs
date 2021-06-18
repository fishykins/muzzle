

#[derive(Clone)]
pub struct FriedlanderWave {
    pub delay: f32, 
    pub positive_phase_duration: f32,
    peak_preasure: f32, 
    /// The agressiveness of the curve. Values between 1 and 2 seem to work best
    curve: f32,
    /// Multiplyer for time: high attack = quick wave
    frequency_mod: f32,
}

impl FriedlanderWave {
    pub fn new(delay: f32, peak: f32, ppd: f32, curve: f32) -> Self {
        Self {
            delay,
            peak_preasure: peak,
            positive_phase_duration: ppd,
            curve,
            frequency_mod: 8.0,
        }
    }

    pub fn preasure(&self, time: f32) -> f32 {
        let t = time * self.frequency_mod;
        self.peak_preasure * (1.0 - (t - self.delay) / self.positive_phase_duration) * self.curve.powf(-(t - self.delay)/self.positive_phase_duration)
    }
}