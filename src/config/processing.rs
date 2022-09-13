use crate::config::constants::{
    CONFIG_ROOT, GENERAL_CONFIG_NAME, LINES_CONFIG_NAME, STATIONS_CONFIG_NAME,
};
use crate::config::structs::{Config, LogicConfig, NetworkConfig, VisualConfig};
use crate::connection::Connection;
use crate::helper::enums::LineName;
use crate::helper::functions::transform_line_name_to_enum;
use crate::line::line::Line;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::yaml::Yaml;
use yaml_rust::YamlLoader;

pub fn load_yaml(config_root: &str, config_name: &str) -> Yaml {
    let config_path = format!("{}{}", config_root, config_name);
    let mut file = File::open(config_path).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let docs = YamlLoader::load_from_str(&contents).unwrap();
    return docs[0].clone();
}

pub fn parse_or_override_visual_config(raw_config: &Yaml, visual_config: &mut VisualConfig) {
    if let Yaml::Hash(hash) = raw_config {
        if let Some(yaml) = hash.get(&Yaml::String(String::from("visual"))) {
            if let Yaml::Hash(hash) = yaml {
                if let Some(yaml) = hash.get(&Yaml::String(String::from("screen_size"))) {
                    if let Yaml::Hash(hash) = yaml {
                        if let Some(yaml) = hash.get(&Yaml::String(String::from("x"))) {
                            if let Some(float) = yaml.as_f64() {
                                visual_config.screen_size.0 = float as f32;
                            }
                        }
                        if let Some(yaml) = hash.get(&Yaml::String(String::from("y"))) {
                            if let Some(float) = yaml.as_f64() {
                                visual_config.screen_size.1 = float as f32;
                            }
                        }
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("latitude"))) {
                    if let Yaml::Hash(hash) = yaml {
                        if let Some(yaml) = hash.get(&Yaml::String(String::from("min"))) {
                            if let Some(float) = yaml.as_f64() {
                                visual_config.latitude_range_bounds.0 = float as f32;
                                visual_config.latitude_range_current.0 = float as f32;
                            }
                        }
                        if let Some(yaml) = hash.get(&Yaml::String(String::from("max"))) {
                            if let Some(float) = yaml.as_f64() {
                                visual_config.latitude_range_bounds.1 = float as f32;
                                visual_config.latitude_range_current.1 = float as f32;
                            }
                        }
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("longitude"))) {
                    if let Yaml::Hash(hash) = yaml {
                        if let Some(yaml) = hash.get(&Yaml::String(String::from("min"))) {
                            if let Some(float) = yaml.as_f64() {
                                visual_config.longitude_range_bounds.0 = float as f32;
                                visual_config.longitude_range_current.0 = float as f32;
                            }
                        }
                        if let Some(yaml) = hash.get(&Yaml::String(String::from("max"))) {
                            if let Some(float) = yaml.as_f64() {
                                visual_config.longitude_range_bounds.1 = float as f32;
                                visual_config.longitude_range_current.1 = float as f32;
                            }
                        }
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("screen_offset"))) {
                    if let Some(float) = yaml.as_f64() {
                        visual_config.screen_offset = float as f32;
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("radius_station"))) {
                    if let Some(float) = yaml.as_f64() {
                        visual_config.radius_station = float as f32;
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("radius_pod"))) {
                    if let Some(float) = yaml.as_f64() {
                        visual_config.radius_pod = float as f32;
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("width_line"))) {
                    if let Some(float) = yaml.as_f64() {
                        visual_config.width_line = float as f32;
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("desired_fps"))) {
                    if let Yaml::Integer(value) = yaml {
                        visual_config.desired_fps = *value as u32;
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("vsync"))) {
                    if let Yaml::Boolean(value) = yaml {
                        visual_config.vsync = *value;
                    }
                }
            }
        }
    }
}

pub fn parse_or_override_logic_config(raw_config: &Yaml, logic_config: &mut LogicConfig) {
    if let Yaml::Hash(hash) = raw_config {
        if let Some(yaml) = hash.get(&Yaml::String(String::from("logic"))) {
            if let Yaml::Hash(hash) = yaml {
                if let Some(yaml) = hash.get(&Yaml::String(String::from("number_of_people"))) {
                    if let Yaml::Integer(value) = yaml {
                        logic_config.number_of_people = *value as i32;
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("pod_capacity"))) {
                    if let Yaml::Integer(value) = yaml {
                        logic_config.pod_capacity = *value as i32;
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("transition_time"))) {
                    if let Yaml::Integer(value) = yaml {
                        logic_config.transition_time = *value as i32;
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("pod_in_station_seconds")))
                {
                    if let Yaml::Integer(value) = yaml {
                        logic_config.pod_in_station_seconds = *value as i32;
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("pods_per_hour"))) {
                    if let Yaml::Integer(value) = yaml {
                        logic_config.pods_per_hour = *value as i32;
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("shuffle_people"))) {
                    if let Yaml::Boolean(value) = yaml {
                        logic_config.shuffle_people = *value;
                    }
                }
            }
        }
    }
}

// This function essentially parses the raw Yaml typed structure we get into the more
// usable Config structure from above.
// It only respects correctly formatted yamls.
// TODO: introduce a validator that panics if yaml is incorrectly formatted.
pub fn parse_config(raw_config: &Yaml) -> Config {
    let mut town = String::default();
    let mut command_on_start = String::default();
    let mut overide_general = false;

    if let Yaml::Hash(hash) = raw_config {
        if let Some(yaml) = hash.get(&Yaml::String(String::from("general"))) {
            if let Yaml::Hash(hash) = yaml {
                if let Some(yaml) = hash.get(&Yaml::String(String::from("town"))) {
                    if let Yaml::String(value) = yaml {
                        town = value.to_string();
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("command_on_start"))) {
                    if let Yaml::String(value) = yaml {
                        command_on_start = value.to_string();
                    }
                }
                if let Some(yaml) = hash.get(&Yaml::String(String::from("override"))) {
                    if let Yaml::Boolean(value) = yaml {
                        overide_general = *value;
                    }
                }
            }
        }
    }

    let town_specific_config_root_path = format!("{}{}/", CONFIG_ROOT, town);

    let raw_general = load_yaml(&town_specific_config_root_path, GENERAL_CONFIG_NAME);
    let raw_lines = load_yaml(&town_specific_config_root_path, LINES_CONFIG_NAME);
    let raw_stations = load_yaml(&town_specific_config_root_path, STATIONS_CONFIG_NAME);

    let (network_config, number_of_pods) = gen_network_config(&raw_stations, &raw_lines);

    // println!("{:?}", network_config);

    let mut logic_config = LogicConfig {
        command_on_start: command_on_start,
        number_of_people: i32::default(),
        number_of_pods: number_of_pods,
        pod_capacity: i32::default(),
        transition_time: i32::default(),
        pod_in_station_seconds: i32::default(),
        pods_per_hour: i32::default(),
        shuffle_people: false,
        on_pause: false,
    };

    parse_or_override_logic_config(&raw_general, &mut logic_config);
    // println!("{:?}", logic_config);
    if overide_general {
        parse_or_override_logic_config(&raw_config, &mut logic_config);
    }
    // println!("{:?}", logic_config);

    let mut visual_config = VisualConfig {
        screen_size: <(f32, f32)>::default(),
        latitude_range_bounds: <(f32, f32)>::default(),
        longitude_range_bounds: <(f32, f32)>::default(),
        latitude_range_current: <(f32, f32)>::default(),
        longitude_range_current: <(f32, f32)>::default(),
        screen_offset: f32::default(),
        radius_station: f32::default(),
        radius_pod: f32::default(),
        width_line: f32::default(),
        desired_fps: u32::default(),
        vsync: bool::default(),
        last_mouse: (0., 0.),
        last_mouse_while_zooming_relative: (0., 0.),
        last_mouse_left: (0., 0.),
    };

    parse_or_override_visual_config(&raw_general, &mut visual_config);
    // println!("{:?}", visual_config);
    if overide_general {
        parse_or_override_visual_config(&raw_config, &mut visual_config);
    }
    // println!("{:?}", visual_config);

    Config {
        network: network_config,
        logic: logic_config,
        visual: visual_config,
    }
}

pub fn gen_network_config(raw_stations: &Yaml, raw_lines: &Yaml) -> (NetworkConfig, i32) {
    let mut n_stations: i64 = 0;
    let mut coordinates_map_stations: HashMap<i32, (String, Vec<String>, String, (f32, f32))> =
        HashMap::new();
    let mut station_platforms: HashMap<i32, Vec<(HashSet<i32>, HashSet<LineName>)>> =
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
                let mut entrypoint_for: Vec<String> = vec![];

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

                if let Some(entrypoint_for_yaml) =
                    station_hash.get(&Yaml::String(String::from("entrypoint_for")))
                {
                    // TODO finish this
                    if let Yaml::Array(entrypoint_for_array) = entrypoint_for_yaml {
                        for line_name in entrypoint_for_array {
                            if let Yaml::String(line_name_string) = line_name {
                                entrypoint_for.push(line_name_string.clone());
                            }
                        }
                    }
                }

                coordinates_map_stations.insert(id, (name, entrypoint_for, city, (lat, lon)));
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
                let mut line_name: LineName = LineName::Placeholder;

                if let Some(name_yaml) = line_hash.get(&Yaml::String(String::from("name"))) {
                    // TODO finish this
                    if let Yaml::String(name_string) = name_yaml {
                        line_name = transform_line_name_to_enum(&name_string);
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
                    &line_name,
                    &stations,
                    circular,
                    &mut station_platforms,
                    &mut edge_map,
                );
                let connections = calc_connections(&line_name, &stations, circular, &distances);
                // println!("{}, {:?}", name, connections);
                let line = Line {
                    name: line_name,
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

    // println!("{:?}", station_platforms);

    // TODO: find more elegant way to do this
    let n_pods = n_stations_line_separated;

    let network_config = NetworkConfig {
        n_stations: n_stations as i32,
        coordinates_map_stations: coordinates_map_stations,
        station_platforms: station_platforms,
        edge_map: edge_map,
        lines: lines,
    };

    (network_config, n_pods)
}

fn update_edge_map_and_group_platforms(
    line_name: &LineName,
    station_ids: &Vec<i32>,
    circular: bool,
    station_platforms: &mut HashMap<i32, Vec<(HashSet<i32>, HashSet<LineName>)>>,
    edge_map: &mut HashMap<i32, HashSet<i32>>,
) {
    for i in 0..station_ids.len() {
        let station_id = station_ids[i];

        if !edge_map.contains_key(&station_id) {
            edge_map.insert(station_id, HashSet::new());
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

        if let Some(platforms) = station_platforms.get_mut(&station_id) {
            let mut platform_existed_already = false;
            for (stations_existing, line_names_existing) in platforms.clone() {
                for station in &stations_involved {
                    let maybe_first_line_name_existing = line_names_existing.iter().next();
                    match maybe_first_line_name_existing {
                        Some(first_line_name_existing) => {
                            if stations_existing.contains(station)
                                && first_line_name_existing.in_same_line_class(line_name)
                            {
                                platform_existed_already = true;
                            }
                        }
                        None => {}
                    }
                }
            }
            if platform_existed_already {
                for (stations_existing, line_names_existing) in platforms {
                    for station in &stations_involved {
                        let maybe_first_line_name_existing = line_names_existing.iter().next();
                        match maybe_first_line_name_existing {
                            Some(first_line_name_existing) => {
                                if stations_existing.contains(station)
                                    && first_line_name_existing.in_same_line_class(line_name)
                                {
                                    stations_existing.extend(stations_involved.clone());
                                    line_names_existing.extend(HashSet::from([line_name.clone()]));
                                }
                            }
                            None => {}
                        }
                    }
                }
            } else {
                platforms.push((
                    stations_involved.clone(),
                    HashSet::from([line_name.clone()]),
                ));
            }
        } else {
            station_platforms.insert(
                station_id,
                vec![(
                    stations_involved.clone(),
                    HashSet::from([line_name.clone()]),
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

fn calc_connections(
    line_name: &LineName,
    station_ids: &Vec<i32>,
    circular: bool,
    distances: &Vec<i32>,
) -> Vec<Connection> {
    let mut connections: Vec<Connection> = vec![];

    // first some verifications
    // println!("circular: {}, stat_len: {}, dist_len: {}", circular, station_ids.len(), distances.len());
    if circular && station_ids.len() != distances.len() {
        panic!("A circular line must have as many distances as it has stations. Ascertain that this is the case for line {:?}", line_name);
    } else if !circular && station_ids.len() != distances.len() + 1 {
        panic!("A non circular line must have exactly n-1 distances if it has n stations. Ascertain that this is the case for line {:?}", line_name);
    }

    // let connection_kind;
    // match name.as_str().chars().nth(0).unwrap() {
    //     'u' => connection_kind = ConnectionKind::Subway,
    //     't' => connection_kind = ConnectionKind::Tram,
    //     _ => panic!("Only Subway and Tram supported so far."),
    // }

    fn get_travel_time(line_name: &LineName, distances: &Vec<i32>, i: usize) -> i32 {
        let travel_time;
        match line_name {
            LineName::S(_) => travel_time = distances[i] / 24, // 87 kmh ~= 24 m/s
            LineName::U(_) => travel_time = distances[i] / 20, // 72 kmh ~= 20 m/s
            LineName::T(_) => travel_time = distances[i] / 12, // 43 kmh ~= 12 m/s
            _ => {
                panic!("Placeholder is not covered here. LineName should never be placeholder at exectution of the Simulation.")
            }
        }
        return travel_time;
    }

    for i in 0..station_ids.len() {
        if i == station_ids.len() - 1 && circular {
            let travel_time = get_travel_time(line_name, distances, i);
            connections.push(Connection {
                station_ids: HashSet::from([station_ids[i], station_ids[0]]),
                travel_time: travel_time,
                line_name: line_name.clone(),
                is_blocked: false,
            });
            break;
        } else if i == station_ids.len() - 1 {
            break;
        } else {
            let travel_time = get_travel_time(line_name, distances, i);
            connections.push(Connection {
                station_ids: HashSet::from([station_ids[i], station_ids[i + 1]]),
                travel_time: travel_time,
                line_name: line_name.clone(),
                is_blocked: false,
            });
        }
    }
    connections
}
