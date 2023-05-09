use parallelogram::Parallelogram;
use rectangle::Rectangle;
use right_triangle::RightTriangle;
use shape::Shape;
use square::Square;
use trapezoid::Trapezoid;

mod parallelogram;
mod rectangle;
mod right_triangle;
mod shape;
mod square;
mod trapezoid;

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Rectangle::new(4., 2., 4., 2.)),
        Box::new(Square::new(3.)),
        Box::new(Parallelogram::new(5., 3., 3.)),
        Box::new(Trapezoid::new(8., 10., 9., 9.)),
        Box::new(RightTriangle::new(3., 4.)),
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
