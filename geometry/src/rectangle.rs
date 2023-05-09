use crate::shape::Shape;

pub struct Rectangle {
    side_a: f64,
    side_b: f64,
    side_c: f64,
    side_d: f64,
}

impl Rectangle {
    pub fn new(side_a: f64, side_b: f64, side_c: f64, side_d: f64) -> Self {
        Self {
            side_a,
            side_b,
            side_c,
            side_d,
        }
    }
}

impl Shape for Rectangle {
    fn name(&self) -> &'static str {
        "rectangle"
    }

    fn perimeter(&self) -> f64 {
        self.side_a + self.side_b + self.side_c + self.side_d
    }

    fn area(&self) -> f64 {
        self.side_a * self.side_b
    }
}
