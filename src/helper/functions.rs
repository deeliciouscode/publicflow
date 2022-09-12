use crate::config::structs::Config;
use crate::connection::YieldTriple;
use crate::helper::enums::{Direction, LineName};
use crate::line::line::Line;
use crate::network::Network;
use geoutils::Location;
use petgraph::graph::UnGraph;
use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

pub fn apply_zoom(config: &mut Config, rel_zoom_factor_wheel: f32) {
    let lat_bounds = config.visual.latitude_range_bounds;
    let lon_bounds = config.visual.longitude_range_bounds;

    let (rel_factor_lat, rel_factor_lon) = config.visual.last_mouse_while_zooming_relative;

    let lat_rng_curr = config.visual.latitude_range_current;
    let lon_rng_curr = config.visual.longitude_range_current;

    let lat_bounds_delta = lat_bounds.1 - lat_bounds.0;
    let lon_bounds_delta = lon_bounds.1 - lon_bounds.0;

    // change of 2% relative to the bounded area
    let lat_change = lat_bounds_delta / 50.;
    let lon_change = lon_bounds_delta / 50.;

    let lat_min = lat_rng_curr.0 + lat_change * rel_factor_lat * rel_zoom_factor_wheel;
    let lat_max = lat_rng_curr.1 - lat_change * (1. - rel_factor_lat) * rel_zoom_factor_wheel;

    let lon_min = lon_rng_curr.0 + lon_change * (1. - rel_factor_lon) * rel_zoom_factor_wheel;
    let lon_max = lon_rng_curr.1 - lon_change * rel_factor_lon * rel_zoom_factor_wheel;

    config.visual.latitude_range_current = (lat_min.max(lat_bounds.0), lat_max.min(lat_bounds.1));
    config.visual.longitude_range_current = (lon_min.max(lon_bounds.0), lon_max.min(lon_bounds.1));
}

pub fn get_screen_coordinates(coordinates: (f32, f32), config: &Config) -> (f32, f32) {
    let (lat_min, lat_max) = config.visual.latitude_range_current;
    let (lon_min, lon_max) = config.visual.longitude_range_current;

    let lat_delta: f32 = lat_max - lat_min;
    let lon_delta: f32 = lon_max - lon_min;

    let x_percentage = (coordinates.0 - lat_min) / lat_delta;
    let y_percentage = (coordinates.1 - lon_min) / lon_delta;

    let offset = config.visual.screen_offset;
    let screen_size = config.visual.screen_size;

    let x = offset
        + (x_percentage * screen_size.0) * ((screen_size.0 - 2.0 * offset) / screen_size.0) as f32;

    let y = offset
        + ((1. - y_percentage) * screen_size.1)
            * ((screen_size.1 - 2.0 * offset) / screen_size.1) as f32;
    (x, y)
}

pub fn format_seconds(seconds: u32) -> String {
    let mut remaining = seconds;
    let hours = remaining / 3600;
    remaining = remaining % 3600;
    let minutes = remaining / 60;
    remaining = remaining % 60;
    let seconds = remaining;

    return format!("{}:{}:{}", hours, minutes, seconds);
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_random_station_id(config: &Config) -> u32 {
    let mut rng = rand::thread_rng();
    let station_ids: Vec<&i32> = config.network.coordinates_map_stations.keys().collect();
    let end_ix = rng.gen_range(0..station_ids.len());
    *station_ids[end_ix] as u32
}

pub fn get_air_travel_time(start: u32, end: u32, network: &Network) -> u32 {
    let mut start_coords: (f32, f32) = (0., 0.);
    let mut end_coords: (f32, f32) = (0., 0.);

    if start == end {
        return 0;
    }

    for station in &network.stations {
        if station.id == start as i32 {
            start_coords = station.coordinates
        }
        if station.id == end as i32 {
            end_coords = station.coordinates
        }
    }

    // if start == 5 && end == 0 || start == 0 && end == 5 {
    //     println!("Coords: {:?} - {:?}", start_coords, end_coords);
    // }

    let start = Location::new(start_coords.0, start_coords.1);
    let end = Location::new(end_coords.0, end_coords.1);
    let distance = start.distance_to(&end).unwrap();
    // println!("Distance = {}", distance.meters());
    let travel_time = distance.meters() / 25.;

    return travel_time as u32;
}

// Experiment: Compare 0 heuristic vs air distance

pub fn transform_line_name_to_enum(line_name: &str) -> LineName {
    let mode = &line_name.chars().nth(0).unwrap();
    let id_str = &line_name[1..];
    let maybe_id = FromStr::from_str(id_str);
    match maybe_id {
        Ok(id) => match mode {
            'U' | 'u' => return LineName::U(id),
            'T' | 't' => return LineName::T(id),
            _ => {
                panic!("{} can not be transformed into an enum.", line_name)
            }
        },
        Err(_) => {
            panic!("Couldn't parse \'{}\' into i32", id_str);
        }
    }
}

pub fn parse_str_to_line_and_directions(line_and_direction: &str) -> (LineName, Vec<Direction>) {
    let line_name_str = line_and_direction.replace(&['+', '-'][..], "");
    let line_name = transform_line_name_to_enum(&line_name_str);
    let mut directions = vec![];

    let contains_plus = line_and_direction.contains('+');
    let contains_minus = line_and_direction.contains('-');

    if contains_plus {
        directions.push(Direction::Pos);
    }

    if contains_minus {
        directions.push(Direction::Neg);
    }

    // default should block both directions
    if !contains_plus && !contains_minus {
        directions.push(Direction::Pos);
        directions.push(Direction::Neg);
    }

    return (line_name, directions);
}

pub fn calc_graph(lines: &Vec<Line>) -> UnGraph<u32, u32> {
    let mut edges: Vec<(u32, u32, u32)> = vec![];

    for line in lines {
        for connection in &line.connections {
            if !connection.is_blocked {
                edges.push(connection.yield_triple())
            }
        }
    }
    let graph = UnGraph::from_edges(edges);
    graph
}