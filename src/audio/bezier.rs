use std::time::Duration;

#[derive(Clone)]
pub struct CubicBezier {
    nodes: (Node, Node, Node, Node),
    start: Duration,
    end: Duration,
}

#[derive(Clone)]
pub struct Node {
    pub x: Duration,
    pub amplitude: f64,
}

impl Node{
    pub fn new(x: u64, amplitude: f64) -> Self {
        Self {
            x: Duration::from_millis(x),
            amplitude,
        }
    }
}

impl CubicBezier {
    pub fn new(start: u64, end: u64, a: Node, b: Node, c: Node, d: Node) -> Self {
        Self {
            start: Duration::from_millis(start),
            end: Duration::from_millis(end),
            nodes: (a, b, c, d),
        }
    }

    pub fn amplitude(&self, time: Duration) -> f64 {
        if time.as_secs_f64() <= self.nodes.0.x.as_secs_f64() {
            return self.nodes.0.amplitude
        } else if time.as_secs_f64() >= self.nodes.3.x.as_secs_f64() {
            return self.nodes.3.amplitude
        }

        // calculate t
        let t = time.as_secs_f64() / (self.end.as_secs_f64() - self.start.as_secs_f64());
        
        // let x = 
        //     self.nodes.0.x.as_secs_f64() * (1.0 - t).powi(3) + 
        //     self.nodes.1.x.as_secs_f64() * 3.0 * t * (1.0 - t).powi(2) +
        //     self.nodes.2.x.as_secs_f64() * 3.0 * t * (t).powi(2) * (1.0 - t) +
        //     self.nodes.3.x.as_secs_f64() * t.powi(3);

        let y = 
            self.nodes.0.amplitude * (1.0 - t).powi(3) + 
            self.nodes.1.amplitude * 3.0 * t * (1.0 - t).powi(2) +
            self.nodes.2.amplitude * 3.0 * t * (t).powi(2) * (1.0 - t) +
            self.nodes.3.amplitude * t.powi(3);

        return y
    }
}