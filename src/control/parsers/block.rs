use crate::control::action::SetAction;
use crate::control::parsers::shared::{parse_id_list_and_ranges, try_parse_connection};
use std::collections::HashSet;
use std::str::FromStr;

pub fn parse_block(input_list: &Vec<&str>) -> Vec<SetAction> {
    let mut set_actions: Vec<SetAction> = vec![];
    if input_list.len() < 2 {
        println!("Block what??");
        return set_actions;
    }

    match input_list[1] {
        "connection" | "connections" | "conn" | "c" => {
            if input_list.len() < 3 {
                println!("Block which connections??");
                return set_actions;
            }
            for connection in &input_list[2..] {
                let maybe_station_ids = try_parse_connection(connection);
                match maybe_station_ids {
                    Some(station_ids) => {
                        for i in 0..(station_ids.len() - 1) {
                            set_actions.push(SetAction::BlockConnection {
                                ids: HashSet::from([station_ids[i], station_ids[i + 1]]),
                            })
                        }
                    }
                    None => return set_actions,
                }
            }
        }
        "station" | "st" => {
            if input_list.len() < 3 {
                println!("Block which stations??");
                return set_actions;
            }
            for arg in &input_list[2..] {
                let ids = parse_id_list_and_ranges(arg);
                for id in ids {
                    set_actions.push(SetAction::BlockStation { id: id })
                }
            }
        }
        "platform" | "pl" => {
            if input_list.len() < 4 {
                println!("Block which platforms??");
                return set_actions;
            }
            let station_id: i32 = FromStr::from_str(input_list[2]).unwrap();
            for arg in &input_list[3..] {
                set_actions.push(SetAction::BlockPlatform {
                    station_id: station_id,
                    line: arg.to_string(),
                })
            }
        }
        _ => {
            println!("Can't block: {}, not implemented.", input_list[1])
        }
    }

    return set_actions;
}

pub fn parse_unblock(input_list: &Vec<&str>) -> Vec<SetAction> {
    let mut set_actions: Vec<SetAction> = vec![];
    if input_list.len() < 2 {
        println!("Block what??");
        return set_actions;
    }

    match input_list[1] {
        "connection" | "connections" | "conn" | "c" => {
            if input_list.len() < 3 {
                println!("Block which connections??");
                return set_actions;
            }
            for connection in &input_list[2..] {
                let maybe_station_ids = try_parse_connection(connection);
                match maybe_station_ids {
                    Some(station_ids) => {
                        for i in 0..(station_ids.len() - 1) {
                            set_actions.push(SetAction::UnblockConnection {
                                ids: HashSet::from([station_ids[i], station_ids[i + 1]]),
                            })
                        }
                    }
                    None => return set_actions,
                }
            }
        }
        "station" | "st" => {
            if input_list.len() < 3 {
                println!("Unblock which stations??");
                return set_actions;
            }
            for arg in &input_list[2..] {
                let ids = parse_id_list_and_ranges(arg);
                for id in ids {
                    set_actions.push(SetAction::UnblockStation { id: id })
                }
            }
        }
        "platform" | "pl" => {
            if input_list.len() < 4 {
                println!("Unblock which platforms??");
                return set_actions;
            }
            let station_id: i32 = FromStr::from_str(input_list[2]).unwrap();
            for arg in &input_list[3..] {
                set_actions.push(SetAction::UnblockPlatform {
                    station_id: station_id,
                    line: arg.to_string(),
                })
            }
        }
        _ => {
            println!("Can't unblock: {}, not implemented.", input_list[1])
        }
    }

    return set_actions;
}

// pub fn parse_ac(input_list: &Vec<&str>, action: SetAction) -> Vec<SetAction> {
//     let mut set_actions: Vec<SetAction> = vec![];
//     if input_list.len() < 2 {
//         println!("Block what??");
//         return set_actions;
//     }

//     match input_list[1] {
//         "connection" | "connections" | "conn" | "c" => {
//             if input_list.len() < 3 {
//                 println!("Block which connections??");
//                 return set_actions;
//             }
//             for connection in &input_list[2..] {
//                 let maybe_station_ids = try_parse_connection(connection);
//                 match maybe_station_ids {
//                     Some(station_ids) => {
//                         for i in 0..(station_ids.len() - 1) {
//                             set_actions.push(SetAction::UnblockConnection {
//                                 ids: HashSet::from([station_ids[i], station_ids[i + 1]]),
//                             })
//                         }
//                     }
//                     None => return set_actions,
//                 }
//             }
//         }
//         "station" | "st" => {
//             if input_list.len() < 3 {
//                 println!("Unblock which stations??");
//                 return set_actions;
//             }
//             for arg in &input_list[2..] {
//                 let ids = parse_id_list_and_ranges(arg);
//                 for id in ids {
//                     set_actions.push(SetAction::UnblockStation { id: id })
//                 }
//             }
//         }
//         "platform" | "pl" => {
//             if input_list.len() < 4 {
//                 println!("Unblock which platforms??");
//                 return set_actions;
//             }
//             let station_id: i32 = FromStr::from_str(input_list[2]).unwrap();
//             for arg in &input_list[3..] {
//                 set_actions.push(SetAction::UnblockPlatform {
//                     station_id: station_id,
//                     line: arg.to_string(),
//                 })
//             }
//         }
//         _ => {
//             println!("Can't unblock: {}, not implemented.", input_list[1])
//         }
//     }

//     return set_actions;
// }
