use crate::control::action::Action;
use crate::control::parsers::shared::parse_id_list_and_ranges;

pub fn parse_dump(input_list: &Vec<&str>) -> Vec<Action> {
    let mut actions: Vec<Action> = vec![];
    if input_list.len() < 2 {
        println!("Dump what??");
        return actions;
    }

    match input_list[1] {
        "person" | "people" => {
            let mut avg = false;
            if input_list.contains(&"--avg") {
                avg = true;
            }

            let mut all = false;
            if input_list.contains(&"--all") {
                all = true;
            }

            if avg {
                actions.push(Action::DumpMetricsPeople { all: all, avg: avg });
                return actions;
            }

            // else
            for arg in &input_list[2..] {
                let ids = parse_id_list_and_ranges(arg);
                for id in ids {
                    actions.push(Action::DumpMetricsPerson { person_id: id })
                }
            }
        }
        "pod" | "pods" => {
            let mut avg = false;
            if input_list.contains(&"--avg") {
                avg = true;
            }

            let mut all = false;
            if input_list.contains(&"--all") {
                all = true;
            }

            if all || avg {
                actions.push(Action::DumpMetricsPods { all: all, avg: avg });
                return actions;
            }

            // else
            for arg in &input_list[2..] {
                let ids = parse_id_list_and_ranges(arg);
                for id in ids {
                    actions.push(Action::DumpMetricsPod { pod_id: id })
                }
            }
        }
        "config" => actions.push(Action::DumpConfig),
        _ => {
            println!("Can't dump: {}, not implemented.", input_list[1])
        }
    }

    return actions;
}
