mod brewable;
mod coffee;
mod coffee_machine;
mod filter;
mod liquid_tank;
mod power_button;

use coffee::Coffee;
use coffee_machine::CoffeeMachine;

use crate::{filter::Filter, liquid_tank::LiquidTank};

fn main() {
    let mut coffee_machine = CoffeeMachine::new(Filter::new(Some(Coffee)));
    let mut pot = LiquidTank::new(1400);

    match coffee_machine.add_water(1600) {
        Ok(_) => println!("Water added to the coffee machine."),
        Err(e) => println!("{}", e),
    }

    coffee_machine.power.flip_switch();

    match coffee_machine.brew(&mut pot) {
        Ok(_) => println!("Coffee brewed and poured into the pot."),
        Err(e) => println!("{}", e),
    }

    println!(
        "The pot now contains {} mL of {}.",
        pot.current_volume(),
        pot.current_liquid_type().unwrap_or(&"nothing".to_string())
    );
}
