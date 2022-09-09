use crate::control::action::SetAction;
use crate::control::parsers::shared::parse_id_list_and_ranges;

pub fn parse_visualize(input_list: &Vec<&str>) -> Vec<SetAction> {
    let mut set_actions: Vec<SetAction> = vec![];
    if input_list.len() < 2 {
        println!("Visualize what??");
        return set_actions;
    }

    // TODO: viuslize all entities
    match input_list[1] {
        "person" | "people" | "ps" => {
            if input_list.len() < 3 {
                println!("Visualize which person/people??");
                return set_actions;
            }
            let mut follow = false;
            for arg in &input_list[2..] {
                match arg {
                    &"--follow" | &"-f" => {
                        follow = true;
                    }
                    id_or_ids => {
                        let ids = parse_id_list_and_ranges(id_or_ids);
                        for id in ids {
                            set_actions.push(SetAction::ShowPerson {
                                id: id,
                                follow: follow,
                            })
                        }
                    }
                }
            }
        }
        "pod" | "pods" => {
            if input_list.len() < 3 {
                println!("Visualize which pod/pods??");
                return set_actions;
            }
            let mut permanent = false;
            for arg in &input_list[2..] {
                match arg {
                    &"--permanent" | &"-p" => {
                        permanent = true;
                    }
                    id_or_ids => {
                        let ids = parse_id_list_and_ranges(id_or_ids);
                        for id in ids {
                            set_actions.push(SetAction::ShowPod {
                                id: id,
                                permanent: permanent,
                            })
                        }
                    }
                }
            }
        }
        "station" | "stations" | "st" => {
            if input_list.len() < 3 {
                println!("Visualize which person/people??");
                return set_actions;
            }
            let mut permanent = false;
            for arg in &input_list[2..] {
                match arg {
                    &"--permanent" | &"-p" => {
                        permanent = true;
                    }
                    id_or_ids => {
                        let ids = parse_id_list_and_ranges(id_or_ids);
                        for id in ids {
                            set_actions.push(SetAction::ShowStation {
                                id: id,
                                permanent: permanent,
                            })
                        }
                    }
                }
            }
        }
        any => {
            println!("Can't visualize: {}, not implemented.", any)
        }
    }

    return set_actions;
}

pub fn parse_hide(input_list: &Vec<&str>) -> Vec<SetAction> {
    let mut set_actions: Vec<SetAction> = vec![];
    if input_list.len() < 2 {
        println!("Hide what??");
        return set_actions;
    }

    match input_list[1] {
        "person" | "people" | "p" => {
            if input_list.len() < 3 {
                println!("Hide which person/people??");
                return set_actions;
            }
            for arg in &input_list[2..] {
                {
                    let ids = parse_id_list_and_ranges(arg);
                    for id in ids {
                        set_actions.push(SetAction::HidePerson { id: id })
                    }
                }
            }
        }
        "pod" | "pods" => {
            if input_list.len() < 3 {
                println!("Hide which pod/pods??");
                return set_actions;
            }
            for arg in &input_list[2..] {
                {
                    let ids = parse_id_list_and_ranges(arg);
                    for id in ids {
                        set_actions.push(SetAction::HidePod { id: id })
                    }
                }
            }
        }
        "station" | "stations" | "st" => {
            if input_list.len() < 3 {
                println!("Hide which station/stations??");
                return set_actions;
            }
            for arg in &input_list[2..] {
                {
                    let ids = parse_id_list_and_ranges(arg);
                    for id in ids {
                        set_actions.push(SetAction::HideStation { id: id })
                    }
                }
            }
        }
        any => {
            println!("Can't hide: {}, not implemented.", any)
        }
    }

    return set_actions;
}