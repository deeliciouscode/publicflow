use crate::control::action::Action;
use crate::helper::functions::parse_str_to_line_and_directions;
use std::str::FromStr;

pub fn parse_spawn(input_list: &Vec<&str>) -> Vec<Action> {
    let mut actions: Vec<Action> = vec![];
    if input_list.len() < 2 {
        println!("Spawn what??");
        return actions;
    }

    // make platform op 0 u1 -> make platform op 0 u1+-
    match input_list[1] {
        "pod" => {
            if input_list.len() < 4 {
                println!("Too few arguments, spawn pod how??");
                return actions;
            }

            // the idea of force is, that pods can spawn in stations that usually do not spawn pods
            // this is primarily useful for startup scripts to initialize the network
            let mut force = false;
            let mut first_id_index = 3;
            if input_list.contains(&"--force") {
                force = true;
                first_id_index += 1;
            }

            let station_id: i32 = FromStr::from_str(input_list[first_id_index - 1]).unwrap();
            for arg in &input_list[first_id_index..] {
                let (line, directions) = parse_str_to_line_and_directions(&arg.to_string());
                for direction in directions {
                    actions.push(Action::SpawnPod {
                        station_id: station_id,
                        line_name: line.clone(),
                        direction: direction,
                        force: force,
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

    return actions;
}
