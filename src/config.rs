use crate::connection::Connection;
use crate::line::Line;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::yaml::Yaml;
use yaml_rust::YamlLoader;

// CONSTANTS
pub const _SPEED_FACTOR: u64 = 1000;
pub const _SIMULATION_DURATION: u64 = 1000;

// pub const CONFIG_PATH: &str = "./config/network_simple.yaml";
// pub const MAX_XY: (f32, f32) = (3.0, 2.0);
pub const CONFIG_PATH: &str = "./config/ubahn.yaml";
pub const STATIONS_PATH: &str = "./config/stations_in_lines.yaml";
pub const MAX_XY: (f32, f32) = (70., 40.);
pub const SCREEN_SIZE: (f32, f32) = (1920.0, 1150.0);
pub const OFFSET: f32 = 100.0;
pub const SIDELEN_STATION: f32 = 50.;
pub const RADIUS_STATION: f32 = 5.;
pub const SIDELEN_POD: f32 = 30.0;
pub const WIDTH_POD: f32 = 30.0;
pub const LENGTH_POD: f32 = 30.0;
pub const WIDTH_LINE: f32 = 5.0;
pub const DESIRED_FPS: u32 = 60; // TODO: decouple game speed from draw rate
pub const POD_CAPACITY: i32 = 20;
pub const TRANSITION_TIME: i32 = 30;
pub const POD_SPAWN_RATE: i32 = 30; // every so many seconds a pod is spawned till enough are there
pub const VSYNC: bool = true;

// EXTERNAL CONFIG
pub fn load_yaml(file: &str) -> Yaml {
    let mut file = File::open(file).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let docs = YamlLoader::load_from_str(&contents).unwrap();
    return docs[0].clone();
}

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub n_stations: i32,
    pub coordinates_map: HashMap<i32, (String, String, (f32, f32))>,
    pub edge_map: HashMap<i32, HashSet<i32>>,
    pub lines: Vec<Line>,
    pub pods: PodsConfig,
}

#[derive(Debug, Clone)]
pub struct PodsConfig {
    pub n_pods: i32,
}

#[derive(Debug, Clone)]
pub struct PeopleConfig {
    pub n_people: i32,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub network: NetworkConfig,
    pub people: PeopleConfig,
}

pub fn parse_raw_config(raw_config: &Yaml, raw_stations: &Yaml) -> Config {
    let mut n_stations: i64 = 0;
    let mut coordinates_map: HashMap<i32, (String, bool, bool, (i32, i32))> = HashMap::new();
    let mut lines: Vec<Line> = vec![];
    let mut n_pods: i64 = 0;
    let mut n_people: i64 = 0;
    let mut edge_map: HashMap<i32, HashSet<i32>> = HashMap::new();

    // This whole construct essentially parses the raw Yaml typed structure we get into the more
    // usable Config structure from above.
    // It only respects correctly formatted yamls.
    // TODO: introduce a validator or something that panics if yaml is incorrectly formatted.
    if let Yaml::Hash(config_hash) = raw_config {
        if let Some(network_yaml) = config_hash.get(&Yaml::String(String::from("network"))) {
            if let Yaml::Hash(network_hash) = network_yaml {
                if let Some(coordinates_yaml) =
                    network_hash.get(&Yaml::String(String::from("station_coordinates")))
                {
                    if let Yaml::Hash(coordinates_hash) = coordinates_yaml {
                        // println!("{:?}", coordinates_hash);

                        let mut coordinates_clone = coordinates_hash.clone();
                        let coordinate_entries = coordinates_clone.entries();
                        for entry in coordinate_entries {
                            let key_yaml = entry.key();
                            let values_yaml = entry.get();
                            let mut name: String = String::from("");
                            let mut id_i32: i32 = -1;
                            let mut is_node_bool: bool = false;
                            let mut can_spawn_bool: bool = false;
                            let mut x_i32: i32 = -1;
                            let mut y_i32: i32 = -1;

                            if let Yaml::String(key_str) = key_yaml {
                                name = key_str.clone()
                            }

                            if let Yaml::Hash(values_hash) = values_yaml {
                                if let Some(id_yaml) =
                                    values_hash.get(&Yaml::String(String::from("id")))
                                {
                                    if let Yaml::Integer(station_id) = id_yaml {
                                        id_i32 = *station_id as i32;
                                    }
                                }
                                if let Some(is_node_yaml) =
                                    values_hash.get(&Yaml::String(String::from("is_node")))
                                {
                                    if let Yaml::Boolean(is_node) = is_node_yaml {
                                        is_node_bool = *is_node;
                                    }
                                }
                                if let Some(can_spawn_yaml) =
                                    values_hash.get(&Yaml::String(String::from("can_spawn")))
                                {
                                    if let Yaml::Boolean(can_spawn) = can_spawn_yaml {
                                        can_spawn_bool = *can_spawn;
                                    }
                                }
                                if let Some(coords_yaml) =
                                    values_hash.get(&Yaml::String(String::from("coords")))
                                {
                                    if let Yaml::Hash(coords_hash) = coords_yaml {
                                        println!("{:?}", coords_hash);
                                        if let Some(x_yaml) =
                                            coords_hash.get(&Yaml::String(String::from("x")))
                                        {
                                            if let Yaml::Integer(x_int) = x_yaml {
                                                x_i32 = *x_int as i32
                                            }
                                        }
                                        if let Some(y_yaml) =
                                            coords_hash.get(&Yaml::String(String::from("y")))
                                        {
                                            if let Yaml::Integer(y_int) = y_yaml {
                                                y_i32 = *y_int as i32
                                            }
                                        }
                                    }
                                }
                            }

                            coordinates_map.insert(
                                id_i32,
                                (name, is_node_bool, can_spawn_bool, (x_i32, y_i32)),
                            );
                        }
                    }
                }
                if let Some(lines_yaml) = network_hash.get(&Yaml::String(String::from("lines"))) {
                    if let Yaml::Array(lines_array) = lines_yaml {
                        for line_yaml in lines_array {
                            if let Yaml::Hash(line_hash) = line_yaml {
                                let mut stations: Vec<i32> = vec![];
                                let mut distances: Vec<i32> = vec![];
                                let mut circular: bool = false;
                                let mut name: String = String::from("placeholder");

                                if let Some(name_yaml) =
                                    line_hash.get(&Yaml::String(String::from("name")))
                                {
                                    // TODO finish this
                                    if let Yaml::String(name_string) = name_yaml {
                                        name = name_string.clone();
                                    }
                                }
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
                                if let Some(distances_yaml) =
                                    line_hash.get(&Yaml::String(String::from("distances")))
                                {
                                    if let Yaml::Array(distances_array) = distances_yaml {
                                        for distance_yaml in distances_array {
                                            if let Yaml::Integer(distance) = distance_yaml {
                                                distances.push(*distance as i32);
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
                                let connections =
                                    calc_connections(&name, &stations, circular, &distances);
                                let line = Line {
                                    name: name,
                                    stations: stations,
                                    distances: distances,
                                    circular: circular,
                                    connections: connections,
                                };
                                println!("{:?}", line);
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

    let mut coordinates_map_new: HashMap<i32, (String, String, (f32, f32))> = HashMap::new();

    if let Yaml::Array(stations_array) = raw_stations {
        n_stations = stations_array.len() as i64;
        for station_yaml in stations_array {
            if let Yaml::Hash(station_hash) = station_yaml {
                let mut city: String = String::from("placeholder");
                let mut id: i32 = -1;
                let mut lat: f32 = -1.0;
                let mut lon: f32 = -1.0;
                let mut name: String = String::from("placeholder");

                if let Some(city_yaml) = station_hash.get(&Yaml::String(String::from("city"))) {
                    // TODO finish this
                    if let Yaml::String(city_string) = city_yaml {
                        city = city_string.clone();
                    }
                }

                if let Some(id_yaml) = station_hash.get(&Yaml::String(String::from("id"))) {
                    // TODO finish this
                    if let Yaml::Integer(id_int) = id_yaml {
                        id = *id_int as i32;
                    }
                }

                if let Some(lat_yaml) = station_hash.get(&Yaml::String(String::from("lat"))) {
                    // TODO finish this
                    if let Some(lat_float) = lat_yaml.as_f64() {
                        lat = lat_float as f32;
                    }
                }

                if let Some(lon_yaml) = station_hash.get(&Yaml::String(String::from("lon"))) {
                    // TODO finish this
                    if let Some(lon_float) = lon_yaml.as_f64() {
                        lon = lon_float as f32;
                    }
                }

                if let Some(name_yaml) = station_hash.get(&Yaml::String(String::from("name"))) {
                    // TODO finish this
                    if let Yaml::String(name_string) = name_yaml {
                        name = name_string.clone();
                    }
                }

                coordinates_map_new.insert(id, (name, city, (lat, lon)));
            }
        }
    }

    println!("{:?}", coordinates_map_new);

    let pods_config = PodsConfig {
        n_pods: n_pods as i32,
    };

    let people_config = PeopleConfig {
        n_people: n_people as i32,
    };

    let network_config = NetworkConfig {
        n_stations: n_stations as i32,
        coordinates_map: coordinates_map_new,
        edge_map: edge_map,
        lines: lines,
        pods: pods_config,
    };

    Config {
        network: network_config,
        people: people_config,
    }
}

fn calc_connections(
    name: &String,
    station_ids: &Vec<i32>,
    circular: bool,
    distances: &Vec<i32>,
) -> Vec<Connection> {
    let mut connections: Vec<Connection> = vec![];

    // first some verifications
    if circular && station_ids.len() != distances.len() {
        panic!("A circular line must have as many distances as it has stations. Ascertain that this is the case for line {}", name);
    } else if station_ids.len() != distances.len() + 1 {
        panic!("A non circular line must have exactly n-1 distances if it has n stations. Ascertain that this is the case for line {}", name);
    }

    for i in 0..station_ids.len() {
        if i == station_ids.len() - 1 && circular {
            let travel_time = distances[i] / 22; // 80 kmh ~= 22 m/s
            connections.push(Connection {
                station_ids: HashSet::from([station_ids[i], station_ids[0]]),
                travel_time: travel_time,
            });
            break;
        } else if i == station_ids.len() - 1 {
            break;
        } else {
            let travel_time = distances[i] / 22; // 80 kmh ~= 22 m/s
            println!(
                "Connection: {:?} | travel_time: {}",
                (station_ids[i], station_ids[i + 1]),
                travel_time
            );
            connections.push(Connection {
                station_ids: HashSet::from([station_ids[i], station_ids[i + 1]]),
                travel_time: travel_time,
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
