use crate::connection::Connection;
use crate::network::Network;
use crate::person::Person;
use crate::pod::Pod;
use crate::station::Station;
use std::collections::HashSet;

pub struct State {
    pub network: Network,
    pub pods: Vec<Pod>,
    pub people: Vec<Person>,
}

impl State {
    pub fn get_available_pods(&self, id: i32) -> Vec<&Pod> {
        let mut pods: Vec<&Pod> = vec![];
        for pod in &self.pods {
            if pod.get_station_id() == id {
                pods.push(pod)
            }
        }

        return pods;
    }
}

pub fn get_state() -> State {
    let one = Station {
        id: 0,
        since_last_pod: 0,
        edges_to: vec![1],
    };
    let two = Station {
        id: 1,
        since_last_pod: 0,
        edges_to: vec![0, 2],
    };
    let three = Station {
        id: 2,
        since_last_pod: 0,
        edges_to: vec![1],
    };

    let conn01 = Connection {
        station_ids: HashSet::from([0, 1]),
        travel_time: 120,
    };

    let conn12 = Connection {
        station_ids: HashSet::from([1, 2]),
        travel_time: 120,
    };
    let network = Network {
        stations: vec![one, two, three],
        connections: vec![conn01, conn12],
    };

    let pod = Pod {
        id: 0,
        capacity: 10,
        line: vec![0, 1, 2],
        line_ix: 0,
        time_to_next_station: 0,
        direction: 1,
        in_station: true,
        in_station_since: 0,
        in_station_for: 30,
    };

    let person = Person {
        in_station_since: 0,
        pod_id: -1,
        station_id: 0,
        transition_time: 60,
    };

    let state = State {
        network: network,
        people: vec![person],
        pods: vec![pod],
    };

    state
}
