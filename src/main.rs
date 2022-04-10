mod infrastructs;
mod config;

use std::{thread, time};
pub use self::infrastructs::{get_state, State};
pub use self::config::{SPEED_FACTOR};
// TODO: first implement something where people are just moving mindlessly, without destination

fn main() {
    let mut state = get_state();
    let mut seconds = 0;
    loop {
        if seconds >= 1000 {
            break;
        }
        thread::sleep(time::Duration::from_millis(1000/SPEED_FACTOR));
        seconds += 1;
        // println!("{}", state.graph.connections[0].stations == state.graph.connections[1].stations);
        // println!("{}", state.network.get_station_by_id(seconds).since_last_pod);
        step_one_second(&mut state)
    }
}

fn step_one_second(state: &mut State) {
    for station in &mut state.network.stations {
        station.since_last_pod += 1;
    }
    for person in &mut state.people {
        let station = state.network.get_station_by_id(person.station_id);
        println!("{}", station.since_last_pod);
        // person.is_moving;
    }
}
