#[derive(Debug, Clone, PartialEq)]
pub enum GetAction {
    GetStation { station_id: i32 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetAction {
    BlockLine { connection: (i32, i32) },
}
