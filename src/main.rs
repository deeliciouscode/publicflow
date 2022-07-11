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
use crate::config::{load_yaml, parse_config, CONFIG_PATH};
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

    let config_yaml = load_yaml(CONFIG_PATH);
    let config = parse_config(&config_yaml);

    // Make a Context.
    let (ctx, event_loop) = ContextBuilder::new("PublicFlow", "David Schmider")
        .window_setup(ggez::conf::WindowSetup::default().title("PublicFlow Simulation"))
        .window_setup(ggez::conf::WindowSetup::default().vsync(config.visual.vsync)) // sync fps to screen refresh rate
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(config.visual.screen_size.0, config.visual.screen_size.1),
        )
        .build()
        .expect("aieee, could not create ggez context!");

    let mut state = State::new(config, rx).add_pods().add_people();

    event::run(ctx, event_loop, state);
}
