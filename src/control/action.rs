use crate::helper::enums::{Direction, LineName};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum GetAction {
    GetStation { id: i32 },
    GetPerson { id: i32 },
    GetPod { id: i32 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetAction {
    BlockStation {
        id: i32,
    },
    UnblockStation {
        id: i32,
    },
    BlockPlatform {
        station_id: i32,
        line: String,
    },
    UnblockPlatform {
        station_id: i32,
        line: String,
    },
    BlockConnection {
        ids: HashSet<i32>,
    },
    UnblockConnection {
        ids: HashSet<i32>,
    },
    MakePlatformOperational {
        station_id: i32,
        line: LineName,
        direction: Direction,
    },
    MakePlatformPassable {
        station_id: i32,
        line: LineName,
        direction: Direction,
    },
    MakePlatformQueuable {
        station_id: i32,
        line: LineName,
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
}

#[derive(Debug, Clone, PartialEq)]
pub enum DoAction {
    KillSimulation { code: i32 },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Actions {
    pub get_actions: Vec<GetAction>,
    pub set_actions: Vec<SetAction>,
    pub do_actions: Vec<DoAction>,
}

impl Actions {
    pub fn new() -> Self {
        Actions {
            get_actions: Vec::default(),
            set_actions: Vec::default(),
            do_actions: Vec::default(),
        }
    }
}
