use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub enum PlatformState {
    Operational { queue: VecDeque<i32> },
    Queuable { queue: VecDeque<i32> },
    Passable,
}
