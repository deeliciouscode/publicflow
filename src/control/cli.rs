use crate::control::action::{Action, Actions};
use crate::control::parsers::block::{parse_block, parse_unblock};
use crate::control::parsers::get::parse_get;
use crate::control::parsers::make::parse_make;
use crate::control::parsers::route::parse_route;
use crate::control::parsers::sleep::parse_sleep;
use crate::control::parsers::spawn::parse_spawn;
use crate::control::parsers::visualize::{parse_hide, parse_visualize};
use crate::helper::functions::read_lines;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};
use std::sync::mpsc;

// TODO: implement all actions for all entities if possible to match text in Thesis

pub fn run_cli(tx: mpsc::Sender<Actions>, command_on_start: String) -> Result<()> {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new()?;
    if rl.load_history(".meta/history.txt").is_err() {
        println!("No previous history.");
    }
    let input_list = command_on_start.split(" ").collect();
    let actions = parse_input(&input_list);
    let _res = tx.send(actions);
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let input_list: Vec<&str> = line.split(" ").collect();
                let actions = parse_input(&input_list);
                let _res = tx.send(actions);
            }
            Err(ReadlineError::Interrupted) => {
                let mut actions = Actions::new();
                actions.actions.push(Action::KillSimulation { code: 0 });
                // .push(EffectAction::KillSimulation { code: 0 });

                let _res = tx.send(actions);
                break;
            }
            Err(ReadlineError::Eof) => {
                let mut actions = Actions::new();
                actions.actions.push(Action::KillSimulation { code: 0 });

                let _res = tx.send(actions);
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(".meta/history.txt")
}

fn parse_input(input_list: &Vec<&str>) -> Actions {
    let mut actions = Actions::new();
    match input_list[0] {
        "get" | "g" => {
            actions.actions = parse_get(&input_list);
        }
        "block" | "b" => {
            actions.actions = parse_block(&input_list);
        }
        "unblock" | "ub" | "u" => {
            actions.actions = parse_unblock(&input_list);
        }
        "show" | "visualize" | "draw" | "v" | "s" => {
            actions.actions = parse_visualize(&input_list);
        }
        "hide" | "h" => {
            actions.actions = parse_hide(&input_list);
        }
        "route" | "r" => {
            actions.actions = parse_route(&input_list);
        }
        "make" | "m" => {
            actions.actions = parse_make(&input_list); // make platform op 0 u1 -> make platform 0 u1 - op
        }
        "spawn" | "sp" => {
            actions.actions = parse_spawn(&input_list); // make platform op 0 u1 -> make platform 0 u1 - op
        }
        "sleep" | "sl" => {
            actions.actions = parse_sleep(&input_list);
        }
        "run" => actions = run_script(&input_list),
        _ => {}
    }
    actions
}

fn run_script(input_list: &Vec<&str>) -> Actions {
    let mut actions = Actions::new();
    let lines_res = read_lines(input_list[1]);
    match lines_res {
        Ok(lines) => {
            for line in lines {
                if let Ok(command) = line {
                    let input_list = command.split(" ").collect();
                    let command_actions = parse_input(&input_list);
                    actions.actions.extend(command_actions.actions);
                }
            }
        }
        Err(e) => {
            println!("Err: {:?}", e)
        }
    }
    actions
}
