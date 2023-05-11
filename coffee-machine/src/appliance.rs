pub trait Appliance {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn is_on(&mut self) -> bool;
}
