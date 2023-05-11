pub struct LiquidTank {
    liquid_type: &'static str,
    capacity: u32,
    current_volume: u32,
}

impl LiquidTank {
    pub fn new(liquid_type: &str, capacity: u32) -> LiquidTank {
        LiquidTank {
            liquid_type,
            capacity,
            current_volume: 0,
        }
    }

    pub fn fill(&mut self, volume: u32) -> Result<(), &'static str> {
        match self.current_volume + volume {
            n if n > self.capacity => Err("The tank can't hold that much liquid."),
            n => {
                self.current_volume = n;
                Ok(())
            }
        }
    }

    pub fn set_liquid_type(&mut self) -> Result<(), &'static str> {
        match self.current_volume {
            0 => Ok(()),
            _ => Err("You need to empty the container before changing its liquid type."),
        }
    }

    pub fn empty(&mut self) -> u32 {
        let output = self.current_volume;
        self.current_volume = 0;
        output
    }

    pub fn current_volume(&self) -> u32 {
        self.current_volume
    }
}
