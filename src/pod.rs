use crate::line::LineState;
use crate::network::Network;
use std::collections::HashSet;

#[derive(Clone)]
pub struct PodsBox {
    pub pods: Vec<Pod>,
}

impl PodsBox {
    pub fn get_available_pods(&self, id: i32) -> Vec<&Pod> {
        let mut pods: Vec<&Pod> = vec![];
        for pod in &self.pods {
            if pod.line_state.get_station_id() == id {
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

#[derive(Clone)]
pub struct Pod {
    pub id: i32,
    pub capacity: i32,
    pub line_state: LineState,
    pub time_to_next_station: i32,
    pub driving_since: i32,
    pub in_station: bool,
    pub in_station_since: i32,
    pub in_station_for: i32,
    pub people_in_pod: HashSet<i32>,
}

impl Pod {
    pub fn leave_station(&mut self, net: &mut Network) {
        self.in_station = false;
        self.line_state.set_next_station_id();
        let current = self.line_state.get_station_id();
        let next = self.line_state.get_next_station_id();
        let maybe_connection = self.line_state.get_connection(current, next);
        match maybe_connection {
            Some(connection) => self.time_to_next_station = connection.travel_time,
            None => panic!("There is no connection between: {} and {}", current, next),
        }
        let maybe_station = net.get_station_by_id(current);
        match maybe_station {
            Some(station) => station.deregister_pod(self.id),
            None => panic!("There is no station with id: {}", current),
        }
    }

    pub fn arrive_in_station(&mut self, net: &mut Network) {
        self.in_station = true;
        self.in_station_since = 0;
        self.line_state.update_line_ix();
        self.time_to_next_station = 0;
        self.driving_since = 0;
        let current = self.line_state.get_station_id();
        let maybe_station = net.get_station_by_id(current);
        match maybe_station {
            Some(station) => station.register_pod(self.id),
            None => panic!("There is no station with id: {}", current),
        }
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
