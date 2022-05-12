use crate::network::Network;
use crate::pod::PodsBox;
use rand::Rng;

// TODO: implement destinations
#[derive(Clone, Debug)]
pub struct PeopleBox {
    pub people: Vec<Person>,
}

impl PeopleBox {
    pub fn print_state(&self) {
        for person in &self.people {
            let maybe_station_id = person.try_get_station_id();
            let station_id;
            match maybe_station_id {
                Some(_station_id) => station_id = _station_id.to_string(),
                None => station_id = String::from("None"),
            }
            let maybe_pod_id = person.try_get_pod_id();
            let pod_id;
            match maybe_pod_id {
                Some(_pod_id) => pod_id = _pod_id.to_string(),
                None => pod_id = String::from("None"),
            }

            println!(
                "Person: {} | Station: {} | Pod: {} | State: {:?}",
                person.id, station_id, pod_id, person.state
            )
        }
    }
}

#[derive(Clone, Debug)]
pub struct Person {
    pub id: i32,
    transition_time: i32,
    state: PersonState,
}

impl Person {
    pub fn new(id: i32, transition_time: i32) -> Self {
        Person {
            id: id,
            transition_time: transition_time,
            state: PersonState::Transitioning {
                station_id: 0,
                previous_pod_id: -1,
                time_in_station: transition_time - 1,
            },
        }
    }

    // TODO: move logic of people from main to this function
    pub fn update_state(&mut self, pods_box: &mut PodsBox, network: &mut Network) {
        match &self.state {
            PersonState::ReadyToTakePod { station_id } => {
                println!("person in ready state");
                // Assign first instead of ..try_to_take_a_pod(.., .., *station_id) because:
                // https://github.com/rust-lang/rust/issues/59159
                let station_id_deref = *station_id;
                self.try_to_take_a_pod(pods_box, network, station_id_deref);
            }
            PersonState::RidingPod { pod_id } => {
                println!("person in riding state");
                let pod_id_deref = *pod_id;
                self.ride_pod(pods_box, pod_id_deref);
            }
            PersonState::JustArrived {
                pod_id,
                station_id: _,
            } => {
                println!("person in arrived state");
                let pod_id_deref = *pod_id;
                self.make_on_arrival_descission(pods_box, pod_id_deref);
            }
            PersonState::Transitioning {
                station_id: _,
                previous_pod_id: _,
                time_in_station,
            } => {
                if *time_in_station < self.transition_time {
                    println!("person in transitioning state and not ready.");
                    self.state = self.state.wait_a_sec();
                } else {
                    println!("person in transitioning state and going to ready state.");
                    self.state = self.state.to_ready();
                }
            }
            PersonState::InvalidState { reason } => {
                panic!("Person {} is in invalid state. Reason: {}", self.id, reason);
            }
        }
    }

    fn try_to_take_a_pod(
        &mut self,
        pods_box: &mut PodsBox,
        network: &mut Network,
        station_id: i32,
    ) {
        let mut rng = rand::thread_rng();
        let station = network.get_station_by_id(station_id).unwrap();
        let maybe_pod_ids: Option<Vec<i32>> = station.get_pod_ids_in_station_as_vec();
        match maybe_pod_ids {
            Some(pod_ids) => {
                let range = rng.gen_range(0..pod_ids.len());
                // println!("the random range: {:?}", range);
                let pod_id_to_take = pod_ids[range];
                let maybe_pod = pods_box.get_pod_by_id(pod_id_to_take);
                match maybe_pod {
                    Some(pod) => {
                        let got_in = pod.try_register_person(self.id);
                        if got_in {
                            println!("Getting into pod with id: {} now", pod_id_to_take);
                            self.state = self.state.to_riding(pod_id_to_take);
                        } else {
                            println!(
                                "Couldn't get into pod with id: {} - it's full.",
                                pod_id_to_take
                            );
                        }
                    }
                    None => println!("Pod with id: {}, does not exist.", pod_id_to_take),
                }
            }
            None => println!("Can't leave the station, no pod here."),
        }
    }

    fn ride_pod(&mut self, pods_box: &mut PodsBox, pod_id: i32) {
        let maybe_pod = pods_box.get_pod_by_id(pod_id);
        match maybe_pod {
            Some(pod) => {
                if pod.is_in_just_arrived_state() {
                    self.state.to_just_arrived(pod.get_station_id());
                }
            }
            None => panic!("Pod with id: {} does not exist.", pod_id),
        }
    }

    fn make_on_arrival_descission(&mut self, pods_box: &mut PodsBox, pod_id: i32) {
        let mut rng = rand::thread_rng();
        let get_out = rng.gen_bool(0.5);
        if get_out {
            self.state = self.state.to_transitioning();
            let maybe_pod = pods_box.get_pod_by_id(pod_id);
            match maybe_pod {
                Some(pod) => {
                    pod.deregister_person(&self.id);
                }
                None => panic!("Pod with id: {} does not exist.", pod_id),
            }
        } else {
            self.state = self.state.to_riding(pod_id); // pod_id is ignored in this case
        }

        // person.get_off_pod(pod.line_state.get_next_station_id());

        self.state = self.state.to_transitioning();
    }

    pub fn try_get_station_id(&self) -> Option<i32> {
        self.state.try_get_station_id()
    }

    pub fn try_get_pod_id(&self) -> Option<i32> {
        self.state.try_get_pod_id()
    }
}

// Person State Machine:
//      +-------------------+------> InvalidState <---------+
//      |                   |               ^               |
//      |                   |               |               |
// ReadyToTakePod ---> RidingPod ---> JustArrived ---> Transitioning ---+
//      ^                    ^                |             |    ^      |
//      |                    +----------------+             |    |      |
//      +---------------------------------------------------+    +------+

#[derive(Debug, Clone, PartialEq)]
pub enum PersonState {
    ReadyToTakePod {
        station_id: i32,
    },
    RidingPod {
        pod_id: i32,
    },
    JustArrived {
        pod_id: i32,
        station_id: i32,
    },
    Transitioning {
        station_id: i32,
        previous_pod_id: i32,
        time_in_station: i32,
    },
    InvalidState {
        reason: String,
    },
}

// State Transitions
impl PersonState {
    fn to_riding(&self, pod_id: i32) -> PersonState {
        match self {
            PersonState::ReadyToTakePod { station_id: _ } => {
                PersonState::RidingPod { pod_id: pod_id }
            }
            PersonState::JustArrived {
                pod_id,
                station_id: _,
            } => PersonState::RidingPod { pod_id: *pod_id },
            _ => PersonState::InvalidState {
                reason: String::from("Person can only take a pod from ReadyToTakePod state."),
            },
        }
    }

    fn to_just_arrived(&self, station_id: i32) -> PersonState {
        match self {
            PersonState::RidingPod { pod_id } => PersonState::JustArrived {
                pod_id: *pod_id,
                station_id: station_id,
            },
            _ => PersonState::InvalidState {
                reason: String::from("Person can only arrive if in RidingPod state."),
            },
        }
    }

    fn to_transitioning(&self) -> PersonState {
        match self {
            PersonState::JustArrived { pod_id, station_id } => PersonState::Transitioning {
                previous_pod_id: *pod_id,
                station_id: *station_id,
                time_in_station: 0,
            },
            _ => PersonState::InvalidState {
                reason: String::from("Person can only transition if in JustArrived state."),
            },
        }
    }

    fn to_ready(&self) -> PersonState {
        match self {
            PersonState::Transitioning {
                previous_pod_id: _,
                station_id,
                time_in_station: _,
            } => PersonState::ReadyToTakePod {
                station_id: *station_id,
            },
            _ => PersonState::InvalidState {
                reason: String::from(
                    "Person can only get ready to take a pod if in Transitioning state.",
                ),
            },
        }
    }

    fn wait_a_sec(&self) -> PersonState {
        match self {
            PersonState::Transitioning {
                previous_pod_id,
                station_id,
                time_in_station,
            } => PersonState::Transitioning {
                previous_pod_id: *previous_pod_id,
                station_id: *station_id,
                time_in_station: time_in_station + 1,
            },
            _ => PersonState::InvalidState {
                reason: String::from("Person can only wait if in Transitioning state"),
            },
        }
    }

    fn try_get_station_id(&self) -> Option<i32> {
        match self {
            PersonState::ReadyToTakePod { station_id } => Some(*station_id),
            PersonState::JustArrived {
                pod_id: _,
                station_id,
            } => Some(*station_id),
            PersonState::Transitioning {
                station_id,
                previous_pod_id: _,
                time_in_station: _,
            } => Some(*station_id),
            _ => None,
        }
    }

    fn try_get_pod_id(&self) -> Option<i32> {
        match self {
            PersonState::RidingPod { pod_id } => Some(*pod_id),
            PersonState::JustArrived {
                pod_id,
                station_id: _,
            } => Some(*pod_id),
            _ => None,
        }
    }
}
