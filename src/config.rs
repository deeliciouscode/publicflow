use crate::connection::Connection;
use crate::line::Line;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::yaml::Yaml;
use yaml_rust::YamlLoader;

// CONSTANTS
pub const SPEED_FACTOR: u64 = 10;

// EXTERNAL CONFIG
pub fn load_file(file: &str) -> Yaml {
    let mut file = File::open(file).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let docs = YamlLoader::load_from_str(&contents).unwrap();
    return docs[0].clone();
}

#[derive(Debug)]
pub struct NetworkConfig {
    pub n_stations: i32,
    pub edge_map: HashMap<i32, HashSet<i32>>,
    pub lines: Vec<Line>,
    pub pods: PodsConfig,
}

#[derive(Debug)]
pub struct PodsConfig {
    pub n_pods: i32,
}

#[derive(Debug)]
pub struct PeopleConfig {
    pub n_people: i32,
}

#[derive(Debug)]
pub struct Config {
    pub network: NetworkConfig,
    pub people: PeopleConfig,
}

pub fn parse_yaml(config_yaml: &Yaml) -> Config {
    let mut n_stations: i64 = 0;
    let mut lines: Vec<Line> = vec![];
    let mut n_pods: i64 = 0;
    let mut n_people: i64 = 0;
    let mut edge_map: HashMap<i32, HashSet<i32>> = HashMap::new();

    // This whole construct essentially parses the raw Yaml typed structure we get into the more
    // usable Config structure from above.
    // It only respects correctly formatted yamls.
    // TODO: introduce a validator or something that panics if yaml is incorrectly formatted.
    if let Yaml::Hash(config_hash) = config_yaml {
        if let Some(network_yaml) = config_hash.get(&Yaml::String(String::from("network"))) {
            if let Yaml::Hash(network_hash) = network_yaml {
                if let Some(n_stations_yaml) =
                    network_hash.get(&Yaml::String(String::from("n_stations")))
                {
                    if let Yaml::Integer(value) = n_stations_yaml {
                        n_stations = *value;
                    }
                }
                if let Some(lines_yaml) = network_hash.get(&Yaml::String(String::from("lines"))) {
                    if let Yaml::Array(lines_array) = lines_yaml {
                        for line_yaml in lines_array {
                            if let Yaml::Hash(line_hash) = line_yaml {
                                let mut stations: Vec<i32> = vec![];
                                let mut circular: bool = false;
                                if let Some(stations_yaml) =
                                    line_hash.get(&Yaml::String(String::from("stations")))
                                {
                                    if let Yaml::Array(stations_array) = stations_yaml {
                                        for station_yaml in stations_array {
                                            if let Yaml::Integer(station_id) = station_yaml {
                                                stations.push(*station_id as i32);
                                            }
                                        }
                                    }
                                }
                                if let Some(circular_yaml) =
                                    line_hash.get(&Yaml::String(String::from("circular")))
                                {
                                    if let Yaml::Boolean(circular_bool) = circular_yaml {
                                        circular = *circular_bool;
                                    }
                                }
                                update_edge_map(&stations, circular, &mut edge_map);
                                let connections = calc_connections(&stations, circular);
                                let line = Line {
                                    stations: stations,
                                    circular: circular,
                                    connections: connections,
                                };
                                lines.push(line);
                            }
                        }
                    }
                }
                if let Some(pods_yaml) = network_hash.get(&Yaml::String(String::from("pods"))) {
                    if let Yaml::Hash(pods_hash) = pods_yaml {
                        if let Some(n_pods_yaml) =
                            pods_hash.get(&Yaml::String(String::from("n_pods")))
                        {
                            if let Yaml::Integer(value) = n_pods_yaml {
                                n_pods = *value;
                            }
                        }
                    }
                }
            }
        }
        if let Some(people_yaml) = config_hash.get(&Yaml::String(String::from("people"))) {
            if let Yaml::Hash(people_hash) = people_yaml {
                if let Some(n_people_yaml) =
                    people_hash.get(&Yaml::String(String::from("n_people")))
                {
                    if let Yaml::Integer(value) = n_people_yaml {
                        n_people = *value;
                    }
                }
            }
        }
    }

    // println!(
    //     "n_stations: {}, n_pods: {}, n_people: {}",
    //     n_stations, n_pods, n_people
    // );

    // println!("lines: {:?}", lines);

    let pods_config = PodsConfig {
        n_pods: n_pods as i32,
    };

    let people_config = PeopleConfig {
        n_people: n_people as i32,
    };

    let network_config = NetworkConfig {
        n_stations: n_stations as i32,
        edge_map: edge_map,
        lines: lines,
        pods: pods_config,
    };

    Config {
        network: network_config,
        people: people_config,
    }
}

fn calc_connections(station_ids: &Vec<i32>, circular: bool) -> Vec<Connection> {
    let mut connections: Vec<Connection> = vec![];

    for i in 0..station_ids.len() {
        if i == station_ids.len() - 1 && circular {
            connections.push(Connection {
                station_ids: HashSet::from([station_ids[i], station_ids[0]]),
                travel_time: 20,
            });
            break;
        } else if i == station_ids.len() - 1 {
            break;
        } else {
            connections.push(Connection {
                station_ids: HashSet::from([station_ids[i], station_ids[i + 1]]),
                travel_time: 20,
            });
        }
    }
    connections
}

fn update_edge_map(
    station_ids: &Vec<i32>,
    circular: bool,
    edge_map: &mut HashMap<i32, HashSet<i32>>,
) {
    for i in 0..station_ids.len() {
        let station_id = station_ids[i];
        if !edge_map.contains_key(&station_id) {
            edge_map.insert(station_id, HashSet::new());
        }

        if i == station_ids.len() - 1 {
            if let Some(mut_hashset) = edge_map.get_mut(&station_id) {
                mut_hashset.insert(station_ids[i - 1]);
                if circular {
                    mut_hashset.insert(station_ids[0]);
                }
            }
        } else if i == 0 {
            if let Some(mut_hashset) = edge_map.get_mut(&station_id) {
                mut_hashset.insert(station_ids[i + 1]);
                if circular {
                    mut_hashset.insert(station_ids[station_ids.len() - 1]);
                }
            }
        } else {
            if let Some(mut_hashset) = edge_map.get_mut(&station_id) {
                mut_hashset.insert(station_ids[i - 1]);
                mut_hashset.insert(station_ids[i + 1]);
            }
        }
    }
}

// Example of a yaml structure we get from load_file()
// Hash({
//     String("network"): Hash({
//         String("n_stations"): Integer(13),
//         String("lines"): Array([
//             Hash({
//                 String("stations"): Array([Integer(0), Integer(1), Integer(2), Integer(4)]),
//                 String("circular"): Boolean(false)}),
//             Hash({
//                 String("stations"): Array([Integer(4), Integer(5), Integer(6)]),
//                 String("circular"): Boolean(false)}),
//             Hash({
//                 String("stations"): Array([Integer(7), Integer(8), Integer(9), Integer(10)]),
//                 String("circular"): Boolean(false)}),
//             Hash({
//                 String("stations"): Array([Integer(2), Integer(6), Integer(9), Integer(11), Integer(12)]),
//                 String("circular"): Boolean(false)}),
//             Hash({
//                 String("stations"): Array([Integer(0), Integer(1), Integer(2), Integer(3), Integer(10), Integer(9), Integer(8), Integer(7), Integer(4)]),
//                 String("circular"): Boolean(true)})]),
//         String("pods"):
//             Hash({String("n_pods"): Integer(13)})}),
//     String("people"): Hash({String("n_people"): Integer(100)})})
