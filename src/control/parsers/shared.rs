use std::str::FromStr;

pub fn parse_id_list_and_ranges(id_or_ids: &str) -> Vec<i32> {
    let mut ids = vec![];
    if id_or_ids.contains("..") {
        let split: Vec<&str> = id_or_ids.split("..").collect();
        // Will fail if on is not parsable
        let from: i32 = FromStr::from_str(split[0]).unwrap();
        let to: i32 = FromStr::from_str(split[1]).unwrap();
        for id in from..=to {
            ids.push(id)
        }
    } else {
        let maybe_id = FromStr::from_str(id_or_ids);
        match maybe_id {
            Ok(id) => ids.push(id),
            Err(_) => println!(
                "Couldn't parse \'{}\' into an id or id range, use \'x..y\'",
                id_or_ids
            ),
        }
    }
    ids
}

pub fn try_parse_connection(connection: &&str) -> Option<Vec<i32>> {
    let station_ids_str: Vec<&str> = connection.split("-").collect();

    if station_ids_str.len() < 2 {
        println!("Thats not a connection: {}", connection);
        return None;
    }

    fn try_make_i32(s: &&str) -> i32 {
        let maybe_id = FromStr::from_str(s);
        match maybe_id {
            Ok(id) => id,
            Err(_) => {
                println!("Couldn't parse \'{}\' into i32", s);
                return -1;
            }
        }
    }

    let station_ids: Vec<i32> = station_ids_str.iter().map(try_make_i32).collect();

    if station_ids.contains(&-1) {
        return None;
    }

    return Some(station_ids);
}
