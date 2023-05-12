use crate::brewable::Brewable;

pub struct TeaLeaves;

impl Brewable for TeaLeaves {
    fn liquid_type(&self) -> &str {
        "green tea"
    }

    fn liquid_amount(&self) -> u32 {
        1000
    }
}
