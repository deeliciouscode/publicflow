// Pod State Machine:
//      +-------------------+------> InvalidState
//      |                   |                 ^
//      |                   |                 |
// BetweenStations ---> JustArrived ---> InStation <--+
//      ^    ^   |                            |  |    |
//      |    +---+                            |  +----+
//      +-------------------------------------+

// Can add defects and stuff like that as a state
#[derive(Debug, Clone, PartialEq)]
pub enum PodState {
    BetweenStations {
        station_id_from: i32,
        station_id_to: i32,
        time_to_next_station: i32,
        distance_between: i32,
    },
    InQueue {
        station_id: i32,
        traveled_distance: i32,
    },
    JustArrived {
        station_id: i32,
        traveled_distance: i32,
    },
    InStation {
        station_id: i32,
        time_in_station: i32,
    },
    InvalidState {
        reason: String,
    },
}

// State Transitions
impl PodState {
    pub fn to_between_stations(
        &self,
        to_pod_id: i32,
        time_to_next_station: i32,
        distance: i32,
    ) -> PodState {
        match self {
            PodState::InStation {
                station_id,
                time_in_station: _,
            } => PodState::BetweenStations {
                station_id_from: *station_id,
                station_id_to: to_pod_id,
                time_to_next_station: time_to_next_station,
                distance_between: distance,
            },
            _ => PodState::InvalidState {
                reason: String::from("Pod can only appart from InStation state."),
            },
        }
    }

    pub fn to_in_queue(&self) -> PodState {
        match self {
            PodState::BetweenStations {
                station_id_from: _,
                station_id_to,
                time_to_next_station: _,
                distance_between,
            } => PodState::InQueue {
                station_id: *station_id_to,
                traveled_distance: *distance_between,
            },
            _ => PodState::InvalidState {
                reason: String::from("Pod can only go in queue if in BetweenStations state."),
            },
        }
    }

    pub fn to_just_arrived(&self) -> PodState {
        match self {
            PodState::BetweenStations {
                station_id_from: _,
                station_id_to,
                time_to_next_station: _,
                distance_between,
            } => PodState::JustArrived {
                station_id: *station_id_to,
                traveled_distance: *distance_between,
            },
            PodState::InQueue {
                station_id,
                traveled_distance,
            } => PodState::JustArrived {
                station_id: *station_id,
                traveled_distance: *traveled_distance,
            },
            _ => PodState::InvalidState {
                reason: String::from("Pod can only arrive if in BetweenStations state."),
            },
        }
    }

    pub fn to_in_station(&self) -> PodState {
        match self {
            PodState::JustArrived {
                station_id,
                traveled_distance: _,
            } => PodState::InStation {
                station_id: *station_id,
                time_in_station: 0,
            },
            _ => PodState::InvalidState {
                reason: String::from("Pod can only get to InStation if in JustArrived state."),
            },
        }
    }

    pub fn wait_a_sec(&self) -> PodState {
        match self {
            PodState::InStation {
                station_id,
                time_in_station,
            } => PodState::InStation {
                station_id: *station_id,
                time_in_station: time_in_station + 1,
            },
            _ => PodState::InvalidState {
                reason: String::from("Pod can only wait if in InStation state"),
            },
        }
    }

    pub fn drive_a_sec(&self) -> PodState {
        match self {
            PodState::BetweenStations {
                station_id_from,
                station_id_to,
                time_to_next_station,
                distance_between,
            } => PodState::BetweenStations {
                station_id_from: *station_id_from,
                station_id_to: *station_id_to,
                time_to_next_station: time_to_next_station - 1,
                distance_between: *distance_between,
            },
            _ => PodState::InvalidState {
                reason: String::from("Pod can only drive if in BetweenStations state"),
            },
        }
    }

    pub fn get_station_id(&self) -> i32 {
        match self {
            PodState::JustArrived {
                station_id,
                traveled_distance: _,
            } => *station_id,
            PodState::InQueue {
                station_id,
                traveled_distance: _,
            } => *station_id,
            PodState::InStation {
                time_in_station: _,
                station_id,
            } => *station_id,
            _ => panic!("Can only get id of stationif in JustArrived, InQueue or InStation state"),
        }
    }

    pub fn get_station_id_to(&self) -> i32 {
        match self {
            PodState::BetweenStations {
                station_id_from: _,
                station_id_to,
                time_to_next_station: _,
                distance_between: _,
            } => *station_id_to,
            _ => panic!("Can only get id of station that the pod is driving towards if in BetweenStations state")
        }
    }

    pub fn get_distance_travelled(&self) -> i32 {
        match self {
            PodState::InQueue {
                station_id: _,
                traveled_distance,
            } => *traveled_distance,
            PodState::JustArrived {
                station_id: _,
                traveled_distance,
            } => *traveled_distance,
            _ => panic!("Can only get distance travelled if in InQueue or JustArrived State."),
        }
    }
}
