use crate::helper::enums::{Direction, LineName};
use std::collections::HashSet;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Action {
    #[default]
    NoAction,
    GetStation {
        id: i32,
    },
    GetPerson {
        id: i32,
    },
    GetPod {
        id: i32,
    },
    BlockConnection {
        ids: HashSet<i32>,
    },
    UnblockConnection {
        ids: HashSet<i32>,
    },
    MakePlatformOperational {
        station_id: i32,
        line_name: LineName,
        direction: Direction,
    },
    MakePlatformPassable {
        station_id: i32,
        line_name: LineName,
        direction: Direction,
    },
    MakePlatformQueuable {
        station_id: i32,
        line_name: LineName,
        direction: Direction,
    },
    SpawnPod {
        station_id: i32,
        line_name: LineName,
        direction: Direction,
    },
    ShowPerson {
        id: i32,
        follow: bool,
    },
    HidePerson {
        id: i32,
    },
    ShowPod {
        id: i32,
        permanent: bool,
    },
    HidePod {
        id: i32,
    },
    ShowStation {
        id: i32,
        permanent: bool,
    },
    HideStation {
        id: i32,
    },
    RoutePerson {
        id: i32,
        station_id: u32,
        random_station: bool,
    },
    KillSimulation {
        code: i32,
    },
    Sleep {
        duration: Duration,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Actions {
    pub actions: Vec<Action>,
}

impl Actions {
    pub fn new() -> Self {
        Actions {
            actions: Vec::default(),
        }
    }
}
