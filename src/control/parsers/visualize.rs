use crate::control::action::Action;
use crate::control::parsers::shared::parse_id_list_and_ranges;

pub fn parse_visualize(input_list: &Vec<&str>) -> Vec<Action> {
    let mut actions: Vec<Action> = vec![];
    if input_list.len() < 2 {
        println!("Visualize what??");
        return actions;
    }

    // TODO: viuslize all entities
    match input_list[1] {
        "person" | "people" | "ps" => {
            if input_list.len() < 3 {
                println!("Visualize which person/people??");
                return actions;
            }
            for arg in &input_list[2..] {
                match arg {
                    id_or_ids => {
                        let ids = parse_id_list_and_ranges(id_or_ids);
                        for id in ids {
                            actions.push(Action::ShowPerson { id: id })
                        }
                    }
                }
            }
        }
        "pod" | "pods" => {
            if input_list.len() < 3 {
                println!("Visualize which pod/pods??");
                return actions;
            }
            for arg in &input_list[2..] {
                match arg {
                    id_or_ids => {
                        let ids = parse_id_list_and_ranges(id_or_ids);
                        for id in ids {
                            actions.push(Action::ShowPod { id: id })
                        }
                    }
                }
            }
        }
        "station" | "stations" | "st" => {
            if input_list.len() < 3 {
                println!("Visualize which person/people??");
                return actions;
            }
            for arg in &input_list[2..] {
                match arg {
                    id_or_ids => {
                        let ids = parse_id_list_and_ranges(id_or_ids);
                        for id in ids {
                            actions.push(Action::ShowStation { id: id })
                        }
                    }
                }
            }
        }
        any => {
            println!("Can't visualize: {}, not implemented.", any)
        }
    }

    return actions;
}

pub fn parse_hide(input_list: &Vec<&str>) -> Vec<Action> {
    let mut actions: Vec<Action> = vec![];
    if input_list.len() < 2 {
        println!("Hide what??");
        return actions;
    }

    match input_list[1] {
        "person" | "people" | "p" => {
            if input_list.len() < 3 {
                println!("Hide which person/people??");
                return actions;
            }
            for arg in &input_list[2..] {
                {
                    let ids = parse_id_list_and_ranges(arg);
                    for id in ids {
                        actions.push(Action::HidePerson { id: id })
                    }
                }
            }
        }
        "pod" | "pods" => {
            if input_list.len() < 3 {
                println!("Hide which pod/pods??");
                return actions;
            }
            for arg in &input_list[2..] {
                {
                    let ids = parse_id_list_and_ranges(arg);
                    for id in ids {
                        actions.push(Action::HidePod { id: id })
                    }
                }
            }
        }
        "station" | "stations" | "st" => {
            if input_list.len() < 3 {
                println!("Hide which station/stations??");
                return actions;
            }
            for arg in &input_list[2..] {
                {
                    let ids = parse_id_list_and_ranges(arg);
                    for id in ids {
                        actions.push(Action::HideStation { id: id })
                    }
                }
            }
        }
        any => {
            println!("Can't hide: {}, not implemented.", any)
        }
    }

    return actions;
}
