use crate::network::Network;

pub struct Pod {
    pub id: i32,
    pub capacity: i32,
    pub line: Vec<i32>,
    pub line_ix: i32,
    pub time_to_next_station: i32,
    pub direction: i32,
    pub in_station: bool,
    pub in_station_since: i32,
    pub in_station_for: i32,
}

impl Pod {
    pub fn get_station_id(&self) -> i32 {
        self.line[self.line_ix as usize]
    }

    pub fn get_next_station_id(&mut self) -> i32 {
        if self.get_station_id() + self.direction > (self.line.len() - 1) as i32 {
            self.direction *= -1;
        } else if self.get_station_id() + self.direction < 0 {
            self.direction *= -1;
        }
        let new_ix = self.get_station_id() + self.direction;
        self.line[new_ix as usize]
    }

    pub fn leave_station(&mut self, net: &mut Network) {
        let current = self.get_station_id();
        let next = self.get_next_station_id();
        let maybe_connection = net.get_connection(current, next);
        match maybe_connection {
            Some(connection) => self.time_to_next_station = connection.travel_time,
            None => panic!("There is no connection between: {} and {}", current, next),
        }
        let maybe_station = net.get_station_by_id(current);
        match maybe_station {
            Some(station) => station.deregister_pod(current),
            None => panic!("There is no station with id: {}", current),
        }
    }
}
