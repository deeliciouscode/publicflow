use crate::control::action::Action;
use crate::helper::functions::parse_str_to_line_and_directions;
use std::str::FromStr;

pub fn parse_spawn(input_list: &Vec<&str>) -> Vec<Action> {
    let mut effect_actions: Vec<Action> = vec![];
    if input_list.len() < 2 {
        println!("Spawn what??");
        return effect_actions;
    }

    // make platform op 0 u1 -> make platform op 0 u1+-
    match input_list[1] {
        "pod" => {
            if input_list.len() < 4 {
                println!("Too few arguments, spawn pod how??");
                return effect_actions;
            }
            let station_id: i32 = FromStr::from_str(input_list[2]).unwrap();
            for arg in &input_list[3..] {
                let (line, directions) = parse_str_to_line_and_directions(&arg.to_string());
                for direction in directions {
                    effect_actions.push(Action::SpawnPod {
                        station_id: station_id,
                        line_name: line.clone(),
                        direction: direction,
                    })
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

    return effect_actions;
}
