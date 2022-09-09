use crate::control::action::SetAction;
use crate::helper::helper::parse_make_arg_to_line_and_direction;
use std::str::FromStr;

pub fn parse_make(input_list: &Vec<&str>) -> Vec<SetAction> {
    let mut set_actions: Vec<SetAction> = vec![];
    if input_list.len() < 2 {
        println!("Make what??");
        return set_actions;
    }

    // make platform op 0 u1 -> make platform op 0 u1+-
    match input_list[1] {
        "platform" | "pl" => {
            if input_list.len() < 5 {
                println!("Too few arguments, make what with platform??");
                return set_actions;
            }
            let station_id: i32 = FromStr::from_str(input_list[3]).unwrap();
            for arg in &input_list[4..] {
                match input_list[2] {
                    "operational" | "op" => {
                        let (line, directions) =
                            parse_make_arg_to_line_and_direction(&arg.to_string());
                        for direction in directions {
                            set_actions.push(SetAction::MakePlatformOperational {
                                station_id: station_id,
                                line: line.clone(),
                                direction: direction,
                            })
                        }
                    }
                    "passable" | "pass" => {
                        let (line, directions) =
                            parse_make_arg_to_line_and_direction(&arg.to_string());
                        for direction in directions {
                            set_actions.push(SetAction::MakePlatformPassable {
                                station_id: station_id,
                                line: line.clone(),
                                direction: direction,
                            })
                        }
                    }
                    "queuable" | "qu" => {
                        let (line, directions) =
                            parse_make_arg_to_line_and_direction(&arg.to_string());
                        for direction in directions {
                            set_actions.push(SetAction::MakePlatformQueuable {
                                station_id: station_id,
                                line: line.clone(),
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

    return set_actions;
}
