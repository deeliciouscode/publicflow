use crate::connection::Connection;
use crate::network::Network;
use crate::person::{PeopleBox, Person};
use crate::pod::{Pod, PodsBox};
use crate::station::Station;
use linked_hash_map::LinkedHashMap;
use std::collections::HashSet;
use yaml_rust::yaml::{Hash, Yaml};

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

    let pod = Pod {
        id: 0,
        capacity: 10,
        line: vec![0, 1, 2],
        line_ix: 0,
        next_ix: 1,
        time_to_next_station: 0,
        driving_since: 0,
        direction: 1,
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

// Hash({
//     String("network"): Hash({
//         String("n_stations"): Integer(13),
//         String("lines"): Array([
//             Hash({
//                 String("stations"): Array([Integer(0), Integer(1), Integer(2), Integer(4)]),
//                 String("circular"): Boolean(false)}),
//             Hash({
//                 String("stations"): Array([Integer(4), Integer(5), Integer(6)]),
//                 String("circular"): Boolean(false)}),
//             Hash({
//                 String("stations"): Array([Integer(7), Integer(8), Integer(9), Integer(10)]),
//                 String("circular"): Boolean(false)}),
//             Hash({
//                 String("stations"): Array([Integer(2), Integer(6), Integer(9), Integer(11), Integer(12)]),
//                 String("circular"): Boolean(false)}),
//             Hash({
//                 String("stations"): Array([Integer(0), Integer(1), Integer(2), Integer(3), Integer(10), Integer(9), Integer(8), Integer(7), Integer(4)]),
//                 String("circular"): Boolean(true)})]),
//         String("pods"):
//             Hash({String("n_pods"): Integer(13)})}),
//     String("people"): Hash({String("n_people"): Integer(100)})})

pub fn gen_state(config_yaml: &Yaml) -> State {
    let mut n_stations: i64 = 0;
    let mut n_pods: i64 = 0;
    let mut n_people: i64 = 0;

    if let Yaml::Hash(config_hash) = config_yaml {
        if let Some(network_yaml) = config_hash.get(&Yaml::String(String::from("network"))) {
            if let Yaml::Hash(network_hash) = network_yaml {
                if let Some(n_stations_yaml) =
                    network_hash.get(&Yaml::String(String::from("n_stations")))
                {
                    if let Yaml::Integer(value) = n_stations_yaml {
                        n_stations = *value;
                    }
                }
                if let Some(pods_yaml) = network_hash.get(&Yaml::String(String::from("pods"))) {
                    if let Yaml::Hash(pods_hash) = pods_yaml {
                        if let Some(n_pods_yaml) =
                            pods_hash.get(&Yaml::String(String::from("n_pods")))
                        {
                            if let Yaml::Integer(value) = n_pods_yaml {
                                n_pods = *value;
                            }
                        }
                    }
                }
            }
        }
        if let Some(people_yaml) = config_hash.get(&Yaml::String(String::from("people"))) {
            if let Yaml::Hash(people_hash) = people_yaml {
                if let Some(n_people_yaml) =
                    people_hash.get(&Yaml::String(String::from("n_people")))
                {
                    if let Yaml::Integer(value) = n_people_yaml {
                        n_people = *value;
                    }
                }
            }
        }
    }

    println!(
        "n_stations: {}, n_pods: {}, n_people: {}",
        n_stations, n_pods, n_people
    );

    get_state()
}

//     let stations = vec![];
//     for i in 0..13 {
//         let station = Station {
//             id: i,
//             since_last_pod: 0,
//             edges_to: vec![1],
//             pods_in_station: HashSet::from([i]),
//         };

//     }

//     let conn01 = Connection {
//         station_ids: HashSet::from([0, 1]),
//         travel_time: 20,
//     };

//     let conn12 = Connection {
//         station_ids: HashSet::from([1, 2]),
//         travel_time: 20,
//     };
//     let network = Network {
//         stations: vec![one, two, three],
//         connections: vec![conn01, conn12],
//     };

//     let pod = Pod {
//         id: 0,
//         capacity: 10,
//         line: vec![0, 1, 2],
//         line_ix: 0,
//         next_ix: 1,
//         time_to_next_station: 0,
//         driving_since: 0,
//         direction: 1,
//         in_station: true,
//         in_station_since: 0,
//         in_station_for: 10,
//         people_in_pod: HashSet::new(), // TODO: init with capacity
//     };

//     let person = Person {
//         id: 0,
//         in_station_since: 15, // to be able to take the first train
//         pod_id: -1,
//         station_id: 0,
//         last_station_id: 0,
//         transition_time: 20,
//     };

//     let people_box = PeopleBox {
//         people: vec![person],
//     };

//     let pods_box = PodsBox { pods: vec![pod] };

//     let state = State {
//         network: network,
//         people_box: people_box,
//         pods_box: pods_box,
//     };

//     state
// }
