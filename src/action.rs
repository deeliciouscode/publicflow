use crate::enums::LineName;
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
    },
    MakePlatformPassable {
        station_id: i32,
        line: LineName,
    },
    MakePlatformQueuable {
        station_id: i32,
        line: LineName,
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

// #[derive(Debug, Clone, PartialEq)]
// pub enum VisualAction {
// }

#[derive(Debug, Clone, PartialEq)]
pub struct Actions {
    pub get_actions: Vec<GetAction>,
    pub set_actions: Vec<SetAction>,
    // pub visual_actions: Vec<VisualAction>,
}

impl Actions {
    pub fn new() -> Self {
        Actions {
            get_actions: Vec::default(),
            set_actions: Vec::default(),
            // visual_actions: Vec::default(),
        }
    }
}
