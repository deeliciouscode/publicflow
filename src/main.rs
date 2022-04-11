mod config;
mod connection;
mod network;
mod person;
mod pod;
mod state;
mod station;

use crate::config::SPEED_FACTOR;
use crate::state::{get_state, State};
use std::{thread, time};
// TODO: first implement something where people are just moving mindlessly, without destination

fn main() {
    let mut state = get_state();
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

    for pod in &mut state.pods {
        if pod.in_station {
            if pod.in_station_since < pod.in_station_for {
                pod.in_station_since += 1;
            } else {
                pod.in_station = false;
                let current_station_id = pod.get_station_id();
                pod.leave_station(&state.network);

                let maybe_station = state.network.get_station_by_id(current_station_id);
                match maybe_station {
                    Some(station) => station.since_last_pod = 0,
                    None => panic!("The pod is in a station that does not exist, whoopsie."),
                }
            }
        } else {
        }
    }

    for person in &mut state.people {
        // let station = state.network.get_station_by_id(person.station_id);
        // println!("{}", station.since_last_pod);
        // person.is_moving;

        if person.pod_id != -1 {}
    }
}
