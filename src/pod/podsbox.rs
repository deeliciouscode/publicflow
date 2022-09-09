use crate::config::structs::Config;
use crate::control::action::SetAction;
use crate::network::Network;
use crate::pod::pod::Pod;
use ggez::Context;

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

    pub fn try_get_pod_by_id_unmut(&self, pod_id: i32) -> Option<&Pod> {
        for pod in &self.pods {
            if pod.id == pod_id {
                return Some(pod);
            }
        }
        return None;
    }

    pub fn try_retrieve_pod(&self, (x, y): (f32, f32)) -> Option<&Pod> {
        // println!("{}, {}", x, y);
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

    pub fn update(&mut self, network: &mut Network, set_actions: &Vec<SetAction>, config: &Config) {
        for action in set_actions {
            match action {
                // TODO: differentiate between permament and not
                SetAction::ShowPod { id, permanent } => {
                    for pod in &mut self.pods {
                        if pod.id == *id {
                            if *permanent {
                                pod.visualize = true;
                            } else {
                                pod.visualize = true;
                            }
                        }
                    }
                }
                SetAction::HidePod { id } => {
                    for pod in &mut self.pods {
                        if pod.id == *id {
                            pod.visualize = false;
                        }
                    }
                }
                SetAction::BlockConnection { ids } => {
                    let ids_ref = &ids;
                    for pod in &mut self.pods {
                        pod.line_state.line.block_connection(ids_ref);
                    }
                }
                SetAction::UnblockConnection { ids } => {
                    let ids_ref = &ids;
                    for pod in &mut self.pods {
                        pod.line_state.line.unblock_connection(ids_ref);
                    }
                }
                _ => {}
            }
        }
        for pod in &mut self.pods {
            pod.update(network, config)
        }

        // TODO: figure out a way to do this in parralel, maybe with message queues or something.
        // self.pods.par_iter_mut().for_each(|pod| pod.update(network, config));
    }
}
