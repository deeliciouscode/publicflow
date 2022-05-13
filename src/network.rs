use crate::line::Line;
use crate::station::Station;

#[derive(Clone, Debug)]
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

    pub fn print_state(&self) {
        for station in &self.stations {
            println!(
                "Station: {} | Pods: {:?}",
                station.id, station.pods_in_station
            )
        }
    }
}
