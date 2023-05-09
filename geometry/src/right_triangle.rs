use crate::shape::Shape;

pub struct RightTriangle {
    leg_a: f64,
    leg_b: f64,
    hypotenuse: f64,
}

impl RightTriangle {
    pub fn new(leg_a: f64, leg_b: f64) -> Self {
        let hypotenuse = (leg_a.powi(2) + leg_b.powi(2)).sqrt();
        Self {
            leg_a,
            leg_b,
            hypotenuse,
        }
    }
}

impl Shape for RightTriangle {
    fn name(&self) -> &'static str {
        "right_triangle"
    }

    fn perimeter(&self) -> f64 {
        self.leg_a + self.leg_b + self.hypotenuse
    }

    fn area(&self) -> f64 {
        (1. / 2.) * self.leg_a * self.leg_b
    }
}
