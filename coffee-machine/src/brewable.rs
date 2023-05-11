pub trait Brewable {
    fn liquid_type(&self) -> &str;
    fn liquid_amount(&self) -> u32;
}
