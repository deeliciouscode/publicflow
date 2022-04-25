#[derive(Debug)]
pub struct Line {
    pub stations: Vec<i32>,
    pub circular: bool,
}

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
}
