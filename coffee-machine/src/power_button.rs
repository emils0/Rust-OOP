pub struct PowerSwitch(bool);

impl PowerSwitch {
    pub fn new() -> Self {
        Self { 0: false }
    }

    pub fn flip_switch(&mut self) {
        self.0 = !self.0
    }

    pub fn is_on(&self) -> bool {
        self.0
    }
}
