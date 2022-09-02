use crate::connection::{Connection, ConnectionKind};
use crate::line::Line;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::yaml::Yaml;
use yaml_rust::YamlLoader;

// CONSTANTS
pub const _SPEED_FACTOR: u64 = 1000;
pub const _SIMULATION_DURATION: u64 = 1000;

pub const CONFIG_PATH: &str = "./config/config.yaml";

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
    pub coordinates_map_stations: HashMap<i32, (String, String, (f32, f32))>,
    pub platforms_to_stations: HashMap<i32, Vec<(i32, HashSet<i32>, Vec<String>)>>,
    pub edge_map: HashMap<i32, HashSet<i32>>,
    pub lines: Vec<Line>,
}

#[derive(Debug, Clone)]
pub struct LogicConfig {
    pub number_of_people: i32,
    pub number_of_pods: i32,
    pub pod_capacity: i32,
    pub transition_time: i32,
    pub pod_spawn_rate: i32,
    pub on_pause: bool,
}

#[derive(Debug, Clone)]
pub struct VisualConfig {
    pub screen_size: (f32, f32),
    pub latitude_range_bounds: (f32, f32),
    pub longitude_range_bounds: (f32, f32),
    pub latitude_range_current: (f32, f32),
    pub longitude_range_current: (f32, f32),
    pub screen_offset: f32,
    pub radius_station: f32,
    pub radius_pod: f32,
    pub width_line: f32,
    pub desired_fps: u32,
    pub vsync: bool,
    pub last_mouse: (f32, f32),
    pub last_mouse_while_zooming_relative: (f32, f32),
    pub last_mouse_left: (f32, f32),
}

#[derive(Debug, Clone)]
pub struct Config {
    pub network: NetworkConfig,
    pub logic: LogicConfig,
    pub visual: VisualConfig,
}

pub fn parse_config(raw_config: &Yaml) -> Config {
    let mut stations_path = String::default();
    let mut all_lines_path = String::default();
    let mut screen_size = <(f32, f32)>::default();
    let mut latitude_range_bounds = <(f32, f32)>::default();
    let mut longitude_range_bounds = <(f32, f32)>::default();
    let mut screen_offset = f32::default();
    let mut radius_station = f32::default();
    let mut radius_pod = f32::default();
    let mut width_line = f32::default();
    let mut desired_fps = u32::default();
    let mut vsync = bool::default();
    let mut number_of_people = i32::default();
    let mut pod_capacity = i32::default();
    let mut transition_time = i32::default();
    let mut pod_spawn_rate = i32::default();

    // This whole construct essentially parses the raw Yaml typed structure we get into the more
    // usable Config structure from above.
    // It only respects correctly formatted yamls.
    // TODO: introduce a validator that panics if yaml is incorrectly formatted.
    if let Yaml::Hash(hash) = raw_config {
        if let Some(yaml) = hash.get(&Yaml::String(String::from("network"))) {
            if let Yaml::Hash(hash) = yaml {
                if let Some(yaml) = hash.get(&Yaml::String(String::from("STATIONS_PATH"))) {
                    if let Yaml::String(value) = yaml {
                        stations_path = value.to_string();
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("ALL_LINES_PATH"))) {
                    if let Yaml::String(value) = yaml {
                        all_lines_path = value.to_string();
                    }
                }
            }
        }
        if let Some(yaml) = hash.get(&Yaml::String(String::from("visual"))) {
            if let Yaml::Hash(hash) = yaml {
                if let Some(yaml) = hash.get(&Yaml::String(String::from("SCREEN_SIZE"))) {
                    if let Yaml::Hash(hash) = yaml {
                        if let Some(yaml) = hash.get(&Yaml::String(String::from("X"))) {
                            if let Some(float) = yaml.as_f64() {
                                screen_size.0 = float as f32;
                            }
                        }
                        if let Some(yaml) = hash.get(&Yaml::String(String::from("Y"))) {
                            if let Some(float) = yaml.as_f64() {
                                screen_size.1 = float as f32;
                            }
                        }
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("LATITUDE"))) {
                    if let Yaml::Hash(hash) = yaml {
                        if let Some(yaml) = hash.get(&Yaml::String(String::from("MIN"))) {
                            if let Some(float) = yaml.as_f64() {
                                latitude_range_bounds.0 = float as f32;
                            }
                        }
                        if let Some(yaml) = hash.get(&Yaml::String(String::from("MAX"))) {
                            if let Some(float) = yaml.as_f64() {
                                latitude_range_bounds.1 = float as f32;
                            }
                        }
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("LONGITUDE"))) {
                    if let Yaml::Hash(hash) = yaml {
                        if let Some(yaml) = hash.get(&Yaml::String(String::from("MIN"))) {
                            if let Some(float) = yaml.as_f64() {
                                longitude_range_bounds.0 = float as f32;
                            }
                        }
                        if let Some(yaml) = hash.get(&Yaml::String(String::from("MAX"))) {
                            if let Some(float) = yaml.as_f64() {
                                longitude_range_bounds.1 = float as f32;
                            }
                        }
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("SCREEN_OFFSET"))) {
                    if let Some(float) = yaml.as_f64() {
                        screen_offset = float as f32;
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("RADIUS_STATION"))) {
                    if let Some(float) = yaml.as_f64() {
                        radius_station = float as f32;
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("RADIUS_POD"))) {
                    if let Some(float) = yaml.as_f64() {
                        radius_pod = float as f32;
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("WIDTH_LINE"))) {
                    if let Some(float) = yaml.as_f64() {
                        width_line = float as f32;
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("DESIRED_FPS"))) {
                    if let Yaml::Integer(value) = yaml {
                        desired_fps = *value as u32;
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("VSYNC"))) {
                    if let Yaml::Boolean(value) = yaml {
                        vsync = *value;
                    }
                }
            }
        }

        if let Some(yaml) = hash.get(&Yaml::String(String::from("logic"))) {
            if let Yaml::Hash(hash) = yaml {
                if let Some(yaml) = hash.get(&Yaml::String(String::from("NUMBER_OF_PEOPLE"))) {
                    if let Yaml::Integer(value) = yaml {
                        number_of_people = *value as i32;
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("POD_CAPACITY"))) {
                    if let Yaml::Integer(value) = yaml {
                        pod_capacity = *value as i32;
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("TRANSITION_TIME"))) {
                    if let Yaml::Integer(value) = yaml {
                        transition_time = *value as i32;
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("POD_SPAWN_RATE"))) {
                    if let Yaml::Integer(value) = yaml {
                        pod_spawn_rate = *value as i32;
                    }
                }
            }
        }
    }

    let raw_stations = load_yaml(&stations_path);
    let raw_lines = load_yaml(&all_lines_path);

    let (network_config, number_of_pods) = gen_network_config(&raw_stations, &raw_lines);

    let logical_config = LogicConfig {
        number_of_people: number_of_people,
        number_of_pods: number_of_pods,
        pod_capacity: pod_capacity,
        transition_time: transition_time,
        pod_spawn_rate: pod_spawn_rate,
        on_pause: false,
    };

    let visual_config = VisualConfig {
        screen_size: screen_size,
        latitude_range_bounds: latitude_range_bounds,
        longitude_range_bounds: longitude_range_bounds,
        latitude_range_current: latitude_range_bounds,
        longitude_range_current: longitude_range_bounds,
        screen_offset: screen_offset,
        radius_station: radius_station,
        radius_pod: radius_pod,
        width_line: width_line,
        desired_fps: desired_fps,
        vsync: vsync,
        last_mouse: (0., 0.),
        last_mouse_while_zooming_relative: (0., 0.),
        last_mouse_left: (0., 0.),
    };

    Config {
        network: network_config,
        logic: logical_config,
        visual: visual_config,
    }
}

pub fn gen_network_config(raw_stations: &Yaml, raw_lines: &Yaml) -> (NetworkConfig, i32) {
    let mut n_stations: i64 = 0;
    let mut coordinates_map_stations: HashMap<i32, (String, String, (f32, f32))> = HashMap::new();
    let mut platforms_to_stations: HashMap<i32, Vec<(i32, HashSet<i32>, Vec<String>)>> =
        HashMap::new();
    let mut lines: Vec<Line> = vec![];
    let mut edge_map: HashMap<i32, HashSet<i32>> = HashMap::new();

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

                coordinates_map_stations.insert(id, (name, city, (lat, lon)));
            }
        }
    }

    let mut n_stations_line_separated: i32 = 0;

    if let Yaml::Array(lines_array) = raw_lines {
        for line_yaml in lines_array {
            if let Yaml::Hash(line_hash) = line_yaml {
                let mut stations: Vec<i32> = vec![];
                let mut distances: Vec<i32> = vec![];
                let mut circular: bool = false;
                let mut name: String = String::from("placeholder");

                if let Some(name_yaml) = line_hash.get(&Yaml::String(String::from("name"))) {
                    // TODO finish this
                    if let Yaml::String(name_string) = name_yaml {
                        name = name_string.clone();
                    }
                }
                if let Some(stations_yaml) = line_hash.get(&Yaml::String(String::from("stations")))
                {
                    if let Yaml::Array(stations_array) = stations_yaml {
                        n_stations_line_separated += stations_array.len() as i32;
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
                if let Some(circular_yaml) = line_hash.get(&Yaml::String(String::from("circular")))
                {
                    if let Yaml::Boolean(circular_bool) = circular_yaml {
                        circular = *circular_bool;
                    }
                }
                update_edge_map_and_group_platforms(
                    &name,
                    &stations,
                    circular,
                    &mut platforms_to_stations,
                    &mut edge_map,
                );
                let connections = calc_connections(&name, &stations, circular, &distances);
                // println!("{}, {:?}", name, connections);
                let line = Line {
                    name: name,
                    stations: stations,
                    distances: distances,
                    circular: circular,
                    connections: connections,
                };
                // println!("{:?}", line);
                lines.push(line);
            }
        }
    }

    // println!("{:?}", platforms_to_stations);

    // TODO: find more elegant way to do this
    let n_pods = n_stations_line_separated;

    let network_config = NetworkConfig {
        n_stations: n_stations as i32,
        coordinates_map_stations: coordinates_map_stations,
        platforms_to_stations: platforms_to_stations,
        edge_map: edge_map,
        lines: lines,
    };

    (network_config, n_pods)
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

    let connection_kind;
    match name.as_str().chars().nth(0).unwrap() {
        'u' => connection_kind = ConnectionKind::Subway,
        't' => connection_kind = ConnectionKind::Tram,
        _ => panic!("Only Subway and Tram supported so far."),
    }

    for i in 0..station_ids.len() {
        if i == station_ids.len() - 1 && circular {
            let mut travel_time = Default::default();
            if connection_kind == ConnectionKind::Subway {
                travel_time = distances[i] / 22; // 80 kmh ~= 22 m/s
            } else if connection_kind == ConnectionKind::Tram {
                travel_time = distances[i] / 12; // 43 kmh ~= 12 m/s
            }
            connections.push(Connection {
                station_ids: HashSet::from([station_ids[i], station_ids[0]]),
                travel_time: travel_time,
                kind: connection_kind,
                is_blocked: false,
            });
            break;
        } else if i == station_ids.len() - 1 {
            break;
        } else {
            let mut travel_time = Default::default();
            if connection_kind == ConnectionKind::Subway {
                travel_time = distances[i] / 22; // 80 kmh ~= 22 m/s
            } else if connection_kind == ConnectionKind::Tram {
                travel_time = distances[i] / 12; // 43 kmh ~= 12 m/s
            }
            connections.push(Connection {
                station_ids: HashSet::from([station_ids[i], station_ids[i + 1]]),
                travel_time: travel_time,
                kind: connection_kind,
                is_blocked: false,
            });
        }
    }
    connections
}

fn update_edge_map_and_group_platforms(
    name: &String,
    station_ids: &Vec<i32>,
    circular: bool,
    platforms_to_stations: &mut HashMap<i32, Vec<(i32, HashSet<i32>, Vec<String>)>>,
    edge_map: &mut HashMap<i32, HashSet<i32>>,
) {
    for i in 0..station_ids.len() {
        let station_id = station_ids[i];

        if !edge_map.contains_key(&station_id) {
            edge_map.insert(station_id, HashSet::new());
        }

        let mut next_platform_id: i32 = 0;
        if platforms_to_stations.contains_key(&station_id) {
            if let Some(platforms) = platforms_to_stations.get(&station_id) {
                next_platform_id = platforms.len() as i32;
            }
        }
        let mut stations_involved = HashSet::from([]);
        if i > 0 && i < station_ids.len() - 1 {
            stations_involved.insert(station_ids[i - 1]);
            stations_involved.insert(station_ids[i + 1]);
        } else if i == 0 {
            stations_involved.insert(station_ids[i + 1]);
            if circular {
                stations_involved.insert(station_ids[station_ids.len() - 1]);
            }
        } else if i == station_ids.len() - 1 {
            stations_involved.insert(station_ids[i - 1]);
            if circular {
                stations_involved.insert(station_ids[0]);
            }
        }
        if let Some(platforms) = platforms_to_stations.get_mut(&station_id) {
            let mut platform_existed_already = false;
            // TODO: try to simplify this
            for (_, stations, _) in platforms.clone() {
                if stations == stations_involved {
                    platform_existed_already = true;
                }
            }
            if platform_existed_already {
                for (_, stations, names) in platforms {
                    if stations == &stations_involved {
                        names.push(name.clone());
                    }
                }
            } else {
                platforms.push((
                    next_platform_id,
                    stations_involved.clone(),
                    vec![name.clone()],
                ));
            }
        } else {
            platforms_to_stations.insert(
                station_id,
                vec![(
                    next_platform_id,
                    stations_involved.clone(),
                    vec![name.clone()],
                )],
            );
        }

        if i == 0 {
            if let Some(mut_hashset) = edge_map.get_mut(&station_id) {
                mut_hashset.insert(station_ids[i + 1]);
                if circular {
                    mut_hashset.insert(station_ids[station_ids.len() - 1]);
                }
            }
        } else if i == station_ids.len() - 1 {
            if let Some(mut_hashset) = edge_map.get_mut(&station_id) {
                mut_hashset.insert(station_ids[i - 1]);
                if circular {
                    mut_hashset.insert(station_ids[0]);
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
