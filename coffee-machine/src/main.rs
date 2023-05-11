mod appliance;
mod brewable;
mod coffee_beans;
mod coffee_machine;
mod crushable;
mod crushed_coffee;
mod filter;
mod liquid_tank;
mod power_button;

use coffee_machine::CoffeeMachine;
use crushed_coffee::CrushedCoffee

use crate::filter::Filter;

fn main() {
    let coffee_machine = CoffeeMachine::new(10, Filter::new(Some(Box::new(CrushedCoffee))));

    println!("stuff.");
}
