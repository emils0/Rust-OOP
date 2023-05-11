use crate::brewable::Brewable;

pub struct CrushedCoffee;

impl Brewable for CrushedCoffee {
    fn liquid_type(&self) -> &str {
        "coffee"
    }
}
