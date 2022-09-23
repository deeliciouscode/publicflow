use crate::config::structs::Config;
use crate::helper::functions::get_air_travel_time;
use crate::network::Network;
use petgraph::algo::astar;
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::VecDeque;

#[derive(Clone, Debug, Default)]
pub struct PathState {
    pub path: VecDeque<NodeIndex<u32>>,
    pub current: NodeIndex<u32>,
}

impl PathState {
    pub fn new(
        graph: &UnGraph<u32, u32>,
        start: u32,
        end: u32,
        network: &Network,
        config: &Config,
    ) -> Self {
        // println!("{:?}", graph);
        // println!("start: {}, end: {}", start, end);
        // println!("air_travel_time: {}", get_air_travel_time(start, end, network, config));
        let maybe_path = astar(
            graph,
            NodeIndex::new(start as usize),
            |finish| finish == NodeIndex::new(end as usize),
            |e| *e.weight(),
            |_| get_air_travel_time(start, end, network, config), // use air distance as heuristik
        );

        match maybe_path {
            Some((_, path)) => {
                // println!("There is a connection between {} and {}", start, end);
                let path_state = PathState {
                    path: VecDeque::from(path),
                    current: NodeIndex::new(start as usize),
                };
                return path_state;
            }
            None => {
                // TODO: make this more robust, define clearly what happens if no path can be found
                // println!("No connection between {} and {}", start, end);
                let path_state = PathState {
                    path: VecDeque::from([NodeIndex::new(start as usize)]),
                    current: NodeIndex::new(start as usize),
                };
                return path_state;
            }
        }
    }

    pub fn finished_journey(&self) -> bool {
        return self.path.len() == 1;
    }

    pub fn try_get_current_station_id(&self) -> Option<u32> {
        if self.path.len() >= 1 {
            Some(self.path[0].index() as u32)
        } else {
            None
        }
    }

    pub fn try_get_next_station_id(&self) -> Option<u32> {
        if self.path.len() >= 2 {
            Some(self.path[1].index() as u32)
        } else {
            None
        }
    }

    pub fn arrive(&mut self) {
        self.path.pop_front();
    }
}
