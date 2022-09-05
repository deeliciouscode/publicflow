use crate::control::action::GetAction;
use crate::control::parsers::shared::parse_id_list_and_ranges;

pub fn parse_get(input_list: &Vec<&str>) -> Vec<GetAction> {
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
        "person" | "people" | "ps" => {
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
        "pod" | "pods" => {
            if input_list.len() < 3 {
                println!("Get which pod/pods??");
                return get_actions;
            }
            for arg in &input_list[2..] {
                let ids = parse_id_list_and_ranges(arg);
                for id in ids {
                    get_actions.push(GetAction::GetPod { id: id })
                }
            }
        }
        _ => {
            println!("Can't get: {}, not implemented.", input_list[1])
        }
    }

    return get_actions;
}
