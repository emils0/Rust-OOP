pub struct PowerButton(bool);

impl PowerButton {
    pub fn new() -> Self {
        Self { 0: false }
    }

    pub fn switch_button(&mut self) {
        self.0 = !self.0
    }

    pub fn check_power(&self) -> bool {
        self.0
    }
}
