use crate::network::Network;
use std::collections::HashSet;

pub struct PodsBox {
    pub pods: Vec<Pod>,
}

impl PodsBox {
    pub fn get_available_pods(&self, id: i32) -> Vec<&Pod> {
        let mut pods: Vec<&Pod> = vec![];
        for pod in &self.pods {
            if pod.get_station_id() == id {
                pods.push(pod)
            }
        }
        return pods;
    }
    pub fn get_pod_by_id(&self, id: i32) -> Option<&Pod> {
        for pod in &self.pods {
            if pod.id == id {
                return Some(pod);
            }
        }
        return None;
    }
}

// #[derive(Clone)]
pub struct Pod {
    pub id: i32,
    pub capacity: i32,
    pub line: Vec<i32>,
    pub line_ix: i32,
    pub next_ix: i32,
    pub time_to_next_station: i32,
    pub driving_since: i32,
    pub direction: i32,
    pub in_station: bool,
    pub in_station_since: i32,
    pub in_station_for: i32,
    pub people_in_pod: HashSet<i32>,
}

impl Pod {
    pub fn get_station_id(&self) -> i32 {
        self.line[self.line_ix as usize]
    }

    pub fn get_next_station_id(&self) -> i32 {
        self.line[self.next_ix as usize]
    }

    fn set_next_station_id(&mut self) {
        if self.get_station_id() + self.direction > (self.line.len() - 1) as i32 {
            self.direction *= -1;
        } else if self.get_station_id() + self.direction < 0 {
            self.direction *= -1;
        }
        self.next_ix = self.get_station_id() + self.direction;
    }

    pub fn leave_station(&mut self, net: &mut Network) {
        self.in_station = false;
        self.set_next_station_id();
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

    pub fn arrive_in_station(&mut self) {
        self.in_station = true;
        self.in_station_since = 0;
        self.line_ix = self.next_ix;
        self.time_to_next_station = 0;
        self.driving_since = 0;
    }

    pub fn try_register_person(&mut self, person_id: i32) -> bool {
        if self.people_in_pod.len() > self.capacity as usize {
            return false;
        }
        self.people_in_pod.insert(person_id);
        return true;
    }

    pub fn deregister_person(&mut self, person_id: &i32) {
        self.people_in_pod.remove(person_id);
    }
}
