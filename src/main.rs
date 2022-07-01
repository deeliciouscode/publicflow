mod config;
mod connection;
mod line;
mod network;
mod pathstate;
mod person;
mod pod;
mod state;
mod station;

use crate::config::{load_yaml, parse_raw_config, CONFIG_PATH, SCREEN_SIZE, STATIONS_PATH, VSYNC};
use crate::state::State;
// use crate::state::{gen_state, State};
// use crate::state::{get_state, State};
use ggez::event::{self};
use ggez::ContextBuilder;
// TODO: first implement something where people are just moving mindlessly, without destination

fn main() {
    // let yaml = load_file("./config/network.yaml");
    let raw_config = load_yaml(CONFIG_PATH);
    // println!("{:?}\n", yaml);
    let raw_stations = load_yaml(STATIONS_PATH);

    let config = parse_raw_config(&raw_config, &raw_stations);
    println!("config: {:?}\n", config);

    // Make a Context.
    let (ctx, event_loop) = ContextBuilder::new("PublicFlow", "David Schmider")
        .window_setup(ggez::conf::WindowSetup::default().title("PublicFlow Simulation"))
        .window_setup(ggez::conf::WindowSetup::default().vsync(VSYNC)) // sync fps to screen refresh rate
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()
        .expect("aieee, could not create ggez context!");

    let state = State::new(&config);
    println!("start simulation...");
    event::run(ctx, event_loop, state);
}
