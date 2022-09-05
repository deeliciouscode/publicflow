use crate::config::Config;
use crate::helper::helper::get_screen_coordinates;
use crate::network::Network;
use crate::pod::pod::Pod;

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
        coordinates: (f32, f32),
    },
    InQueue {
        station_id: i32,
        coordinates: (f32, f32),
    },
    JustArrived {
        station_id: i32,
        coordinates: (f32, f32),
    },
    InStation {
        station_id: i32,
        time_in_station: i32,
        coordinates: (f32, f32),
    },
    InvalidState {
        reason: String,
    },
}

// State Transitions
impl PodState {
    pub fn to_between_stations(&self, to_pod_id: i32, time_to_next_station: i32) -> PodState {
        match self {
            PodState::InStation {
                station_id,
                time_in_station: _,
                coordinates,
            } => {
                PodState::BetweenStations {
                    station_id_from: *station_id,
                    station_id_to: to_pod_id,
                    time_to_next_station: time_to_next_station,
                    coordinates: *coordinates,
                } // TODO to
            }
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
                coordinates,
            } => PodState::InQueue {
                station_id: *station_id_to,
                coordinates: *coordinates,
            },
            _ => PodState::InvalidState {
                reason: String::from("Pod can only arrive if in BetweenStations state."),
            },
        }
    }

    pub fn to_just_arrived(&self) -> PodState {
        match self {
            PodState::BetweenStations {
                station_id_from: _,
                station_id_to,
                time_to_next_station: _,
                coordinates,
            } => PodState::JustArrived {
                station_id: *station_id_to,
                coordinates: *coordinates,
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
                coordinates,
            } => PodState::InStation {
                station_id: *station_id,
                time_in_station: 0,
                coordinates: *coordinates,
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
                coordinates,
            } => PodState::InStation {
                station_id: *station_id,
                time_in_station: time_in_station + 1,
                coordinates: *coordinates,
            },
            _ => PodState::InvalidState {
                reason: String::from("Pod can only wait if in InStation state"),
            },
        }
    }

    pub fn drive_a_sec(&self, pod: &Pod, network: &Network, config: &Config) -> PodState {
        match self {
            PodState::BetweenStations {
                station_id_from,
                station_id_to,
                time_to_next_station,
                coordinates: _,
            } => {
                let travel_time = pod
                    .line_state
                    .try_get_connection(*station_id_from, *station_id_to)
                    .unwrap()
                    .travel_time;

                let station_from = network
                    .try_get_station_by_id_unmut(*station_id_from)
                    .unwrap();
                let station_to = network.try_get_station_by_id_unmut(*station_id_to).unwrap();

                let coordinates_from = get_screen_coordinates(station_from.coordinates, config);
                let coordinates_to = get_screen_coordinates(station_to.coordinates, config);
                let x = coordinates_from.0
                    + (coordinates_to.0 - coordinates_from.0)
                        * ((travel_time as f32 - *time_to_next_station as f32)
                            / travel_time as f32);

                let y = coordinates_from.1
                    + (coordinates_to.1 - coordinates_from.1)
                        * ((travel_time as f32 - *time_to_next_station as f32)
                            / travel_time as f32);

                let real_x = x;
                let real_y = y;

                PodState::BetweenStations {
                    station_id_from: *station_id_from,
                    station_id_to: *station_id_to,
                    time_to_next_station: time_to_next_station - 1,
                    coordinates: (real_x, real_y),
                }
            }
            _ => PodState::InvalidState {
                reason: String::from("Pod can only drive if in BetweenStations state"),
            },
        }
    }

    pub fn get_station_id(&self) -> i32 {
        match self {
            PodState::JustArrived {
                station_id,
                coordinates: _
            } => *station_id,
            PodState::InStation {
                time_in_station: _,
                station_id,
                coordinates: _
            } => *station_id,
            _ => panic!("Can only get id of station in which pod arrives if in JustArrived or InStation state")
        }
    }

    pub fn get_station_id_to(&self) -> i32 {
        match self {
            PodState::BetweenStations {
                station_id_from: _,
                station_id_to,
                time_to_next_station: _,
                coordinates: _
            } => *station_id_to,
            _ => panic!("Can only get id of station that the pod is driving towards if in BetweenStations state")
        }
    }

    pub fn try_get_coordinates(&self) -> Option<(f32, f32)> {
        match self {
            PodState::JustArrived {
                station_id: _,
                coordinates,
            } => Some(*coordinates),
            PodState::InQueue {
                station_id: _,
                coordinates,
            } => Some(*coordinates),
            PodState::InStation {
                time_in_station: _,
                station_id: _,
                coordinates,
            } => Some(*coordinates),
            PodState::BetweenStations {
                station_id_from: _,
                station_id_to: _,
                time_to_next_station: _,
                coordinates,
            } => Some(*coordinates),
            _ => None,
        }
    }
}
