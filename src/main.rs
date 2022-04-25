mod config;
mod connection;
mod line;
mod network;
mod person;
mod pod;
mod state;
mod station;

use crate::config::{load_file, parse_yaml, SPEED_FACTOR};
use crate::state::{gen_state, get_state, State};
// use crate::state::{get_state, State};
use rand::Rng;
use std::{thread, time};
// TODO: first implement something where people are just moving mindlessly, without destination

fn main() {
    let yaml = load_file("./config/network.yaml");
    let config = parse_yaml(&yaml);

    println!("{:?}\n", config);

    let mut state = gen_state(&config);
    // let mut state = get_s    tate();

    println!("start simulation...");
    let mut seconds = 0;
    loop {
        if seconds >= 1000 {
            break;
        }
        thread::sleep(time::Duration::from_millis(1000 / SPEED_FACTOR));
        seconds += 1;
        println!("{}", seconds);
        // println!("{}", state.graph.connections[0].stations == state.graph.connections[1].stations);
        // println!("{}", state.network.get_station_by_id(seconds).since_last_pod);
        step_one_second(&mut state)
    }
}

fn step_one_second(state: &mut State) {
    for station in &mut state.network.stations {
        station.since_last_pod += 1;
    }

    for pod in &mut state.pods_box.pods {
        if pod.in_station {
            println!("Pod is in station {}.", pod.line_state.get_station_id());
            if pod.in_station_since < pod.in_station_for {
                pod.in_station_since += 1;
            } else {
                let current_station_id = pod.line_state.get_station_id();
                pod.leave_station(&mut state.network);

                let maybe_station = state.network.get_station_by_id(current_station_id);
                match maybe_station {
                    Some(station) => station.since_last_pod = 0,
                    None => panic!("The pod is in a station that does not exist, whoopsie."),
                }
            }
        } else {
            println!("Pod is out there.");
            pod.driving_since += 1;
            if pod.driving_since >= pod.time_to_next_station {
                pod.arrive_in_station(&mut state.network);
                println!(
                    "Just arrived in station {}",
                    pod.line_state.get_station_id()
                );
            }
        }
    }

    for person in &mut state.people_box.people {
        if person.station_id >= 0 {
            if person.in_station_since < person.transition_time {
                person.in_station_since += 1;
                continue;
            }

            println!(
                "person {} in station {} is ready to hop a pod.",
                person.id, person.station_id
            );

            let maybe_station = state.network.get_station_by_id(person.station_id);
            let maybe_pod_ids: Option<Vec<i32>>;

            match maybe_station {
                // TODO: probably suboptimal - look for solution without clone
                Some(station) => maybe_pod_ids = station.get_pods_in_station_as_vec(),
                None => panic!("There is no station with id: {}.", person.station_id),
            }

            match maybe_pod_ids {
                Some(pod_ids) => {
                    let mut rng = rand::thread_rng();
                    let pod_id_to_take = pod_ids[rng.gen_range(0..pod_ids.len())];
                    person.take_pod(pod_id_to_take);
                    println!("Getting into pod with id: {} now", pod_id_to_take);
                }
                None => println!("Can't leave the station, no pod here."),
            }
        } else if person.pod_id >= 0 {
            // entirely different when riding the pod
            println!(
                "Person {} is riding in pod {} now.",
                person.id, person.pod_id
            );
            let maybe_pod = state.pods_box.get_pod_by_id(person.pod_id);
            match maybe_pod {
                Some(pod) => {
                    let mut rng = rand::thread_rng();
                    if pod.line_state.get_station_id() != person.last_station_id
                        && rng.gen_bool(0.5)
                    {
                        person.get_off_pod(pod.line_state.get_next_station_id())
                    }
                }
                None => {
                    println!("Somethings not right, person should be in either pod or station")
                }
            }
        } else {
            println!("Somethings not right, person should be in either pod or station");
        }
    }
}
