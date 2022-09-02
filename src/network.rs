use crate::action::SetAction;
use crate::config::Config;
use crate::connection::{Connection, YieldTriple, YieldTuple};
use crate::helper::get_screen_coordinates;
use crate::line::Line;
use crate::station::{Station, StationGroup};
use ggez::Context;
use petgraph::dot::{Config as PetConfig, Dot};
use petgraph::graph::UnGraph;
// use std::;

#[derive(Clone, Debug)]
pub struct Network {
    pub station_groups: Vec<StationGroup>,
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
    pub fn new(station_groups: Vec<StationGroup>, config: &Config) -> Self {
        let lines = config.network.lines.clone();
        let mut network = Network {
            station_groups: station_groups,
            graph: calc_graph(&lines),
            lines: lines,
        };
        // network.print_state();
        network
    }

    pub fn update(&mut self, set_actions: &Vec<SetAction>, config: &Config) {
        if set_actions.len() != 0 {
            let mut recalculate_graph = false;
            for action in set_actions {
                match action {
                    // TODO: differentiate between permament and not
                    SetAction::ShowStation { id, permanent } => {
                        for station_group in &mut self.station_groups {
                            if station_group.id == *id {
                                if *permanent {
                                    station_group.visualize = true;
                                } else {
                                    station_group.visualize = true;
                                }
                            }
                        }
                    }
                    SetAction::HideStation { id } => {
                        for station_group in &mut self.station_groups {
                            if station_group.id == *id {
                                station_group.visualize = false;
                            }
                        }
                    }
                    SetAction::BlockConnection { ids } => {
                        let ids_ref = &ids;
                        for line in &mut self.lines {
                            line.block_connection(ids_ref);
                        }
                        recalculate_graph = true;
                    }
                    SetAction::UnblockConnection { ids } => {
                        let ids_ref = &ids;
                        for line in &mut self.lines {
                            line.unblock_connection(ids_ref);
                        }
                        recalculate_graph = true;
                    }
                    _ => {}
                }
            }
            if recalculate_graph {
                self.graph = calc_graph(&self.lines);
            }
        }
        // TODO: make something useful with this
        for station_group in &mut self.station_groups {
            station_group.update();
        }
    }

    pub fn try_get_station_group_by_id(&mut self, id: i32) -> Option<&mut StationGroup> {
        for station_group in &mut self.station_groups {
            if station_group.id == id {
                return Some(station_group);
            }
        }
        return None;
    }

    pub fn try_get_station_group_by_station_id(&mut self, id: i32) -> Option<&mut StationGroup> {
        for station_group in &mut self.station_groups {
            for station in &mut station_group.stations {
                if station.id == id {
                    return Some(station_group);
                }
            }
        }
        return None;
    }

    pub fn try_get_station_by_id(&mut self, id: i32) -> Option<&mut Station> {
        for station_group in &mut self.station_groups {
            for station in &mut station_group.stations {
                if station.id == id {
                    return Some(station);
                }
            }
        }
        return None;
    }

    pub fn try_get_station_by_id_unmut(&self, id: i32) -> Option<&Station> {
        for station_group in &self.station_groups {
            for station in &station_group.stations {
                if station.id == id {
                    return Some(station);
                }
            }
        }
        None
    }

    pub fn try_retrieve_station_group(
        &self,
        (x, y): (f32, f32),
        config: &Config,
    ) -> Option<&StationGroup> {
        // println!("{}, {}", x, y);
        let mut closest_distance = 10000.;
        let mut closest_station = &self.station_groups[0];
        for station_group in &self.station_groups {
            let station_coords = get_screen_coordinates(station_group.coordinates, config);
            let distance = ((station_coords.0 - x).powi(2) + (station_coords.1 - y).powi(2)).sqrt();

            if distance < closest_distance && distance < 10. {
                closest_distance = distance;
                closest_station = station_group
            }
        }
        if closest_distance == 10000. {
            None
        } else {
            Some(closest_station)
        }
    }

    pub fn print_state(&self) {
        for station_group in &self.station_groups {
            println!(
                "Station: {} | Pods: {:?}",
                station_group.id, station_group.pods_in_station_group
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

        for station_group in &self.station_groups {
            let _res = station_group.draw(ctx, config); // TODO: handle result error case
        }
    }
}
