use crate::brewable::Brewable;

pub struct EspressoPowder;

impl Brewable for EspressoPowder {
    fn liquid_type(&self) -> &str {
        "espresso coffee"
    }

    fn liquid_amount(&self) -> u32 {
        250
    }
}
