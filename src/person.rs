use crate::network::Network;
use crate::pod::PodsBox;

// TODO: implement destinations
#[derive(Clone, Debug)]
pub struct PeopleBox {
    pub people: Vec<Person>,
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
                self.state = self.state.to_riding(0);
                println!("person in ready state")
            }
            PersonState::RidingPod { pod_id } => {
                self.state = self.state.to_just_arrived(0);
                println!("person in riding state")
            }
            PersonState::JustArrived { pod_id, station_id } => {
                self.state = self.state.to_transitioning();
                println!("person in arrived state")
            }
            PersonState::Transitioning {
                station_id,
                previous_pod_id,
                time_in_station,
            } => {
                if time_in_station < &self.transition_time {
                    self.state = self.state.wait_a_sec();
                    println!("person in transitioning state and not ready.")
                } else {
                    self.state = self.state.to_ready();
                    println!("person in transitioning state and going to ready state.")
                }
            }
            PersonState::InvalidState { reason } => {
                panic!("Person {} is in invalid state. Reason: {}", self.id, reason)
            }
        }
    }

    // pub fn take_pod(&mut self, pod_id: i32) {
    //     self.pod_id = pod_id;
    //     self.last_station_id = self.station_id;
    //     self.station_id = -1;
    // }

    // pub fn get_off_pod(&mut self, station_id: i32) {
    //     self.station_id = station_id;
    //     self.in_station_since = 0;
    //     self.pod_id = -1;
    // }
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
}

// #[derive(Clone, Debug)]
// pub struct PersonOld {
//     pub id: i32,
//     pub in_station_since: i32,
//     pub pod_id: i32,
//     pub station_id: i32,
//     pub last_station_id: i32,
//     pub transition_time: i32,
// }
