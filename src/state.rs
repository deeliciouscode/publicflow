use crate::config::Config;
use crate::connection::Connection;
use crate::line::{Line, LineState};
use crate::network::Network;
use crate::person::{PeopleBox, Person};
use crate::pod::{Pod, PodsBox};
use crate::station::Station;
use std::collections::HashSet;

#[derive(Clone)]
pub struct State {
    pub network: Network,
    pub pods_box: PodsBox,
    pub people_box: PeopleBox,
}

pub fn get_state() -> State {
    let one = Station {
        id: 0,
        since_last_pod: 0,
        edges_to: HashSet::from([1]),
        pods_in_station: HashSet::from([0]),
    };
    let two = Station {
        id: 1,
        since_last_pod: 0,
        edges_to: HashSet::from([0, 2]),
        pods_in_station: HashSet::new(),
    };
    let three = Station {
        id: 2,
        since_last_pod: 0,
        edges_to: HashSet::from([1]),
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

    let lines = vec![Line {
        stations: vec![0, 1, 2],
        circular: true,
        connections: vec![conn01, conn12],
    }];

    let network = Network {
        stations: vec![one, two, three],
        lines: lines.clone(),
    };

    let line_state = LineState {
        line: lines[0].clone(),
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

    state.clone()
}

pub fn gen_state(config: &Config) -> State {
    let mut stations: Vec<Station> = vec![];
    for station_id in 0..config.network.n_stations {
        stations.push(Station {
            id: station_id,
            since_last_pod: 0,
            edges_to: HashSet::new(),
            pods_in_station: HashSet::from([station_id]),
        })
    }

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

// Config {
//     network: NetworkConfig {
//         n_stations: 13,
//         edge_map: {
//             0: {1, 4},
//             1: {2, 0},
//             2: {3, 1, 6},
//             3: {10, 2},
//             4: {5, 0, 7},
//             5: {4, 6},
//             6: {2, 5, 9}},
//             7: {4, 8},
//             8: {7, 9},
//             9: {11, 10, 8, 6},
//             10: {3, 9},
//             11: {9, 12},
//             12: {11},
//         lines: [
//             Line {
//                 stations: [0, 1, 2, 3],
//                 circular: false,
//                 connections: [
//                     Connection { station_ids: {0, 1}, travel_time: 20 },
//                     Connection { station_ids: {2, 1}, travel_time: 20 },
//                     Connection { station_ids: {2, 3}, travel_time: 20 }] },
//             Line {
//                 stations: [4, 5, 6],
//                 circular: false,
//                 connections: [
//                     Connection { station_ids: {4, 5}, travel_time: 20 },
//                     Connection { station_ids: {5, 6}, travel_time: 20 }] },
//             Line {
//                 stations: [7, 8, 9, 10],
//                 circular: false,
//                 connections: [
//                     Connection { station_ids: {7, 8}, travel_time: 20 },
//                     Connection { station_ids: {9, 8}, travel_time: 20 },
//                     Connection { station_ids: {9, 10}, travel_time: 20 }] },
//             Line {
//                 stations: [2, 6, 9, 11, 12],
//                 circular: false,
//                 connections: [
//                     Connection { station_ids: {6, 2}, travel_time: 20 },
//                     Connection { station_ids: {9, 6}, travel_time: 20 },
//                     Connection { station_ids: {11, 9}, travel_time: 20 },
//                     Connection { station_ids: {11, 12}, travel_time: 20 }] },
//             Line {
//                 stations: [0, 1, 2, 3, 10, 9, 8, 7, 4],
//                 circular: true,
//                 connections: [
//                     Connection { station_ids: {1, 0}, travel_time: 20 },
//                     Connection { station_ids: {1, 2}, travel_time: 20 },
//                     Connection { station_ids: {2, 3}, travel_time: 20 },
//                     Connection { station_ids: {3, 10}, travel_time: 20 },
//                     Connection { station_ids: {10, 9}, travel_time: 20 },
//                     Connection { station_ids: {9, 8}, travel_time: 20 },
//                     Connection { station_ids: {8, 7}, travel_time: 20 },
//                     Connection { station_ids: {7, 4}, travel_time: 20 },
//                     Connection { station_ids: {0, 4}, travel_time: 20 }] }],
//         pods: PodsConfig { n_pods: 13 } },
//     people: PeopleConfig {
//         n_people: 100
//     }
// }
