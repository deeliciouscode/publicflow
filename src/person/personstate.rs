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
        just_got_in: bool,
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

impl Default for PersonState {
    fn default() -> Self {
        PersonState::ReadyToTakePod { station_id: 0 }
    }
}

// State Transitions
impl PersonState {
    pub fn to_riding(&self, pod_id: i32) -> PersonState {
        match self {
            PersonState::ReadyToTakePod { station_id: _ } => PersonState::RidingPod {
                pod_id: pod_id,
                just_got_in: true,
            },
            PersonState::JustArrived {
                pod_id,
                station_id: _,
            } => PersonState::RidingPod {
                pod_id: *pod_id,
                just_got_in: true,
            },
            // _ => panic!("Person can only take a pod from ReadyToTakePod state.")
            _ => PersonState::InvalidState {
                reason: String::from("Person can only take a pod from ReadyToTakePod state."),
            },
        }
    }

    pub fn to_just_arrived(&self, station_id: i32) -> PersonState {
        match self {
            PersonState::RidingPod {
                pod_id,
                just_got_in: _,
            } => PersonState::JustArrived {
                pod_id: *pod_id,
                station_id: station_id,
            },
            _ => PersonState::InvalidState {
                reason: String::from("Person can only arrive if in RidingPod state."),
            },
        }
    }

    pub fn to_transitioning(&self) -> PersonState {
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

    pub fn to_ready(&self) -> PersonState {
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
    pub fn wait_a_sec(&self) -> PersonState {
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

    pub fn remove_just_got_in(&self) -> PersonState {
        match self {
            PersonState::RidingPod {
                pod_id,
                just_got_in,
            } => {
                if *just_got_in {
                    PersonState::RidingPod {
                        pod_id: *pod_id,
                        just_got_in: !just_got_in,
                    }
                } else {
                    panic!("just_got_in has to be true for this function to be applied")
                }
            }
            _ => PersonState::InvalidState {
                reason: String::from("remove_just_got_in can only be applied to RidingPod State"),
            },
        }
    }

    pub fn try_get_station_id(&self) -> Option<i32> {
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
}
