use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub enum PlatformState {
    Operational { queue: VecDeque<i32> },
    Queueable { queue: VecDeque<i32> },
    _Passable,
}

impl PlatformState {
    pub fn get_queue(&self) -> &VecDeque<i32> {
        match self {
            PlatformState::Operational { queue } | PlatformState::Queueable { queue } => queue,
            PlatformState::_Passable => {
                panic!("PlatformState::Passable doesn't have a queue")
            }
        }
    }

    pub fn make_operational(&self) -> PlatformState {
        match &self {
            PlatformState::Queueable { queue } => PlatformState::Operational {
                queue: queue.clone(),
            },
            PlatformState::_Passable => PlatformState::Operational {
                queue: VecDeque::from([]),
            },
            PlatformState::Operational { queue: _ } => {
                println!("Is Operational already.");
                self.clone()
            }
        }
    }

    pub fn make_queueable(&self) -> PlatformState {
        match self {
            PlatformState::Operational { queue } => PlatformState::Queueable {
                queue: queue.clone(),
            },
            PlatformState::_Passable => PlatformState::Queueable {
                queue: VecDeque::from([]),
            },
            PlatformState::Queueable { queue: _ } => {
                println!("Is Queueable already.");
                self.clone()
            }
        }
    }

    pub fn make_passable(&self) -> PlatformState {
        println!(
            "Passable is not implemented for Platform, stay in state: {:?}",
            self
        );
        self.clone()
    }
}
