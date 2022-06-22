use petgraph::algo::astar;
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct PathState {
    pub path: VecDeque<NodeIndex<u32>>,
    pub current: NodeIndex<u32>,
}

// TODO: implement weights using the time to travel between stations
impl PathState {
    pub fn new(graph: &UnGraph<u32, u32>, start: u32, end: u32) -> Self {
        // println!("{:?}", graph);
        let maybe_path = astar(
            graph,
            NodeIndex::new(start as usize),
            |finish| finish == NodeIndex::new(end as usize),
            |e| *e.weight(),
            |_| 0,
        );

        match maybe_path {
            Some((_, path)) => {
                let path_state = PathState {
                    path: VecDeque::from(path),
                    current: NodeIndex::new(start as usize),
                };
                return path_state;
            }
            None => panic!("No connection between {} and {}", start, end),
        }
    }

    pub fn finished_journey(&self) -> bool {
        return self.path.len() == 1;
    }

    pub fn get_current_station_id(&self) -> Option<u32> {
        if self.path.len() >= 1 {
            Some(self.path[0].index() as u32)
        } else {
            None
        }
    }

    pub fn get_next_station_id(&self) -> Option<u32> {
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
