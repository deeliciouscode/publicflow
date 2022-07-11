use crate::config::Config;
use crate::network::Network;
use geoutils::Location;
use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn get_screen_coordinates(coordinates: (f32, f32), config: &Config) -> (f32, f32) {
    let lat_min: f32 = 11.4606;
    let lat_max: f32 = 11.7036;
    let lat_delta: f32 = lat_max - lat_min;

    let lon_min: f32 = 48.0416;
    let lon_max: f32 = 48.2649;
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

pub fn get_random_station_id(network: &Network, config: &Config) -> u32 {
    let mut rng = rand::thread_rng();
    let station_ids: Vec<&i32> = config.network.coordinates_map.keys().collect();
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
