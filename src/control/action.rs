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
    MakePlatformQueueable {
        station_id: i32,
        line_name: LineName,
        direction: Direction,
    },
    SpawnPod {
        station_id: i32,
        line_name: LineName,
        direction: Direction,
        force: bool,
    },
    ShowPerson {
        id: i32,
    },
    HidePerson {
        id: i32,
    },
    ShowPod {
        id: i32,
    },
    HidePod {
        id: i32,
    },
    ShowStation {
        id: i32,
    },
    HideStation {
        id: i32,
    },
    RoutePerson {
        id: i32,
        station_id: u32,
        stay_there: bool,
        random_station: bool,
    },
    KillSimulation {
        code: i32,
    },
    GatherMetrics,
    DumpMetricsPeople {
        all: bool,
        avg: bool,
    },
    DumpMetricsPerson {
        person_id: i32,
    },
    DumpMetricsPods {
        all: bool,
        avg: bool,
    },
    DumpMetricsPod {
        pod_id: i32,
    },
    DumpConfig,
    Sleep {
        duration: Duration,
    },
    Loop {
        n: u32,
    },
    Endloop,
    StartConcurency,
    DoConcurrently,
    EndConcurency,
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
