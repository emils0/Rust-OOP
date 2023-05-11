pub struct LiquidTank {
    liquid_type: Option<String>,
    capacity: u32,       // in mL
    current_volume: u32, // in mL
}

impl LiquidTank {
    pub fn new(capacity: u32) -> LiquidTank {
        LiquidTank {
            liquid_type: None,
            capacity,
            current_volume: 0,
        }
    }

    pub fn fill(&mut self, volume: u32, liquid_type: String) -> Result<(), &'static str> {
        match self.current_volume + volume {
            n if n > self.capacity => Err("The tank can't hold that much liquid."),
            n => {
                self.current_volume = n;
                self.liquid_type = Some(liquid_type);
                Ok(())
            }
        }
    }

    pub fn empty(&mut self, volume: u32) -> Result<(), &'static str> {
        if volume > self.current_volume {
            Err("The tank does not have that much liquid.")
        } else {
            self.current_volume -= volume;
            if self.current_volume == 0 {
                self.liquid_type = None;
            }
            Ok(())
        }
    }

    pub fn current_volume(&self) -> u32 {
        self.current_volume
    }

    pub fn current_liquid_type(&self) -> Option<&String> {
        self.liquid_type.as_ref()
    }
}
