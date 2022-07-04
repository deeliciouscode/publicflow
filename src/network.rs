use crate::config::Config as SimConfig;
use crate::connection::{Connection, YieldTriple, YieldTuple};
use crate::helper::get_real_coordinates;
use crate::line::Line;
use crate::station::Station;
use ggez::Context;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, UnGraph};
// use std::;

#[derive(Clone, Debug)]
pub struct Network {
    pub stations: Vec<Station>,
    pub graph: UnGraph<u32, u32>,
    pub lines: Vec<Line>,
    pub config: SimConfig,
    // pub mesh
}

fn calc_graph(lines: &Vec<Line>) -> UnGraph<u32, u32> {
    let mut edges: Vec<(u32, u32, u32)> = vec![];

    for line in lines {
        for connection in &line.connections {
            edges.push(connection.yield_triple())
        }
    }
    let graph = UnGraph::from_edges(edges);
    graph
}

impl Network {
    pub fn new(stations: Vec<Station>, config: &SimConfig) -> Self {
        let lines = config.network.lines.clone();
        let mut network = Network {
            stations: stations,
            graph: calc_graph(&lines),
            lines: lines,
            config: config.clone(),
        };
        network.print_state();
        network
    }

    pub fn update(&mut self) {
        for station in &mut self.stations {
            station.since_last_pod += 1;
        }
    }

    // fn calc_graph

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

    pub fn try_retrieve_station(&self, (x, y): (f32, f32)) -> Option<&Station> {
        // println!("{}, {}", x, y);
        let mut closest_distance = 10000.;
        let mut closest_station = &self.stations[0];
        for station in &self.stations {
            let station_coords = get_real_coordinates(station.coordinates);
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
            // println!(
            //     "Station: {} | Pods: {:?}",
            //     station.id, station.pods_in_station
            // )
        }
        // println!(
        //     "{:?}",
        //     Dot::with_config(&self.graph, &[Config::NodeIndexLabel])
        // );
    }

    pub fn draw(&self, ctx: &mut Context) {
        for line in &self.lines {
            let _res = line.draw(ctx, self);
        }

        for station in &self.stations {
            let _res = station.draw(ctx); // TODO: handle result error case
        }
    }
}
