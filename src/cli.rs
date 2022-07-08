use crate::action::{Actions, GetAction, SetAction};
use crate::state::State;
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;
use std::sync::mpsc;

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

        let mut actions = Actions::new();

        match input_list[0] {
            "get" => {
                actions.get_actions = parse_get(&input_list);
            }
            "block" => {
                actions.set_actions = parse_block(&input_list);
            }
            _ => {}
        }

        if input_list[0] == "set" {
            println!("set me if ya can")
        }

        let _res = tx.send(actions);
    }
}

fn parse_get(input_list: &Vec<&str>) -> Vec<GetAction> {
    let mut get_actions: Vec<GetAction> = vec![];
    if input_list.len() < 2 {
        println!("Get what??");
        return get_actions;
    }

    match input_list[1] {
        "station" => {
            if input_list.len() < 3 {
                println!("Get which stations??");
                return get_actions;
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

    return get_actions;
}

fn parse_block(input_list: &Vec<&str>) -> Vec<SetAction> {
    let mut set_actions: Vec<SetAction> = vec![];
    if input_list.len() < 2 {
        println!("Block what??");
        return set_actions;
    }

    match input_list[1] {
        "connection" => {
            if input_list.len() < 3 {
                println!("Block which connections??");
                return set_actions;
            }
            for connection in &input_list[2..] {
                let station_ids_str: Vec<&str> = connection.split("-").collect();

                if station_ids_str.len() < 2 {
                    println!("Thats not a connection: {}", connection);
                    return set_actions;
                }

                fn try_make_i32(s: &&str) -> i32 {
                    let maybe_id = FromStr::from_str(s);
                    match maybe_id {
                        Ok(id) => id,
                        Err(_) => {
                            println!("Couldn't parse \'{}\' into i32", s);
                            return -1;
                        }
                    }
                }

                let station_ids: Vec<i32> = station_ids_str.iter().map(try_make_i32).collect();

                if station_ids.contains(&-1) {
                    return set_actions;
                }

                for i in 0..(station_ids.len() - 1) {
                    set_actions.push(SetAction::BlockConnection {
                        ids: HashSet::from([station_ids[i], station_ids[i + 1]]),
                    })
                }
            }
        }
        _ => {
            println!("Can't block: {}, not implemented.", input_list[1])
        }
    }

    return set_actions;
}

pub fn recv_queries(state: &State, rx: &mpsc::Receiver<Actions>) -> Actions {
    let mut actions = Actions::new();

    let maybe_received = rx.try_recv();

    match maybe_received {
        Ok(received) => {
            return received;
        }
        Err(_) => {}
    }
    return actions;
}
