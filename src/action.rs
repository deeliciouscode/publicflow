use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum GetAction {
    GetStation { station_id: i32 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetAction {
    BlockConnection { ids: HashSet<i32> },
}
