use crate::filter::Filter;
use crate::liquid_tank::LiquidTank;
use crate::power_button::PowerSwitch;

pub struct CoffeeMachine {
    pub power: PowerSwitch,
    pub filter: Filter,
    water_tank: LiquidTank,
}

impl CoffeeMachine {
    pub fn new(filter: Filter) -> CoffeeMachine {
        CoffeeMachine {
            power: PowerSwitch::new(),
            water_tank: LiquidTank::new(1800),
            filter,
        }
    }

    pub fn add_water(&mut self, volume: u32) -> Result<(), &'static str> {
        self.water_tank.fill(volume, "water".to_string())
    }

    pub fn brew(&mut self, receiving_tank: &mut LiquidTank) -> Result<(), &'static str> {
        if !self.power.is_on() {
            return Err("The machine is turned off.");
        }

        let (brewable, brew_volume) = self.filter.consume_brewable();
        if self.water_tank.current_volume() < brew_volume {
            return Err("Not enough water in the tank.");
        }

        self.water_tank.empty(brew_volume)?;
        receiving_tank.fill(brew_volume, brewable)?;

        Ok(())
    }
}
