use crate::shape::Shape;

pub struct Trapezoid {
    base_top: f64,
    base_bottom: f64,
    side_a: f64,
    side_b: f64,
}

impl Trapezoid {
    pub fn new(base_top: f64, base_bottom: f64, side_a: f64, side_b: f64) -> Self {
        Self {
            base_top,
            base_bottom,
            side_a,
            side_b,
        }
    }
}

impl Shape for Trapezoid {
    fn name(&self) -> &'static str {
        "trapezoid"
    }

    fn perimeter(&self) -> f64 {
        self.base_top + self.base_bottom + self.side_a + self.side_b
    }

    fn area(&self) -> f64 {
        let s = (self.side_a + self.side_b - self.base_top + self.base_bottom) / 2.;

        let height = 2. / (self.base_bottom - self.base_top)
            * (s * (s - self.side_a + self.base_bottom) * (s - self.side_b) * (s - self.base_top))
                .sqrt();
        ((self.base_top + self.base_bottom) * height) / 2.
    }
}
