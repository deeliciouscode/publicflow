use crate::enums::LineName;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Platform {
    pub id: i32,
    pub since_last_pod: i32,
    pub edges_to: HashSet<i32>,
    pub lines_using_this: Vec<LineName>,
    pub pods_at_platform: HashSet<i32>,
    pub state: PlatformState,
}

impl Platform {
    pub fn new(id: i32, edges_to: &HashSet<i32>, lines_using_this: &Vec<LineName>) -> Self {
        Platform {
            id: id,
            since_last_pod: 0,
            edges_to: edges_to.clone(),
            lines_using_this: lines_using_this.clone(),
            pods_at_platform: HashSet::new(),
            state: PlatformState::Operational,
        }
    }

    pub fn is_operational(&self) -> bool {
        match self.state {
            PlatformState::Operational => true,
            _ => false,
        }
    }

    pub fn is_queuable(&self) -> bool {
        match self.state {
            PlatformState::Queuable { queue: _ } => true,
            _ => false,
        }
    }

    pub fn register_pod(&mut self, pod_id: i32) {
        self.pods_at_platform.insert(pod_id);
    }

    pub fn deregister_pod(&mut self, pod_id: i32) {
        self.pods_at_platform.remove(&pod_id);
    }

    pub fn queue_pod(&mut self, pod_id: i32) {
        match &self.state {
            PlatformState::Queuable { queue } => {
                let mut new_queue = queue.clone();
                new_queue.push(pod_id);
                self.state = PlatformState::Queuable { queue: new_queue }
            }
            _ => {
                println!(
                    "Pod can not be queued because platform is in state {:?}",
                    self.state
                )
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum PlatformState {
    Operational,
    Passable,
    Queuable { queue: Vec<i32> },
    InvalidState { reason: String },
}
