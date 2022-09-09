use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub enum PlatformState {
    Operational { queue: VecDeque<i32> },
    Queuable { queue: VecDeque<i32> },
    Passable,
}

impl PlatformState {
    pub fn get_queue(&self) -> &VecDeque<i32> {
        match self {
            PlatformState::Operational { queue } | PlatformState::Queuable { queue } => queue,
            PlatformState::Passable => {
                panic!("PlatformState::Passable doesn't have a queue")
            }
        }
    }
}
