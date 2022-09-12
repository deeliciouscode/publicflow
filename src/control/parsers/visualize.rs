use crate::control::action::Action;
use crate::control::parsers::shared::parse_id_list_and_ranges;

pub fn parse_visualize(input_list: &Vec<&str>) -> Vec<Action> {
    let mut effect_actions: Vec<Action> = vec![];
    if input_list.len() < 2 {
        println!("Visualize what??");
        return effect_actions;
    }

    // TODO: viuslize all entities
    match input_list[1] {
        "person" | "people" | "ps" => {
            if input_list.len() < 3 {
                println!("Visualize which person/people??");
                return effect_actions;
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
                            effect_actions.push(Action::ShowPerson {
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
                return effect_actions;
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
                            effect_actions.push(Action::ShowPod {
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
                return effect_actions;
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
                            effect_actions.push(Action::ShowStation {
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

    return effect_actions;
}

pub fn parse_hide(input_list: &Vec<&str>) -> Vec<Action> {
    let mut effect_actions: Vec<Action> = vec![];
    if input_list.len() < 2 {
        println!("Hide what??");
        return effect_actions;
    }

    match input_list[1] {
        "person" | "people" | "p" => {
            if input_list.len() < 3 {
                println!("Hide which person/people??");
                return effect_actions;
            }
            for arg in &input_list[2..] {
                {
                    let ids = parse_id_list_and_ranges(arg);
                    for id in ids {
                        effect_actions.push(Action::HidePerson { id: id })
                    }
                }
            }
        }
        "pod" | "pods" => {
            if input_list.len() < 3 {
                println!("Hide which pod/pods??");
                return effect_actions;
            }
            for arg in &input_list[2..] {
                {
                    let ids = parse_id_list_and_ranges(arg);
                    for id in ids {
                        effect_actions.push(Action::HidePod { id: id })
                    }
                }
            }
        }
        "station" | "stations" | "st" => {
            if input_list.len() < 3 {
                println!("Hide which station/stations??");
                return effect_actions;
            }
            for arg in &input_list[2..] {
                {
                    let ids = parse_id_list_and_ranges(arg);
                    for id in ids {
                        effect_actions.push(Action::HideStation { id: id })
                    }
                }
            }
        }
        any => {
            println!("Can't hide: {}, not implemented.", any)
        }
    }

    return effect_actions;
}
