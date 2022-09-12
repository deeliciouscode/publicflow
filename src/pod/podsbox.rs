use crate::config::structs::Config;
use crate::helper::enums::{Direction, LineName};
use crate::line::line::Line;
use crate::line::linestate::LineState;
use crate::network::Network;
use crate::pod::pod::Pod;
use ggez::Context;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct PodsBox {
    pub pods: Vec<Pod>,
}

impl PodsBox {
    pub fn try_get_pod_by_id_mut(&mut self, pod_id: i32) -> Option<&mut Pod> {
        for pod in &mut self.pods {
            if pod.id == pod_id {
                return Some(pod);
            }
        }
        return None;
    }

    pub fn add_pod(
        &mut self,
        line_name: &LineName,
        direction: &Direction,
        lines: &Vec<Line>,
        config: &Config,
    ) {
        let id = self.get_highest_id() + 1;
        for line in lines {
            if &line.name == line_name {
                let line_max_ix = line.stations.len() as i32 - 1;
                let line_state;
                match direction {
                    Direction::Pos => {
                        line_state = LineState {
                            line: line.clone(),
                            line_ix: 0,
                            next_ix: 1,
                            direction: 1,
                        };
                    }
                    Direction::Neg => {
                        if line.circular {
                            line_state = LineState {
                                line: line.clone(),
                                line_ix: 0,
                                next_ix: line_max_ix,
                                direction: -1,
                            };
                        } else {
                            line_state = LineState {
                                line: line.clone(),
                                line_ix: line_max_ix,
                                next_ix: line_max_ix - 1,
                                direction: -1,
                            };
                        }
                    }
                }
                let pod = Pod::new(
                    id,
                    config.logic.pod_in_station_seconds,
                    config.logic.pod_capacity,
                    line_state,
                );
                self.pods.push(pod);
            }
        }
    }

    pub fn get_highest_id(&self) -> i32 {
        let mut highest_id = 0;
        for pod in &self.pods {
            highest_id = highest_id.max(pod.id);
        }
        return highest_id;
    }

    pub fn try_get_pod_by_id_unmut(&self, pod_id: i32) -> Option<&Pod> {
        for pod in &self.pods {
            if pod.id == pod_id {
                return Some(pod);
            }
        }
        return None;
    }

    pub fn try_retrieve_pod(&self, (x, y): (f32, f32)) -> Option<&Pod> {
        if &self.pods.len() == &0 {
            return None;
        }
        let mut closest_distance = 10000.;
        let mut closest_pod = &self.pods[0];
        for pod in &self.pods {
            let pod_coordinates = pod.get_coordinates();
            let distance =
                ((pod_coordinates.0 - x).powi(2) + (pod_coordinates.1 - y).powi(2)).sqrt();

            if distance < closest_distance && distance < 10. {
                closest_distance = distance;
                closest_pod = pod
            }
        }
        if closest_distance == 10000. {
            None
        } else {
            Some(closest_pod)
        }
    }

    pub fn draw(&self, ctx: &mut Context, config: &Config) {
        for pod in &self.pods {
            let _res = pod.draw(ctx, config);
        }
    }

    pub fn apply_show_pod(&mut self, id: i32, permanent: bool) {
        for pod in &mut self.pods {
            if pod.id == id {
                if permanent {
                    pod.visualize = true;
                } else {
                    pod.visualize = true;
                }
            }
        }
    }

    pub fn apply_hide_pod(&mut self, id: i32) {
        for pod in &mut self.pods {
            if pod.id == id {
                pod.visualize = false;
            }
        }
    }

    pub fn apply_block_connection(&mut self, ids: &HashSet<i32>) {
        let ids_ref = &ids;
        for pod in &mut self.pods {
            pod.line_state.line.block_connection(ids_ref);
        }
    }

    pub fn apply_unblock_connection(&mut self, ids: &HashSet<i32>) {
        let ids_ref = &ids;
        for pod in &mut self.pods {
            pod.line_state.line.unblock_connection(ids_ref);
        }
    }

    pub fn update(&mut self, network: &mut Network, config: &Config) {
        for pod in &mut self.pods {
            pod.update(network, config)
        }

        // TODO: figure out a way to do this in parralel, maybe with message queues or something.
        // self.pods.par_iter_mut().for_each(|pod| pod.update(network, config));
    }
}
