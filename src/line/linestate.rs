use crate::connection::Connection;
use crate::helper::enums::Direction;
use crate::line::line::Line;
use std::collections::HashSet;

// block connection 650-641-631-611
// block connections 5-44 5-0

#[derive(Clone, Debug)]
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

    pub fn set_next_station_ix(&mut self) {
        if self.line_ix + self.direction > (self.line.stations.len() - 1) as i32 {
            if !self.line.circular {
                self.direction *= -1;
                self.next_ix = self.line_ix + self.direction;
            } else {
                self.next_ix = 0;
            }
        } else if self.line_ix + self.direction < 0 {
            if !self.line.circular {
                self.direction *= -1;
                self.next_ix = self.line_ix + self.direction;
            } else {
                self.next_ix = (self.line.stations.len() - 1) as i32;
            }
        } else {
            self.next_ix = self.line_ix + self.direction;
        }
    }

    pub fn update_line_ix(&mut self) {
        self.line_ix = self.next_ix;
    }

    pub fn get_direction(&self) -> Direction {
        if self.direction > 0 {
            Direction::Pos
        } else {
            Direction::Neg
        }
    }

    pub fn try_get_connection(&self, fst: i32, snd: i32) -> Option<&Connection> {
        for connection in &self.line.connections {
            if connection.station_ids == HashSet::from([fst, snd]) {
                return Some(connection);
            }
        }
        return None;
    }

    pub fn _try_get_current_connection(&self) -> Option<&Connection> {
        let fst = self.line.stations[self.line_ix as usize];
        let snd = self.line.stations[self.next_ix as usize];
        for connection in &self.line.connections {
            if connection.station_ids == HashSet::from([fst, snd]) {
                return Some(connection);
            }
        }
        return None;
    }
}
