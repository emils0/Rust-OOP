pub trait Shape {
    fn name() -> &'static str;

    fn perimeter(&self) -> f64;

    fn area(&self) -> f64;
}
