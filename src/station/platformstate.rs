#[derive(Clone, Debug)]
pub enum PlatformState {
    Operational,
    Passable,
    Queuable { queue: Vec<i32> },
    _InvalidState { reason: String },
}
