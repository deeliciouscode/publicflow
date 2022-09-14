use crate::control::action::Action;
use crate::control::parsers::shared::parse_id_list_and_ranges;

pub fn parse_route(input_list: &Vec<&str>) -> Vec<Action> {
    let mut actions: Vec<Action> = vec![];
    if input_list.len() < 2 {
        println!("Route what??");
        return actions;
    }

    match input_list[1] {
        "person" | "people" | "p" => {
            if input_list[2..].len() < 2 {
                println!("Route which person/people to where??");
                println!("Syntax: route person [--random | -r] [id id | lower..upper | id lower..upper] to_station");
                println!("If --random is used, no to_station is needed.");
                return actions;
            }

            let mut stay_there = false;
            let mut random_station = false;
            let mut first_id_index = 2;
            let mut last_id_index = input_list.len() - 1;
            if input_list.contains(&"--stay") {
                stay_there = true;
                first_id_index += 1;
            }
            if input_list.contains(&"--random") {
                random_station = true;
                first_id_index += 1;
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
                    actions.push(Action::RoutePerson {
                        id: id,
                        station_id: station_id,
                        stay_there: stay_there,
                        random_station: random_station,
                    })
                }
            }
        }
        _ => {
            println!("Can't route: {}, not implemented.", input_list[1])
        }
    }

    return actions;
}
