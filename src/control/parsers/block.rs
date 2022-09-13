use crate::control::action::Action;
use crate::control::parsers::shared::try_parse_connection;
use std::collections::HashSet;

pub fn parse_block(input_list: &Vec<&str>) -> Vec<Action> {
    let mut actions: Vec<Action> = vec![];
    if input_list.len() < 2 {
        println!("Block what??");
        return actions;
    }

    match input_list[1] {
        "connection" | "connections" | "conn" | "c" => {
            if input_list.len() < 3 {
                println!("Block which connections??");
                return actions;
            }
            for connection in &input_list[2..] {
                let maybe_station_ids = try_parse_connection(connection);
                match maybe_station_ids {
                    Some(station_ids) => {
                        for i in 0..(station_ids.len() - 1) {
                            actions.push(Action::BlockConnection {
                                ids: HashSet::from([station_ids[i], station_ids[i + 1]]),
                            })
                        }
                    }
                    None => return actions,
                }
            }
        }
        _ => {
            println!("Can't block: {}, not implemented.", input_list[1])
        }
    }

    return actions;
}

pub fn parse_unblock(input_list: &Vec<&str>) -> Vec<Action> {
    let mut actions: Vec<Action> = vec![];
    if input_list.len() < 2 {
        println!("Block what??");
        return actions;
    }

    match input_list[1] {
        "connection" | "connections" | "conn" | "c" => {
            if input_list.len() < 3 {
                println!("Block which connections??");
                return actions;
            }
            for connection in &input_list[2..] {
                let maybe_station_ids = try_parse_connection(connection);
                match maybe_station_ids {
                    Some(station_ids) => {
                        for i in 0..(station_ids.len() - 1) {
                            actions.push(Action::UnblockConnection {
                                ids: HashSet::from([station_ids[i], station_ids[i + 1]]),
                            })
                        }
                    }
                    None => return actions,
                }
            }
        }
        _ => {
            println!("Can't unblock: {}, not implemented.", input_list[1])
        }
    }

    return actions;
}

// pub fn parse_ac(input_list: &Vec<&str>, action: Action) -> Vec<Action> {
//     let mut actions: Vec<Action> = vec![];
//     if input_list.len() < 2 {
//         println!("Block what??");
//         return actions;
//     }

//     match input_list[1] {
//         "connection" | "connections" | "conn" | "c" => {
//             if input_list.len() < 3 {
//                 println!("Block which connections??");
//                 return actions;
//             }
//             for connection in &input_list[2..] {
//                 let maybe_station_ids = try_parse_connection(connection);
//                 match maybe_station_ids {
//                     Some(station_ids) => {
//                         for i in 0..(station_ids.len() - 1) {
//                             actions.push(Action::UnblockConnection {
//                                 ids: HashSet::from([station_ids[i], station_ids[i + 1]]),
//                             })
//                         }
//                     }
//                     None => return actions,
//                 }
//             }
//         }
//         "station" | "st" => {
//             if input_list.len() < 3 {
//                 println!("Unblock which stations??");
//                 return actions;
//             }
//             for arg in &input_list[2..] {
//                 let ids = parse_id_list_and_ranges(arg);
//                 for id in ids {
//                     actions.push(Action::UnblockStation { id: id })
//                 }
//             }
//         }
//         "platform" | "pl" => {
//             if input_list.len() < 4 {
//                 println!("Unblock which platforms??");
//                 return actions;
//             }
//             let station_id: i32 = FromStr::from_str(input_list[2]).unwrap();
//             for arg in &input_list[3..] {
//                 actions.push(Action::UnblockPlatform {
//                     station_id: station_id,
//                     line: arg.to_string(),
//                 })
//             }
//         }
//         _ => {
//             println!("Can't unblock: {}, not implemented.", input_list[1])
//         }
//     }

//     return actions;
// }
