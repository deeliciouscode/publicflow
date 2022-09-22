use crate::helper::enums::LineName;
use crate::line::line::Line;
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub enum ExecutionMode {
    Headless,
    Visual,
}

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub n_stations: i32,
    pub coordinates_map_stations: HashMap<i32, (String, Vec<String>, String, (f32, f32))>,
    pub station_platforms: HashMap<i32, Vec<(HashSet<i32>, HashSet<LineName>)>>,
    pub edge_map: HashMap<i32, HashSet<i32>>,
    pub lines: Vec<Line>,
}

#[derive(Debug, Clone)]
pub struct LogicConfig {
    pub number_of_people: i32,
    pub command_on_start: String,
    pub number_of_pods: i32,
    pub pod_capacity: i32,
    pub transition_time: i32,
    pub pod_in_station_seconds: i32,
    pub line_pods_per_hour: i32,
    pub station_pods_per_hour: i32,
    pub shuffle_people: bool,
    pub on_pause: bool,
    pub speed_multiplier: u32,
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
    pub vsync: bool,
    pub last_mouse: (f32, f32),
    pub last_mouse_while_zooming_relative: (f32, f32),
    pub last_mouse_left: (f32, f32),
}

#[derive(Debug, Clone)]
pub struct Config {
    pub timestamp_run: Option<DateTime<Utc>>,
    pub mode: ExecutionMode,
    pub environment: String,
    pub network: NetworkConfig,
    pub logic: LogicConfig,
    pub visual: VisualConfig,
}

impl Config {
    pub fn add_timestamp_run(&mut self, timstamp: DateTime<Utc>) {
        self.timestamp_run = Some(timstamp)
    }
}
