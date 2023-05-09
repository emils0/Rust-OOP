use crate::shape::Shape;
use std::f64::consts::PI;

pub struct Parallelogram {
    side_a: f64,
    side_b: f64,
    angle: f64, // In radians
}

impl Parallelogram {
    pub fn new(side_a: f64, side_b: f64, angle: f64) -> Self {
        let angle = (angle * (PI / 180.)).sin();
        Self {
            side_a,
            side_b,
            angle,
        }
    }
}

impl Shape for Parallelogram {
    fn name(&self) -> &'static str {
        "parallelogram"
    }

    fn perimeter(&self) -> f64 {
        2. * (self.side_a + self.side_b)
    }

    fn area(&self) -> f64 {
        self.side_a * self.side_b * self.angle.sin()
    }
}
