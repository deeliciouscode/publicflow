use crate::control::action::Actions;
use crate::control::parsers::block::{parse_block, parse_unblock};
use crate::control::parsers::get::parse_get;
use crate::control::parsers::make::parse_make;
use crate::control::parsers::route::parse_route;
use crate::control::parsers::viusualize::{parse_hide, parse_visualize};
use crate::helper::helper::read_lines;
use std::io::Write;
use std::sync::mpsc;

// TODO: implement all aktions for all entities if possible to match text in Thesis

fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");

    return line.trim().to_string();
}

pub fn run_cli(tx: mpsc::Sender<Actions>) {
    loop {
        let input = prompt("> ");

        if input == "exit" {
            break;
        };

        let input_list: Vec<&str> = input.split(" ").collect();

        let actions = parse_input(&input_list);

        let _res = tx.send(actions);
    }
}

fn parse_input(input_list: &Vec<&str>) -> Actions {
    let mut actions = Actions::new();
    match input_list[0] {
        "get" | "g" => {
            actions.get_actions = parse_get(&input_list);
        }
        "block" | "b" => {
            actions.set_actions = parse_block(&input_list);
        }
        "unblock" | "ub" | "u" => {
            actions.set_actions = parse_unblock(&input_list);
        }
        "show" | "visualize" | "draw" | "v" | "s" => {
            actions.set_actions = parse_visualize(&input_list);
        }
        "hide" | "h" => {
            actions.set_actions = parse_hide(&input_list);
        }
        "route" | "r" => {
            actions.set_actions = parse_route(&input_list);
        }
        "make" | "m" => {
            actions.set_actions = parse_make(&input_list);
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
                    actions.get_actions.extend(command_actions.get_actions);
                    actions.set_actions.extend(command_actions.set_actions);
                }
            }
        }
        Err(e) => {
            println!("Err: {:?}", e)
        }
    }
    actions
}

pub fn recv_queries(rx: &mpsc::Receiver<Actions>) -> Actions {
    let maybe_received = rx.try_recv();
    match maybe_received {
        Ok(received) => {
            return received;
        }
        Err(_) => {}
    }
    return Actions::new();
}
