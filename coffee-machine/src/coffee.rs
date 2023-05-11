use crate::brewable::Brewable;

pub struct Coffee;

impl Brewable for Coffee {
    fn liquid_type(&self) -> &str {
        "coffee"
    }

    fn liquid_amount(&self) -> u32 {
        1200
    }
}
