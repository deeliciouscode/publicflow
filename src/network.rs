use crate::config::structs::Config;
use crate::helper::enums::Direction;
use crate::helper::enums::LineName;
use crate::helper::functions::{calc_graph, get_screen_coordinates};
use crate::line::line::Line;
use crate::pod::podsbox::PodsBox;
use crate::station::platform::Platform;
use crate::station::station::Station;
use ggez::Context;
use petgraph::dot::{Config as PetConfig, Dot};
use petgraph::graph::UnGraph;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Network {
    pub stations: Vec<Station>,
    pub graph: UnGraph<u32, u32>,
    pub lines: Vec<Line>,
}

impl Network {
    pub fn new(stations: Vec<Station>, config: &Config) -> Self {
        let lines = config.network.lines.clone();
        let network = Network {
            stations: stations,
            graph: calc_graph(&lines),
            lines: lines,
        };
        // network.print_state();
        network
    }

    pub fn update(&mut self) {
        // TODO: make something useful with this
        for station in &mut self.stations {
            station.update();
        }
    }

    pub fn apply_show_station(&mut self, id: i32, permanent: bool) {
        for station in &mut self.stations {
            if station.id == id {
                if permanent {
                    station.visualize = true;
                } else {
                    station.visualize = true;
                }
            }
        }
    }

    pub fn apply_hide_station(&mut self, id: i32) {
        for station in &mut self.stations {
            if station.id == id {
                station.visualize = false;
            }
        }
    }

    pub fn apply_block_connection(&mut self, ids: &HashSet<i32>) {
        let ids_ref = &ids;
        for line in &mut self.lines {
            line.block_connection(ids_ref);
        }
    }

    pub fn apply_unblock_connection(&mut self, ids: &HashSet<i32>) {
        let ids_ref = &ids;
        for line in &mut self.lines {
            line.unblock_connection(ids_ref);
        }
    }

    pub fn apply_make_platform_op(
        &mut self,
        station_id: i32,
        line_name: LineName,
        direction: Direction,
    ) {
        for station in &mut self.stations {
            if station.id == station_id {
                station.make_operational(&line_name, &direction);
            }
        }
    }

    pub fn apply_make_platform_qu(
        &mut self,
        station_id: i32,
        line_name: LineName,
        direction: Direction,
    ) {
        for station in &mut self.stations {
            if station.id == station_id {
                station.make_queuable(&line_name, &direction);
            }
        }
    }

    pub fn apply_make_platform_pass(
        &mut self,
        station_id: i32,
        line_name: LineName,
        direction: Direction,
    ) {
        for station in &mut self.stations {
            if station.id == station_id {
                station.make_passable(&line_name, &direction);
            }
        }
    }

    pub fn apply_spawn_pod(
        &mut self,
        station_id: i32,
        line_name: LineName,
        direction: Direction,
        pods_box: &mut PodsBox,
        config: &Config,
    ) {
        for station in &mut self.stations {
            if station.id == station_id {
                station.spawn_pod(&line_name, &direction, pods_box, &self.lines, config);
            }
        }
    }

    pub fn try_get_station_by_id(&mut self, id: i32) -> Option<&mut Station> {
        for station in &mut self.stations {
            if station.id == id {
                return Some(station);
            }
        }
        return None;
    }

    pub fn try_get_platform(
        &mut self,
        id: i32,
        line_name: &LineName,
        direction: Direction,
    ) -> Option<&mut Platform> {
        for station in &mut self.stations {
            if station.id == id {
                for platform in &mut station.platforms {
                    if platform.lines_using_this.contains(line_name)
                        && platform.direction == direction
                    {
                        return Some(platform);
                    }
                }
            }
        }
        return None;
    }

    pub fn try_get_station_by_id_unmut(&self, id: i32) -> Option<&Station> {
        for station in &self.stations {
            if station.id == id {
                return Some(station);
            }
        }
        None
    }

    pub fn try_retrieve_station(&self, (x, y): (f32, f32), config: &Config) -> Option<&Station> {
        // println!("{}, {}", x, y);
        let mut closest_distance = 10000.;
        let mut closest_station = &self.stations[0];
        for station in &self.stations {
            let station_coords = get_screen_coordinates(station.coordinates, config);
            let distance = ((station_coords.0 - x).powi(2) + (station_coords.1 - y).powi(2)).sqrt();

            if distance < closest_distance && distance < 10. {
                closest_distance = distance;
                closest_station = station
            }
        }
        if closest_distance == 10000. {
            None
        } else {
            Some(closest_station)
        }
    }

    pub fn _print_state(&self) {
        for station in &self.stations {
            println!(
                "Station: {} | Pods: {:?}",
                station.id,
                station.get_pods_in_station_as_vec()
            )
        }
        println!(
            "{:?}",
            Dot::with_config(&self.graph, &[PetConfig::NodeIndexLabel])
        );
    }

    pub fn draw(&self, ctx: &mut Context, config: &Config) {
        for line in &self.lines {
            let _res = line.draw(ctx, self, config);
        }

        for station in &self.stations {
            let _res = station.draw(ctx, config); // TODO: handle result error case
        }
    }
}
