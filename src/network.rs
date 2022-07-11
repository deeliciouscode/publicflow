use crate::action::SetAction;
use crate::config::Config;
use crate::connection::{Connection, YieldTriple, YieldTuple};
use crate::helper::get_screen_coordinates;
use crate::line::Line;
use crate::station::Station;
use ggez::Context;
use petgraph::dot::{Config as PetConfig, Dot};
use petgraph::graph::{DiGraph, UnGraph};
// use std::;

#[derive(Clone, Debug)]
pub struct Network {
    pub stations: Vec<Station>,
    pub graph: UnGraph<u32, u32>,
    pub lines: Vec<Line>,
}

fn calc_graph(lines: &Vec<Line>) -> UnGraph<u32, u32> {
    let mut edges: Vec<(u32, u32, u32)> = vec![];

    for line in lines {
        for connection in &line.connections {
            if !connection.is_blocked {
                edges.push(connection.yield_triple())
            }
        }
    }
    let graph = UnGraph::from_edges(edges);
    graph
}

impl Network {
    pub fn new(stations: Vec<Station>, config: &Config) -> Self {
        let lines = config.network.lines.clone();
        let mut network = Network {
            stations: stations,
            graph: calc_graph(&lines),
            lines: lines,
        };
        // network.print_state();
        network
    }

    pub fn update(&mut self, set_actions: &Vec<SetAction>, config: &Config) {
        if set_actions.len() != 0 {
            for action in set_actions {
                match action {
                    SetAction::BlockConnection { ids } => {
                        let ids_ref = &ids;
                        for line in &mut self.lines {
                            line.block_connection(ids_ref);
                        }
                    }
                    SetAction::UnblockConnection { ids } => {
                        let ids_ref = &ids;
                        for line in &mut self.lines {
                            line.unblock_connection(ids_ref);
                        }
                    }
                    _ => {}
                }
            }
            self.graph = calc_graph(&self.lines);
        }
        for station in &mut self.stations {
            station.since_last_pod += 1;
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

    pub fn print_state(&self) {
        for station in &self.stations {
            println!(
                "Station: {} | Pods: {:?}",
                station.id, station.pods_in_station
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
