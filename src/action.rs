use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum GetAction {
    GetStation { id: i32 },
    GetPerson { id: i32 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetAction {
    BlockConnection { ids: HashSet<i32> },
    UnblockConnection { ids: HashSet<i32> },
    ShowPerson { id: i32, follow: bool },
    HidePerson { id: i32 },
    RoutePerson { id: i32, station_id: u32 },
}

// #[derive(Debug, Clone, PartialEq)]
// pub enum VisualAction {
// }

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
