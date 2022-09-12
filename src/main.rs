mod config;
mod connection;
mod control;
mod helper;
mod line;
mod network;
mod pathstate;
mod person;
mod pod;
mod state;
mod station;

use crate::config::constants::{CONFIG_NAME, CONFIG_ROOT};
use crate::config::processing::{load_yaml, parse_config};
use crate::control::cli::run_cli;
use crate::control::proxy::run_emmiter;
use crate::state::State;
use ggez::event::{self};
use ggez::ContextBuilder;
use std::sync::mpsc;
use std::thread;

fn main() {
    println!("start simulation...");
    let config_yaml = load_yaml(CONFIG_ROOT, CONFIG_NAME);
    let config = parse_config(&config_yaml);

    let (proxy_tx, proxy_rx) = mpsc::channel();
    let (tx, rx) = mpsc::channel();

    let command_on_start = config.logic.command_on_start.clone();
    thread::spawn(|| {
        let _res = run_cli(proxy_tx, command_on_start);
    });

    thread::spawn(|| {
        run_emmiter(proxy_rx, tx);
    });

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

    // let state = State::new(config, rx).add_pods().add_people();
    let state = State::new(config, rx).add_people();

    event::run(ctx, event_loop, state);
}
