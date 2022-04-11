use crate::connection::Connection;
use crate::station::Station;

use std::collections::HashSet;

pub struct Network {
    pub stations: Vec<Station>,
    pub connections: Vec<Connection>,
}

impl Network {
    pub fn get_station_by_id(&mut self, id: i32) -> Option<&mut Station> {
        for station in &mut self.stations {
            if station.id == id {
                return Some(station);
            }
        }
        return None;
    }

    pub fn get_connection(&self, fst: i32, snd: i32) -> Option<&Connection> {
        for connection in &self.connections {
            if connection.station_ids == HashSet::from([fst, snd]) {
                return Some(connection);
            }
        }
        return None;
    }
}
