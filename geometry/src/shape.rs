pub trait Shape {
    fn name(&self) -> &'static str;

    fn perimeter(&self) -> f64;

    fn area(&self) -> f64;
}
