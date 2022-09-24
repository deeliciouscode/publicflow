use crate::config::structs::Config;
use crate::helper::enums::{Direction, LineName};
use crate::line::line::Line;
use crate::line::linestate::LineState;
use crate::metrics::timeseries::TimeSeries;
use crate::metrics::traits::Series;
use crate::network::Network;
use crate::pod::pod::Pod;
use ggez::Context;
use std::collections::HashSet;
use std::fs::File;
use std::fs::*;
use std::io::prelude::*;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct PodsBox {
    pub pods: Vec<Pod>,
}

impl PodsBox {
    pub fn update(&mut self, network: &mut Network, config: &Config, time_passed: u32) {
        for pod in &mut self.pods {
            pod.update(network, config, time_passed)
        }
        // TODO: figure out a way to do this in parralel, maybe with message queues or something.
        // self.pods.par_iter_mut().for_each(|pod| pod.update(network, config));
    }
    pub fn try_get_pod_by_id_mut(&mut self, pod_id: i32) -> Option<&mut Pod> {
        for pod in &mut self.pods {
            if pod.id == pod_id {
                return Some(pod);
            }
        }
        return None;
    }

    pub fn start_gather_metrics(&mut self) {
        for pod in &mut self.pods {
            pod.start_gather_metrics();
        }
    }

    pub fn add_pod(
        &mut self,
        line_name: &LineName,
        direction: &Direction,
        station_id: &i32,
        lines: &Vec<Line>,
        config: &Config,
        time_passed: u32,
    ) {
        let id = self.get_highest_id() + 1;
        for line in lines {
            if &line.name == line_name {
                let mut line_ix = 0;
                for (i, st_id) in line.stations.iter().enumerate() {
                    if st_id == station_id {
                        line_ix = i as i32;
                    }
                }
                // println!("line_ix: {}", line_ix);
                // println!("lines: {:?}", lines);
                let line_max_ix = line.stations.len() as i32 - 1;
                let line_state;
                match direction {
                    Direction::Pos => {
                        let next_ix;
                        let direction;
                        if line.circular && line_ix == line_max_ix {
                            next_ix = 0;
                            direction = 1;
                        } else if line_ix == line_max_ix {
                            // this is basically just turning around
                            next_ix = line_ix - 1;
                            direction = -1;
                        } else {
                            next_ix = line_ix + 1;
                            direction = 1;
                        }

                        line_state = LineState {
                            line: line.clone(),
                            line_ix: line_ix,
                            next_ix: next_ix,
                            direction: direction,
                        };
                    }
                    Direction::Neg => {
                        let next_ix;
                        let direction;
                        if line.circular && line_ix == 0 {
                            next_ix = line_max_ix;
                            direction = -1;
                        } else if line_ix == 0 {
                            // this is basically just turning around
                            next_ix = line_ix + 1;
                            direction = 1;
                        } else {
                            next_ix = line_ix - 1;
                            direction = -1;
                        }

                        line_state = LineState {
                            line: line.clone(),
                            line_ix: line_ix,
                            next_ix: next_ix,
                            direction: direction,
                        };
                    }
                }
                let pod = Pod::new(
                    id,
                    config.logic.pod_in_station_seconds,
                    config.logic.pod_capacity,
                    line_state,
                    time_passed,
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

    pub fn apply_show_pod(&mut self, id: i32) {
        for pod in &mut self.pods {
            if pod.id == id {
                pod.visualize = true;
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

    pub fn dump_metrics(&self, pod_id: i32, config: &Config) {
        let maybe_pod = self.try_get_pod_by_id_unmut(pod_id);
        match maybe_pod {
            Some(pod) => {
                if let Some(timestamp_run) = config.timestamp_run {
                    let timestamp = timestamp_run
                        .naive_utc()
                        .format("%Y.%m.%d %H:%M")
                        .to_string()
                        .replace(" ", "_");
                    println!("timestamp_run: {:?}", timestamp);

                    let path_str = format!(
                        "{}/{}/{}/{}/{}/{}.txt",
                        "metrics", config.environment, timestamp, "pods", "ids", pod_id
                    );
                    let path = Path::new(&path_str);
                    let parent = path.parent().unwrap();
                    let _res = create_dir_all(parent);
                    let res = File::create(path);
                    match res {
                        Ok(mut file) => {
                            let txt = pod.time_series.format_to_file(String::from(
                                "ts,utilization,time_in_station,time_in_queue,time_driving,meters_traveled\n",
                            ));
                            let res = file.write_all(txt.as_bytes());
                            match res {
                                Ok(_) => {
                                    println!("written file");
                                }
                                Err(e) => {
                                    println!("error writing file: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("error opening file: {}", e);
                        }
                    }
                }
            }
            None => {}
        }
    }

    pub fn dump_all_metrics(&self, config: &Config) {
        for pod in &self.pods {
            self.dump_metrics(pod.id, config)
        }
    }

    pub fn dump_avg_metrics(&self, config: &Config) {
        let mut timeseries_accumulator = TimeSeries::new();
        for pod in &self.pods {
            timeseries_accumulator.add_layer(&pod.time_series);
        }

        timeseries_accumulator.normalize_by(self.pods.len() as u32);

        if let Some(timestamp_run) = config.timestamp_run {
            let timestamp = timestamp_run
                .naive_utc()
                .format("%Y.%m.%d %H:%M")
                .to_string()
                .replace(" ", "_");
            println!("timestamp_run: {:?}", timestamp);

            let path_str = format!(
                "{}/{}/{}/{}/{}.txt",
                "metrics", config.environment, timestamp, "pods", "avg"
            );
            let path = Path::new(&path_str);
            let parent = path.parent().unwrap();
            let _res = create_dir_all(parent);
            let res = File::create(path);
            match res {
                Ok(mut file) => {
                    let txt = timeseries_accumulator.format_to_file(String::from(
                        "ts,utilization,time_in_station,time_in_queue,time_driving,meters_traveled\n",
                    ));
                    let res = file.write_all(txt.as_bytes());
                    match res {
                        Ok(_) => {
                            println!("written file");
                        }
                        Err(e) => {
                            println!("error writing file: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("error opening file: {}", e);
                }
            }
        }
    }
}
