use crate::brewable::Brewable;
use crate::crushable::Crushable;
use crate::crushed_coffee::CrushedCoffee;

pub struct CoffeeBeans;

impl Crushable for CoffeeBeans {
    fn crushing_output() -> Box<dyn Brewable> {
        Box::new(CrushedCoffee)
    }
}
