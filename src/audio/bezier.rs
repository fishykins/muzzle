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
    pub fn new(x: Duration, amplitude: f64) -> Self {
        Self {
            x,
            amplitude,
        }
    }
}

impl CubicBezier {
    pub fn new(start: Duration, end: Duration, a: Node, b: Node, c: Node, d: Node) -> Self {
        Self {
            start,
            end,
            nodes: (a, b, c, d),
        }
    }

    pub fn amplitude(&self, time: Duration) -> f64 {

        // let t: f64;

        // if time.as_secs_f64() <= self.start.as_secs_f64() {
        //     t = 0.0;
        //     return 0.0;
        // } else if time.as_secs_f64() >= self.end.as_secs_f64() {
        //     t = 1.0;
        //     return 0.0;
        // } else {
        //     t = time.as_secs_f64() / (self.end.as_secs_f64() - self.start.as_secs_f64());
        // }

        let t = time.as_secs_f64() / (self.end.as_secs_f64() - self.start.as_secs_f64());

        let y = 
            self.nodes.0.amplitude * (1.0 - t).powi(3) + 
            self.nodes.1.amplitude * 3.0 * t * (1.0 - t).powi(2) +
            self.nodes.2.amplitude * 3.0 * t.powi(2) * (1.0 - t) +
            self.nodes.3.amplitude * t.powi(3);

        return y
    }
}