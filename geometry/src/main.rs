use parallelogram::Parallelogram;
use rectangle::Rectangle;
use right_triangle::RightTriangle;
use shape::Shape;
use square::Square;
use std::f64::consts::PI;
use trapezoid::Trapezoid;

mod parallelogram;
mod rectangle;
mod right_triangle;
mod shape;
mod square;
mod trapezoid;

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Rectangle::new(4.0, 2.0, 4.0, 2.0)),
        Box::new(Square::new(3.0)),
        Box::new(Parallelogram::new(5.0, 3.0, PI / 3.0)),
        Box::new(Trapezoid::new(3.0, 5.0, 2.0, 4.0)),
        Box::new(RightTriangle::new(3.0, 4.0)),
    ];

    for shape in &shapes {
        println!(
            "Shape: {}\nPerimeter: {}\nArea: {}\n",
            shape.name(),
            shape.perimeter(),
            shape.area()
        );
    }
}
