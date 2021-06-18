

pub struct FriedlanderWave {
    peak_preasure: f32, 
    delay: f32, 
    positive_phase_duration: f32
}

impl FriedlanderWave {
    pub fn new(delay: f32, peak: f32, ppd: f32) -> Self {
        Self {
            delay,
            peak_preasure: peak,
            positive_phase_duration: ppd,
        }
    }

    pub fn preasure(&self, t: f32) -> f32 {
        let e: f32 = 2.0;
        let p = self.peak_preasure * (1.0 - (t - self.delay) / self.positive_phase_duration) * e.powf(-(t - self.delay)/self.positive_phase_duration);
        return p;
    }
}