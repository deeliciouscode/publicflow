use crate::config::structs::Config;
use crate::control::action::Action;
use crate::helper::enums::Direction;
use crate::helper::enums::LineName;
use crate::helper::functions::parse_str_to_line_and_directions;
use std::str::FromStr;
use std::time::Duration;

pub fn parse_spawn(input_list: &Vec<&str>, config: &Config) -> Vec<Action> {
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
        "pods" => {
            if input_list.len() < 4 {
                println!("Too few arguments, spawn pods how??");
                return actions;
            }

            if input_list.len() > 4 {
                println!("Too many arguments, this is a very restrictive command at the moment.");
                return actions;
            }

            if input_list[2] != "fill" {
                println!("Only fill supported right now.");
                return actions;
            }

            let (line_name, directions) = parse_str_to_line_and_directions(input_list[3]);

            actions = generate_actions_to_fill_line(line_name, directions, config);
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

fn generate_actions_to_fill_line(
    line_name: LineName,
    directions: Vec<Direction>,
    config: &Config,
) -> Vec<Action> {
    let mut actions = vec![];

    for line in &config.network.lines {
        if line.name == line_name {
            let conns = &line.connections;
            let mut travel_time = conns.len() as i32 * config.logic.pod_in_station_seconds;
            for conn in conns {
                travel_time += conn.travel_time;
            }

            if line.circular {
                let ideal_time_interval = 3600 / config.logic.line_pods_per_hour;
                let mut n_pods = travel_time / ideal_time_interval;
                if (travel_time % ideal_time_interval) as f32 > (0.5 * ideal_time_interval as f32) {
                    n_pods += 1;
                }

                let time_interval = travel_time / n_pods;
                println!(
                    "line_name: {:?} - travel_time: {} - ideal_time_interval: {} - n_pods: {} - time_interval: {}",
                    line_name, travel_time, ideal_time_interval, n_pods, time_interval
                );

                for _ in 0..n_pods {
                    for direction in &directions {
                        actions.push(Action::SpawnPod {
                            station_id: line.stations[0],
                            line_name: line_name.clone(),
                            direction: direction.clone(),
                            force: false,
                        })
                    }
                    actions.push(Action::Sleep {
                        duration: Duration::from_millis(
                            (time_interval as u64 * 1000) / config.logic.speed_multiplier as u64,
                        ),
                    })
                }
            } else {
                let ideal_time_interval = 3600 / config.logic.line_pods_per_hour;
                let mut n_pods = travel_time / ideal_time_interval;
                if (travel_time % ideal_time_interval) as f32 > (0.5 * ideal_time_interval as f32) {
                    n_pods += 1;
                }

                let time_interval = travel_time / n_pods;
                println!(
                    "line_name: {:?} - travel_time: {} - ideal_time_interval: {} - n_pods: {} - time_interval: {}",
                    line_name, travel_time, ideal_time_interval, n_pods, time_interval
                );

                for _ in 0..n_pods {
                    for direction in &directions {
                        let station_id;
                        if *direction == Direction::Pos {
                            station_id = *line.stations.first().unwrap();
                        } else {
                            station_id = *line.stations.last().unwrap();
                        }
                        actions.push(Action::SpawnPod {
                            station_id: station_id,
                            line_name: line_name.clone(),
                            direction: direction.clone(),
                            force: false,
                        })
                    }
                    actions.push(Action::Sleep {
                        duration: Duration::from_millis(
                            (time_interval as u64 * 1000) / config.logic.speed_multiplier as u64,
                        ),
                    })
                }
            }
        }
    }

    actions
}
