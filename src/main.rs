mod action;
mod cli;
mod config;
mod connection;
mod helper;
mod line;
mod network;
mod pathstate;
mod person;
mod pod;
mod state;
mod station;

use crate::cli::run_cli;
use crate::config::{
    load_yaml, parse_raw_config, ALL_LINES_PATH, CONFIG_PATH, SCREEN_SIZE, STATIONS_PATH,
    SUBWAY_LINES_PATH, TRAM_LINES_PATH, VSYNC,
};
use crate::state::State;
use ggez::event::{self};
use ggez::ContextBuilder;
use std::sync::mpsc;
use std::thread;

fn main() {
    println!("start simulation...");
    let (tx, rx) = mpsc::channel();

    thread::spawn(|| {
        run_cli(tx);
    });
    // let yaml = load_file("./config/network.yaml");
    let raw_config = load_yaml(CONFIG_PATH);
    // println!("{:?}\n", yaml);
    let raw_stations = load_yaml(STATIONS_PATH);
    // let raw_subway_lines = load_yaml(SUBWAY_LINES_PATH);
    // let raw_tram_lines = load_yaml(TRAM_LINES_PATH);
    let raw_lines = load_yaml(ALL_LINES_PATH);

    let config = parse_raw_config(&raw_config, &raw_stations, &raw_lines);
    // println!("config: {:?}\n", config);

    // Make a Context.
    let (ctx, event_loop) = ContextBuilder::new("PublicFlow", "David Schmider")
        .window_setup(ggez::conf::WindowSetup::default().title("PublicFlow Simulation"))
        .window_setup(ggez::conf::WindowSetup::default().vsync(VSYNC)) // sync fps to screen refresh rate
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()
        .expect("aieee, could not create ggez context!");

    let mut state = State::new(&config, rx).add_pods().add_people();

    event::run(ctx, event_loop, state);
}
