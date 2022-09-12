use crate::control::action::Action;
use crate::control::parsers::shared::parse_id_list_and_ranges;

pub fn parse_get(input_list: &Vec<&str>) -> Vec<Action> {
    let mut actions: Vec<Action> = vec![];
    if input_list.len() < 2 {
        println!("Get what??");
        return actions;
    }

    match input_list[1] {
        "station" | "stations" | "st" => {
            if input_list.len() < 3 {
                println!("Get which stations??");
                return actions;
            }
            for arg in &input_list[2..] {
                let ids = parse_id_list_and_ranges(arg);
                for id in ids {
                    actions.push(Action::GetStation { id: id })
                }
            }
        }
        "person" | "people" | "ps" => {
            if input_list.len() < 3 {
                println!("Get which person/people??");
                return actions;
            }
            for arg in &input_list[2..] {
                let ids = parse_id_list_and_ranges(arg);
                for id in ids {
                    actions.push(Action::GetPerson { id: id })
                }
            }
        }
        "pod" | "pods" => {
            if input_list.len() < 3 {
                println!("Get which pod/pods??");
                return actions;
            }
            for arg in &input_list[2..] {
                let ids = parse_id_list_and_ranges(arg);
                for id in ids {
                    actions.push(Action::GetPod { id: id })
                }
            }
        }
        _ => {
            println!("Can't get: {}, not implemented.", input_list[1])
        }
    }

    return actions;
}
