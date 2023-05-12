mod brewable;
mod coffee;
mod coffee_machine;
mod filter;
mod liquid_tank;
mod power_button;
mod tea_leaves;

use coffee::Coffee;
use coffee_machine::CoffeeMachine;
use tea_leaves::TeaLeaves;

use crate::{filter::Filter, liquid_tank::LiquidTank};

fn main() {
    let mut coffee_machine = CoffeeMachine::new(Filter::new(Some(Coffee)));
    let mut pot = LiquidTank::new(1400);

    coffee_machine.add_water(1600).unwrap();
    println!("Added 1600 mL water to the coffee machine.");

    coffee_machine.power.flip_switch();

    coffee_machine.brew(&mut pot).unwrap();
    println!("Coffee brewed and poured into the pot.");

    println!(
        "The pot now contains {} mL of {}.",
        pot.current_volume(),
        pot.current_liquid_type().unwrap_or(&"nothing".to_string())
    );

    // part 2 (tea)
    coffee_machine.filter.add(TeaLeaves).unwrap();
    println!("Put tea leaves into the filter.");

    coffee_machine.add_water(600).unwrap();
    println!("Added 600 mL water to the coffee machine.");

    let mut teapot = LiquidTank::new(1000);

    coffee_machine.brew(&mut teapot).unwrap();
    println!("Tea brewed and poured into the pot.");

    println!(
        "The teapot now contains {} mL of {}.",
        teapot.current_volume(),
        teapot
            .current_liquid_type()
            .unwrap_or(&"nothing".to_string())
    );
}
