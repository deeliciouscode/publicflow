use crate::connection::Connection;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Line {
    pub stations: Vec<i32>,
    pub circular: bool,
    pub connections: Vec<Connection>,
}

#[derive(Clone)]
pub struct LineState {
    pub line: Line,
    pub line_ix: i32,
    pub next_ix: i32,
    pub direction: i32,
}

impl LineState {
    pub fn get_station_id(&self) -> i32 {
        self.line.stations[self.line_ix as usize]
    }

    pub fn get_next_station_id(&self) -> i32 {
        self.line.stations[self.next_ix as usize]
    }

    pub fn set_next_station_id(&mut self) {
        if self.get_station_id() + self.direction > (self.line.stations.len() - 1) as i32 {
            self.direction *= -1;
        } else if self.get_station_id() + self.direction < 0 {
            self.direction *= -1;
        }
        self.next_ix = self.get_station_id() + self.direction;
    }

    pub fn update_line_ix(&mut self) {
        self.line_ix = self.next_ix;
    }

    pub fn get_connection(&self, fst: i32, snd: i32) -> Option<&Connection> {
        for connection in &self.line.connections {
            if connection.station_ids == HashSet::from([fst, snd]) {
                return Some(connection);
            }
        }
        return None;
    }
}
