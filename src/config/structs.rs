use crate::helper::enums::LineName;
use crate::line::line::Line;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub n_stations: i32,
    pub coordinates_map_stations: HashMap<i32, (String, String, (f32, f32))>,
    pub platforms_to_stations: HashMap<i32, Vec<(HashSet<i32>, Vec<LineName>)>>,
    pub edge_map: HashMap<i32, HashSet<i32>>,
    pub lines: Vec<Line>,
}

#[derive(Debug, Clone)]
pub struct LogicConfig {
    pub number_of_people: i32,
    pub number_of_pods: i32,
    pub pod_capacity: i32,
    pub transition_time: i32,
    pub pods_per_hour: i32,
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
