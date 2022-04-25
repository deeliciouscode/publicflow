use crate::line::Line;
use crate::station::Station;

#[derive(Clone)]
pub struct Network {
    pub stations: Vec<Station>,
    pub lines: Vec<Line>,
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
}
