use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Connection {
    pub station_ids: HashSet<i32>,
    pub travel_time: i32,
}
