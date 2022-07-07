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

pub fn run_cli(tx: mpsc::Sender<(Vec<GetAction>, Vec<SetAction>)>) {
    loop {
        let input = prompt("> ");

        if input == "exit" {
            break;
        };

        let input_list: Vec<&str> = input.split(" ").collect();

        let mut get_actions: Vec<GetAction> = vec![];
        let mut set_actions: Vec<SetAction> = vec![];

        if input_list[0] == "get" {
            if input_list.len() < 2 {
                println!("Get what??");
                continue;
            }

            match input_list[1] {
                "station" => {
                    if input_list.len() < 3 {
                        println!("Get which stations??");
                        continue;
                    }
                    for id in &input_list[2..] {
                        let maybe_station_id = FromStr::from_str(id);
                        match maybe_station_id {
                            Ok(station_id) => get_actions.push(GetAction::GetStation {
                                station_id: station_id,
                            }),
                            Err(_) => println!("Couldn't parse \'{}\' into a station_id", id),
                        }
                    }
                }
                _ => {
                    println!("Can't get: {}, not implemented.", input_list[1])
                }
            }
        }

        if input_list[0] == "set" {
            println!("set me if ya can")
        }

        let _res = tx.send((get_actions, set_actions));
    }
}

pub fn recv_queries(
    state: &State,
    rx: &mpsc::Receiver<(Vec<GetAction>, Vec<SetAction>)>,
) -> (Vec<GetAction>, Vec<SetAction>) {
    let mut actions = (vec![], vec![]);

    let maybe_received = rx.try_recv();

    match maybe_received {
        Ok(received) => {
            return received;
        }
        Err(_) => {}
    }
    return actions;
}
