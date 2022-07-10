use crate::config::{OFFSET, SCREEN_SIZE};
use crate::network::Network;
use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn get_real_coordinates(coordinates: (f32, f32)) -> (f32, f32) {
    let lat_min: f32 = 11.4606;
    let lat_max: f32 = 11.7036;
    let lat_delta: f32 = lat_max - lat_min;

    let lon_min: f32 = 48.0416;
    let lon_max: f32 = 48.2649;
    let lon_delta: f32 = lon_max - lon_min;

    let x_percentage = (coordinates.0 - lat_min) / lat_delta;
    let y_percentage = (coordinates.1 - lon_min) / lon_delta;

    let x = OFFSET
        + (x_percentage * SCREEN_SIZE.0) * ((SCREEN_SIZE.0 - 2.0 * OFFSET) / SCREEN_SIZE.0) as f32;

    let y = OFFSET
        + ((1. - y_percentage) * SCREEN_SIZE.1)
            * ((SCREEN_SIZE.1 - 2.0 * OFFSET) / SCREEN_SIZE.1) as f32;
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

pub fn get_random_station_id(network: &Network) -> u32 {
    let mut rng = rand::thread_rng();
    let station_ids: Vec<&i32> = network.config.network.coordinates_map.keys().collect();
    let end_ix = rng.gen_range(0..station_ids.len());
    *station_ids[end_ix] as u32
}
