use crate::config::Config;
use crate::connection::Connection;
use crate::line::{Line, LineState};
use crate::network::Network;
use crate::person::{PeopleBox, Person};
use crate::pod::{Pod, PodsBox};
use crate::station::Station;
use std::collections::HashSet;

pub struct State {
    pub network: Network,
    pub pods_box: PodsBox,
    pub people_box: PeopleBox,
}

pub fn get_state() -> State {
    let one = Station {
        id: 0,
        since_last_pod: 0,
        edges_to: vec![1],
        pods_in_station: HashSet::from([0]),
    };
    let two = Station {
        id: 1,
        since_last_pod: 0,
        edges_to: vec![0, 2],
        pods_in_station: HashSet::new(),
    };
    let three = Station {
        id: 2,
        since_last_pod: 0,
        edges_to: vec![1],
        pods_in_station: HashSet::new(),
    };

    let conn01 = Connection {
        station_ids: HashSet::from([0, 1]),
        travel_time: 20,
    };

    let conn12 = Connection {
        station_ids: HashSet::from([1, 2]),
        travel_time: 20,
    };
    let network = Network {
        stations: vec![one, two, three],
        connections: vec![conn01, conn12],
    };

    let line_state = LineState {
        line: Line {
            stations: vec![0, 1, 2],
            circular: true,
        },
        line_ix: 0,
        next_ix: 1,
        direction: 1,
    };

    let pod = Pod {
        id: 0,
        capacity: 10,
        line_state: line_state,
        time_to_next_station: 0,
        driving_since: 0,
        in_station: true,
        in_station_since: 0,
        in_station_for: 10,
        people_in_pod: HashSet::new(), // TODO: init with capacity
    };

    let person = Person {
        id: 0,
        in_station_since: 15, // to be able to take the first train
        pod_id: -1,
        station_id: 0,
        last_station_id: 0,
        transition_time: 20,
    };

    let people_box = PeopleBox {
        people: vec![person],
    };

    let pods_box = PodsBox { pods: vec![pod] };

    let state = State {
        network: network,
        people_box: people_box,
        pods_box: pods_box,
    };

    state
}

pub fn gen_state(config: &Config) -> State {
    get_state()
}

// Config {
//     network: NetworkConfig {
//         n_stations: 13,
//         lines: [
//             Line { stations: [0, 1, 2, 4], circular: false },
//             Line { stations: [4, 5, 6], circular: false },
//             Line { stations: [7, 8, 9, 10], circular: false },
//             Line { stations: [2, 6, 9, 11, 12], circular: false },
//             Line { stations: [0, 1, 2, 3, 10, 9, 8, 7, 4], circular: true }],
//         pods: PodsConfig {
//             n_pods: 13
//         }
//     },
//     people: PeopleConfig {
//         n_people: 100
//     }
// }
