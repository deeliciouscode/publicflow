use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum GetAction {
    GetStation { station_id: i32 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetAction {
    BlockConnection { ids: HashSet<i32> },
}

pub struct Actions {
    pub get_actions: Vec<GetAction>,
    pub set_actions: Vec<SetAction>,
}

impl Actions {
    pub fn new() -> Self {
        Actions {
            get_actions: Vec::default(),
            set_actions: Vec::default(),
        }
    }
}
