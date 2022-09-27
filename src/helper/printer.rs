use crate::network::Network;
use crate::person::peoplebox::PeopleBox;
use crate::pod::podsbox::PodsBox;

pub fn print_get_station(network: &Network, id: i32) {
    let maybe_station = network.try_get_station_by_id_unmut(id);
    match maybe_station {
        Some(station) => {
            println!("----------------------");
            println!("Id: {}", station.id);
            println!("Name: {}", station.name);
            println!("City: {}", station.city);
            // println!("Since Last Pod: {}", station.since_last_pod);
            println!("Platforms: {}", station.stringify_platforms());
            println!("Edges To: {:?}", station.edges_to);
            println!(
                "Pods in Station: {:?}",
                station.get_pods_in_station_as_vec()
            );
            println!(
                "No. of People in Station: {}",
                station.people_in_station.len()
            );
            println!("People in Station: {:?}", station.people_in_station);
            println!("Coordinates: {:?}", station.coordinates);
            println!("----------------------");
        }
        None => {
            println!("No station with id {} exists", id)
        }
    }
}

pub fn print_get_person(people_box: &PeopleBox, id: i32) {
    let maybe_person = people_box.try_get_person_by_id_unmut(id);
    match maybe_person {
        Some(person) => {
            println!("----------------------");
            println!("Id: {}", person.id);
            println!("Coordinates: {:?}", person.real_coordinates);
            println!("Path: {:?}", person.path_state.path);
            println!("----------------------");
        }
        None => {
            println!("No person with id {} exists", id)
        }
    }
}

pub fn print_get_pod(pods_box: &PodsBox, id: i32) {
    let maybe_pod = pods_box.try_get_pod_by_id_unmut(id);
    match maybe_pod {
        Some(pod) => {
            println!("----------------------");
            println!("Id: {}", pod.id);
            println!("Capacity: {:?}", pod.capacity);
            println!("State: {:?}", pod.state);
            println!("People in Pod: {:?}", pod.people_in_pod.len());
            println!(
                "Last Station: {:?}",
                pod.line_state.line.stations[pod.line_state.line_ix as usize]
            );
            println!(
                "Next Station: {:?}",
                pod.line_state.line.stations[pod.line_state.next_ix as usize]
            );
            println!("----------------------");
        }
        None => {
            println!("No pod with id {} exists", id)
        }
    }
}
