use crate::shape::Shape;

pub struct Square {
    side_length: f64,
}

impl Square {
    pub fn new(side_length: f64) -> Self {
        Self { side_length }
    }
}

impl Shape for Square {
    fn name() -> &'static str {
        "square"
    }

    fn perimeter(&self) -> f64 {
        self.side_length * 4.
    }

    fn area(&self) -> f64 {
        self.side_length.powi(2)
    }
}
