use crate::line::Line;
use crate::station::Station;
use ggez::Context;

#[derive(Clone, Debug)]
pub struct Network {
    pub stations: Vec<Station>,
    pub lines: Vec<Line>,
}

impl Network {
    pub fn try_get_station_by_id(&mut self, id: i32) -> Option<&mut Station> {
        for station in &mut self.stations {
            if station.id == id {
                return Some(station);
            }
        }
        return None;
    }

    pub fn try_get_station_by_id_unmut(&self, id: i32) -> Option<&Station> {
        for station in &self.stations {
            if station.id == id {
                return Some(station);
            }
        }
        None
    }

    pub fn print_state(&self) {
        for station in &self.stations {
            println!(
                "Station: {} | Pods: {:?}",
                station.id, station.pods_in_station
            )
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        for line in &self.lines {
            let _res = line.draw(ctx, self);
        }

        for station in &self.stations {
            let _res = station.draw(ctx); // TODO: handle result error case
        }
    }
}
