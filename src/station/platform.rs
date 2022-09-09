use crate::config::structs::Config;
use crate::helper::enums::{Direction, LineName};
use crate::station::platformstate::PlatformState;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Platform {
    pub direction: Direction,
    pub since_last_pod: i32,
    pub seconds_between_pods: i32,
    pub edges_to: HashSet<i32>,
    pub lines_using_this: Vec<LineName>,
    pub pods_at_platform: HashSet<i32>,
    pub state: PlatformState,
}

impl Platform {
    pub fn new(
        config: &Config,
        direction: Direction,
        edges_to: &HashSet<i32>,
        lines_using_this: &Vec<LineName>,
    ) -> Self {
        Platform {
            direction: direction,
            since_last_pod: 0,
            seconds_between_pods: 3600 / config.logic.pods_per_hour,
            edges_to: edges_to.clone(),
            lines_using_this: lines_using_this.clone(),
            pods_at_platform: HashSet::new(),
            state: PlatformState::Operational {
                queue: VecDeque::from([]),
            },
        }
    }

    // TODO: choose sensible rate at which a platform can let pods through
    pub fn update(&mut self) {
        self.since_last_pod += 1;
        // if self.edges_to == HashSet::from([1, 0]) && self.direction == Direction::Pos {
        //     println!("Since last pod: {} | seconds between: {}", self.since_last_pod, self.seconds_between_pods);
        // }
        if self.since_last_pod >= self.seconds_between_pods {
            match self.state {
                PlatformState::Operational { queue: _ } => {
                    self.let_pod_enter();
                }
                _ => {}
            }
        }
    }

    pub fn _is_operational(&self) -> bool {
        match self.state {
            PlatformState::Operational { queue: _ } => true,
            _ => false,
        }
    }

    pub fn _is_queuable(&self) -> bool {
        match self.state {
            PlatformState::Queuable { queue: _ } => true,
            _ => false,
        }
    }

    pub fn is_passable(&self) -> bool {
        match self.state {
            PlatformState::Passable => true,
            _ => false,
        }
    }

    pub fn let_pod_enter(&mut self) {
        match &self.state {
            PlatformState::Operational { queue } => {
                let mut queue = queue.clone();
                if let Some(pod_id) = queue.pop_front() {
                    self.pods_at_platform.insert(pod_id);
                    self.since_last_pod = 0;
                }
                self.state = PlatformState::Operational { queue: queue }
            }
            _ => {}
        }
    }

    pub fn _try_get_queue(&self) -> Option<&VecDeque<i32>> {
        match &self.state {
            PlatformState::Queuable { queue } => {
                return Some(&queue);
            }
            PlatformState::Operational { queue } => {
                return Some(&queue);
            }
            _ => return None,
        }
    }

    pub fn register_pod(&mut self, pod_id: i32) -> bool {
        match &self.state {
            PlatformState::Queuable { queue } => {
                let mut new_queue = queue.clone();
                new_queue.push_back(pod_id);
                self.state = PlatformState::Queuable { queue: new_queue };
                return false;
            }
            PlatformState::Operational { queue } => {
                let mut new_queue = queue.clone();
                new_queue.push_back(pod_id);
                self.state = PlatformState::Operational { queue: new_queue };
                if self.state.get_queue().len() == 1
                    && self.since_last_pod >= self.seconds_between_pods
                {
                    self.let_pod_enter();
                    return true;
                } else {
                    return false;
                }
            }
            _ => {
                panic!(
                    "Pod can not be queued because platform is in state {:?}",
                    self.state
                );
            }
        }
        // self.pods_at_platform.insert(pod_id);
    }

    pub fn deregister_pod(&mut self, pod_id: i32) {
        self.pods_at_platform.remove(&pod_id);
    }
}
