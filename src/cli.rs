use crate::action::{GetAction, SetAction};
use crate::state::State;
use std::io::Write;
use std::str::FromStr;
use std::sync::mpsc;
use std::time::{Duration, SystemTime};

fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");

    return line.trim().to_string();
}

pub fn run_cli(tx: mpsc::Sender<String>) {
    loop {
        let input = prompt("> ");

        if input == "exit" {
            break;
        };

        let _res = tx.send(input);
    }
}

pub fn handle_queries(
    state: &State,
    rx: &mpsc::Receiver<String>,
) -> (Vec<GetAction>, Vec<SetAction>) {
    let maybe_received = rx.try_recv();
    let mut get_actions = vec![];
    let mut set_actions = vec![];

    match maybe_received {
        Ok(received) => {
            let input_list: Vec<&str> = received.split(" ").collect();

            if input_list[0] == "get" {
                if input_list[1] == "station" {
                    println!("get station: {}", input_list[2]);
                    get_actions.push(GetAction::GetStation {
                        station_id: FromStr::from_str(input_list[2]).unwrap(),
                    })
                }
            }

            if input_list[0] == "set" {
                println!("set me if ya can")
            }
        }
        Err(_) => {}
    }
    return (get_actions, set_actions);
}
