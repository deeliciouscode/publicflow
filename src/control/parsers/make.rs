use crate::control::action::Action;
use crate::helper::functions::parse_str_to_line_and_directions;
use std::str::FromStr;

pub fn parse_make(input_list: &Vec<&str>) -> Vec<Action> {
    let mut actions: Vec<Action> = vec![];
    if input_list.len() < 2 {
        println!("Make what??");
        return actions;
    }

    // make platform op 0 u1 -> make platform op 0 u1+-
    match input_list[1] {
        "platform" | "pl" => {
            if input_list.len() < 5 {
                println!("Too few arguments, make what with platform??");
                return actions;
            }
            let station_id: i32 = FromStr::from_str(input_list[3]).unwrap();
            for arg in &input_list[4..] {
                match input_list[2] {
                    "operational" | "op" => {
                        let (line, directions) = parse_str_to_line_and_directions(&arg.to_string());
                        for direction in directions {
                            actions.push(Action::MakePlatformOperational {
                                station_id: station_id,
                                line_name: line.clone(),
                                direction: direction,
                            })
                        }
                    }
                    "passable" | "pass" => {
                        let (line, directions) = parse_str_to_line_and_directions(&arg.to_string());
                        for direction in directions {
                            actions.push(Action::MakePlatformPassable {
                                station_id: station_id,
                                line_name: line.clone(),
                                direction: direction,
                            })
                        }
                    }
                    "queuable" | "qu" => {
                        let (line, directions) = parse_str_to_line_and_directions(&arg.to_string());
                        for direction in directions {
                            actions.push(Action::MakePlatformQueuable {
                                station_id: station_id,
                                line_name: line.clone(),
                                direction: direction,
                            })
                        }
                    }
                    _ => {
                        println!("Make action: {} not implemented.", input_list[2])
                    }
                }
            }
        }
        _ => {
            println!(
                "Can't make anything with: {}, not implemented.",
                input_list[1]
            )
        }
    }

    return actions;
}
