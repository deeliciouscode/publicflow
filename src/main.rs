mod config;
mod connection;
mod line;
mod network;
mod person;
mod pod;
mod state;
mod station;

use crate::config::{load_file, parse_yaml, SIMULATION_DURATION, SPEED_FACTOR};
use crate::state::gen_state;
// use crate::state::{get_state, State};
use std::{thread, time};
// TODO: first implement something where people are just moving mindlessly, without destination

fn main() {
    // let yaml = load_file("./config/network.yaml");
    let yaml = load_file("./config/network_simple.yaml");
    // println!("{:?}\n", yaml);

    let config = parse_yaml(&yaml);
    println!("config: {:?}\n", config);

    // let mut state = get_state();
    let mut state = gen_state(&config);
    // println!("initial state: {:?}\n", state);
    state.print_state();

    println!("start simulation...");
    let mut seconds = 0;
    loop {
        if seconds >= SIMULATION_DURATION {
            break;
        }
        thread::sleep(time::Duration::from_millis(1000 / SPEED_FACTOR));
        seconds += 1;
        state.update();

        if seconds % 25 == 0 {
            println!("-------------------------------");
            println!("time passed: {}", seconds);
            state.print_state();
        }
    }

    // state.print_state();
}
