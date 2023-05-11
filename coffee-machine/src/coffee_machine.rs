use crate::brewable::Brewable;
use crate::filter::Filter;
use crate::liquid_tank::LiquidTank;
use crate::power_button::PowerButton;

pub struct CoffeeMachine {
    pub power: PowerButton,
    water_tank: LiquidTank,
    filter: Filter,
}

impl CoffeeMachine {
    pub fn new(water_capacity: u32, filter: Filter) -> CoffeeMachine {
        CoffeeMachine {
            power: PowerButton::new(),
            water_tank: LiquidTank::new("water", water_capacity),
            filter,
        }
    }

    pub fn add_water(&mut self, volume: u32) -> Result<(), &'static str> {
        self.water_tank.fill(volume)
    }

    pub fn add(&mut self, item: Box<dyn Brewable>) {
        self.filter.add(item);
    }

    pub fn brew(&mut self) -> Result<(), &'static str> {
        if !self.power.check_power() {
            return Err("The machine is turned off.");
        }

        if self.water_tank.current_volume() == 0 {
            return Err("The water tank is empty.");
        }

        self.water_tank.empty();

        self.filter.add(-1);

        Ok(())
    }
}
