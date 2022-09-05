use crate::control::action::SetAction;
use crate::helper::helper::transform_line_name_to_enum;
use std::str::FromStr;

pub fn parse_make(input_list: &Vec<&str>) -> Vec<SetAction> {
    let mut set_actions: Vec<SetAction> = vec![];
    if input_list.len() < 2 {
        println!("Make what??");
        return set_actions;
    }

    match input_list[1] {
        "platform" | "pl" => {
            if input_list.len() < 5 {
                println!("Too few arguments, make what with platform??");
                return set_actions;
            }
            let station_id: i32 = FromStr::from_str(input_list[3]).unwrap();
            for arg in &input_list[4..] {
                match input_list[2] {
                    "operational" | "op" => set_actions.push(SetAction::MakePlatformOperational {
                        station_id: station_id,
                        line: transform_line_name_to_enum(&arg.to_string()),
                    }),
                    "passable" | "pass" => set_actions.push(SetAction::MakePlatformPassable {
                        station_id: station_id,
                        line: transform_line_name_to_enum(&arg.to_string()),
                    }),
                    "queuable" | "qu" => set_actions.push(SetAction::MakePlatformQueuable {
                        station_id: station_id,
                        line: transform_line_name_to_enum(&arg.to_string()),
                    }),
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
