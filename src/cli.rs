use crate::action::{Actions, GetAction, SetAction};
use crate::helper::read_lines;
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

fn parse_route(input_list: &Vec<&str>) -> Vec<SetAction> {
    let mut set_actions: Vec<SetAction> = vec![];
    if input_list.len() < 2 {
        println!("Route what??");
        return set_actions;
    }

    match input_list[1] {
        "person" | "people" | "p" => {
            if input_list[2..].len() < 2 {
                println!("Route which person/people to where??");
                println!("Syntax: route person [--random | -r] [id id | lower..upper | id lower..upper] to_station");
                println!("If --random is used, no to_station is needed.");
                return set_actions;
            }

            let mut random_station = false;
            let mut first_id_index = 2;
            let mut last_id_index = input_list.len() - 1;
            if let "--random" | "-r" = input_list[2] {
                random_station = true;
                first_id_index = 3;
                last_id_index = input_list.len();
            }

            let station_id;
            if !random_station {
                station_id = *parse_id_list_and_ranges(input_list[input_list.len() - 1])
                    .first()
                    .unwrap() as u32;
            } else {
                station_id = u32::MAX;
            }

            for arg in &input_list[first_id_index..last_id_index] {
                let ids = parse_id_list_and_ranges(arg);
                for id in ids {
                    set_actions.push(SetAction::RoutePerson {
                        id: id,
                        station_id: station_id,
                        random_station: random_station,
                    })
                }
            }
        }
        _ => {
            println!("Can't route: {}, not implemented.", input_list[1])
        }
    }

    return set_actions;
}

fn parse_get(input_list: &Vec<&str>) -> Vec<GetAction> {
    let mut get_actions: Vec<GetAction> = vec![];
    if input_list.len() < 2 {
        println!("Get what??");
        return get_actions;
    }

    match input_list[1] {
        "station" | "stations" | "st" => {
            if input_list.len() < 3 {
                println!("Get which stations??");
                return get_actions;
            }
            for arg in &input_list[2..] {
                let ids = parse_id_list_and_ranges(arg);
                for id in ids {
                    get_actions.push(GetAction::GetStation { id: id })
                }
            }
        }
        "person" | "people" | "p" => {
            if input_list.len() < 3 {
                println!("Get which person/people??");
                return get_actions;
            }
            for arg in &input_list[2..] {
                let ids = parse_id_list_and_ranges(arg);
                for id in ids {
                    get_actions.push(GetAction::GetPerson { id: id })
                }
            }
        }
        _ => {
            println!("Can't get: {}, not implemented.", input_list[1])
        }
    }

    return get_actions;
}

fn parse_visualize(input_list: &Vec<&str>) -> Vec<SetAction> {
    let mut set_actions: Vec<SetAction> = vec![];
    if input_list.len() < 2 {
        println!("Visualize what??");
        return set_actions;
    }

    match input_list[1] {
        "person" | "people" | "p" => {
            if input_list.len() < 3 {
                println!("Visualize which person/people??");
                return set_actions;
            }
            let mut follow = false;
            for arg in &input_list[2..] {
                match arg {
                    &"--follow" | &"-f" => {
                        follow = true;
                    }
                    id_or_ids => {
                        let ids = parse_id_list_and_ranges(id_or_ids);
                        for id in ids {
                            set_actions.push(SetAction::ShowPerson {
                                id: id,
                                follow: follow,
                            })
                        }
                    }
                }
            }
        }
        _ => {
            println!("Can't visualize: {}, not implemented.", input_list[1])
        }
    }

    return set_actions;
}

fn parse_hide(input_list: &Vec<&str>) -> Vec<SetAction> {
    let mut set_actions: Vec<SetAction> = vec![];
    if input_list.len() < 2 {
        println!("Hide what??");
        return set_actions;
    }

    match input_list[1] {
        "person" | "people" | "p" => {
            if input_list.len() < 3 {
                println!("Hide which person/people??");
                return set_actions;
            }
            let mut follow = false;
            for arg in &input_list[2..] {
                {
                    let ids = parse_id_list_and_ranges(arg);
                    for id in ids {
                        set_actions.push(SetAction::HidePerson { id: id })
                    }
                }
            }
        }
        _ => {
            println!("Can't hide: {}, not implemented.", input_list[1])
        }
    }

    return set_actions;
}

fn parse_id_list_and_ranges(id_or_ids: &str) -> Vec<i32> {
    let mut ids = vec![];
    if id_or_ids.contains("..") {
        let split: Vec<&str> = id_or_ids.split("..").collect();
        // Will fail if on is not parsable
        let from: i32 = FromStr::from_str(split[0]).unwrap();
        let to: i32 = FromStr::from_str(split[1]).unwrap();
        for id in from..=to {
            ids.push(id)
        }
    } else {
        let maybe_id = FromStr::from_str(id_or_ids);
        match maybe_id {
            Ok(id) => ids.push(id),
            Err(_) => println!(
                "Couldn't parse \'{}\' into an id or id range, use \'x..y\'",
                id_or_ids
            ),
        }
    }
    ids
}

fn parse_block(input_list: &Vec<&str>) -> Vec<SetAction> {
    let mut set_actions: Vec<SetAction> = vec![];
    if input_list.len() < 2 {
        println!("Block what??");
        return set_actions;
    }

    match input_list[1] {
        "connection" | "connections" | "conn" | "c" => {
            if input_list.len() < 3 {
                println!("Block which connections??");
                return set_actions;
            }
            for connection in &input_list[2..] {
                let maybe_station_ids = parse_connection(connection);
                match maybe_station_ids {
                    Some(station_ids) => {
                        for i in 0..(station_ids.len() - 1) {
                            set_actions.push(SetAction::BlockConnection {
                                ids: HashSet::from([station_ids[i], station_ids[i + 1]]),
                            })
                        }
                    }
                    None => return set_actions,
                }
            }
        }
        _ => {
            println!("Can't block: {}, not implemented.", input_list[1])
        }
    }

    return set_actions;
}

fn parse_unblock(input_list: &Vec<&str>) -> Vec<SetAction> {
    let mut set_actions: Vec<SetAction> = vec![];
    if input_list.len() < 2 {
        println!("Block what??");
        return set_actions;
    }

    match input_list[1] {
        "connection" | "connections" | "conn" | "c" => {
            if input_list.len() < 3 {
                println!("Block which connections??");
                return set_actions;
            }
            for connection in &input_list[2..] {
                let maybe_station_ids = parse_connection(connection);
                match maybe_station_ids {
                    Some(station_ids) => {
                        for i in 0..(station_ids.len() - 1) {
                            set_actions.push(SetAction::UnblockConnection {
                                ids: HashSet::from([station_ids[i], station_ids[i + 1]]),
                            })
                        }
                    }
                    None => return set_actions,
                }
            }
        }
        _ => {
            println!("Can't unblock: {}, not implemented.", input_list[1])
        }
    }

    return set_actions;
}

fn parse_connection(connection: &&str) -> Option<Vec<i32>> {
    let station_ids_str: Vec<&str> = connection.split("-").collect();

    if station_ids_str.len() < 2 {
        println!("Thats not a connection: {}", connection);
        return None;
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
        return None;
    }

    return Some(station_ids);
}

pub fn recv_queries(state: &State, rx: &mpsc::Receiver<Actions>) -> Actions {
    let maybe_received = rx.try_recv();
    match maybe_received {
        Ok(received) => {
            return received;
        }
        Err(_) => {}
    }
    return Actions::new();
}
