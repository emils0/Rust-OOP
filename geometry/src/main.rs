use rectangle::Rectangle;
use square::Square;

use crate::shape::Shape;

mod rectangle;
mod shape;
mod square;

fn main() {
    let vec![Square::new(7), Rectangle::new()];

    let square = Square::new(7.);

    println!("{}", square.area())
}
