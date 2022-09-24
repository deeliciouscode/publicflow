use crate::config::structs::Config;
use crate::helper::enums::LineName;
use crate::helper::functions::get_screen_coordinates;
use crate::line::linestate::LineState;
use crate::metrics::components::pod::PodMetrics;
use crate::metrics::timeseries::TimeSeries;
use crate::network::Network;
use crate::pod::podstate::PodState;
use ggez::{graphics, Context, GameResult};
use std::collections::HashSet;
// use rayon::prelude::*; // For Parralelism

#[derive(Clone, Debug)]
pub struct Pod {
    pub id: i32,
    pub needs_initialization: bool,
    pub gather_metrics: bool,
    pub visualize: bool,
    pub metrics: PodMetrics,
    pub time_series: TimeSeries<PodMetrics>,
    pub in_station_for: i32,
    pub capacity: i32,
    pub people_in_pod: HashSet<i32>,
    pub coordinates: (f32, f32),
    pub line_state: LineState,
    pub state: PodState,
}

impl Pod {
    pub fn new(
        id: i32,
        in_station_for: i32,
        capacity: i32,
        line_state: LineState,
        _time_passed: u32, // This could be used again, if gather command becomes optional
    ) -> Self {
        let station_id = line_state.get_station_id();
        // let time_series = TimeSeries::from(time_passed);
        let time_series = TimeSeries::new();
        Pod {
            id: id,
            needs_initialization: true,
            gather_metrics: false,
            visualize: false,
            metrics: PodMetrics::new(),
            time_series: time_series,
            in_station_for: in_station_for,
            capacity: capacity,
            people_in_pod: HashSet::new(),
            coordinates: (0., 0.),
            line_state: line_state,
            state: PodState::InQueue {
                station_id: station_id,
            },
        }
    }

    // TODO: remove unused stuff
    pub fn update(&mut self, network: &mut Network, config: &Config, time_passed: u32) {
        if self.needs_initialization {
            self.initialize(network)
        }
        if self.gather_metrics {
            // println!("gather shit");
            self.do_gather_metrics(time_passed)
        }
        self.set_coordinates(network, config);
        match &self.state {
            PodState::BetweenStations {
                station_id_from: _,
                station_id_to: _,
                time_to_next_station,
            } => {
                // println!("Pod in BetweenStations State");
                if *time_to_next_station > 0 {
                    self.state = self.state.drive_a_sec();
                } else {
                    self.arrive_in_station(network);
                }
            }
            PodState::JustArrived { station_id: _ } => {
                // println!("Pod in JustArrived State");
                self.state = self.state.to_in_station();
            }
            PodState::InStation {
                station_id: _,
                time_in_station,
            } => {
                // if self.id == 0 {
                //     println!("Pod 0 in InStation state {}, {}", self.in_station_for, time_in_station);
                // }
                if self.in_station_for > *time_in_station {
                    self.state = self.state.wait_a_sec();
                } else {
                    self.depart_from_station(network);
                }
            }
            PodState::InQueue { station_id } => {
                self.check_if_in_station(network, *station_id);
            }
            PodState::InvalidState { reason } => {
                panic!("Pod {} is in invalid state. Reason: {}", self.id, reason)
            }
        }
    }

    pub fn start_gather_metrics(&mut self) {
        self.gather_metrics = true;
    }

    pub fn do_gather_metrics(&mut self, time_passed: u32) {
        self.metrics
            .set_utilization(self.people_in_pod.len() as f32 / self.capacity as f32);
        match &self.state {
            PodState::BetweenStations {
                station_id_from: _,
                station_id_to: _,
                time_to_next_station: _,
            } => {
                self.metrics.increase_time_driving();
            }
            PodState::JustArrived { station_id: _ } => {
                // TODO: use actual distance in network
                self.metrics.increase_meters_traveled(1000.);
                self.metrics.increase_time_in_station();
            }
            PodState::InStation {
                station_id: _,
                time_in_station: _,
            } => {
                self.metrics.increase_time_in_station();
            }
            PodState::InQueue { station_id: _ } => {
                self.metrics.increase_time_in_queue();
            }
            PodState::InvalidState { reason: _ } => {}
        }
        self.time_series
            .add_timestamp(time_passed, self.metrics.clone());
    }

    pub fn draw(&self, ctx: &mut Context, config: &Config) -> GameResult<()> {
        // let red = self.people_in_pod.len() as f32 / POD_CAPACITY as f32;
        // let green = 1. - red;

        let color = self.get_rgba().into();

        let mut _res: GameResult<()> = std::result::Result::Ok(());

        let (real_x, real_y) = self.get_coordinates();

        let circle = graphics::Mesh::new_circle(
            ctx,
            // graphics::DrawMode::stroke(2.),
            graphics::DrawMode::fill(),
            ggez::mint::Point2 {
                x: real_x,
                y: real_y,
            },
            config.visual.radius_pod,
            1.,
            color,
        )?;

        _res = graphics::draw(ctx, &circle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));

        if self.visualize {
            let circle = graphics::Mesh::new_circle(
                ctx,
                // graphics::DrawMode::stroke(2.),
                graphics::DrawMode::stroke(4.),
                ggez::mint::Point2 {
                    x: real_x,
                    y: real_y,
                },
                config.visual.radius_pod + 4.,
                1.,
                color,
            )?;

            _res = graphics::draw(ctx, &circle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
        }

        match _res {
            Err(err) => panic!("Error 3: {}", err),
            Ok(_m) => {
                // println!("No error at 3: {:?}", m);
                return _res;
            }
        }
    }

    fn get_rgba(&self) -> [f32; 4] {
        match self.line_state.line.name {
            LineName::U(1) => return [0.0, 1.0, 0.0, 1.0],
            LineName::U(2) => return [1.0, 0.0, 0.0, 1.0],
            LineName::U(3) => return [0.99, 0.63, 0.01, 1.0],
            LineName::U(4) => return [0.13, 0.74, 0.69, 1.0],
            LineName::U(5) => return [0.82, 0.73, 0.06, 1.0],
            LineName::U(6) => return [0.0, 0.0, 1.0, 1.0],
            LineName::U(7) => return [0.77, 0.75, 0.43, 1.0],
            LineName::U(8) => return [0.68, 0.67, 0.55, 1.0],
            _ => return [0.6, 0.6, 0.6, 1.0], // TODO: color for trams
                                              // any => panic!("The line: {} is not defined.", any),
        };
    }

    pub fn initialize(&mut self, net: &mut Network) {
        let station_id = self.state.get_station_id();
        let maybe_platform = net.try_get_platform(
            station_id,
            &self.line_state.line.name,
            self.line_state.get_direction(),
        );

        match maybe_platform {
            Some(platform) => {
                if platform.is_passable() {
                    println!("Passable is not yet implemented so the behaviour is the same as for operational.")
                } else {
                    let is_at_platform = platform.register_pod(self.id);
                    if is_at_platform {
                        self.state = self.state.to_just_arrived();
                    }
                    self.needs_initialization = false;
                }
            }
            None => {
                println!("Got no platform back")
            }
        }
    }

    fn arrive_in_station(&mut self, net: &mut Network) {
        self.line_state.update_line_ix();
        self.line_state.set_next_station_ix();
        let station_id_to = self.state.get_station_id_to();
        let maybe_platform = net.try_get_platform(
            station_id_to,
            &self.line_state.line.name,
            self.line_state.get_direction(),
        );

        match maybe_platform {
            Some(platform) => {
                if platform.is_passable() {
                    println!("Passable is not yet implemented so the behaviour is the same as for operational.")
                } else {
                    let is_at_platform = platform.register_pod(self.id);
                    if is_at_platform {
                        self.state = self.state.to_just_arrived();
                    } else {
                        self.state = self.state.to_in_queue();
                    }
                }
            }
            None => {
                println!("Got no platform back")
            }
        }
    }

    fn check_if_in_station(&mut self, net: &mut Network, station_id: i32) {
        let maybe_platform = net.try_get_platform(
            station_id,
            &self.line_state.line.name,
            self.line_state.get_direction(),
        );
        match maybe_platform {
            Some(platform) => {
                if platform.pods_at_platform.contains(&self.id) {
                    self.state = self.state.to_just_arrived()
                }
            }
            None => panic!("There is no station with id: {}", station_id),
        }
    }

    fn depart_from_station(&mut self, net: &mut Network) {
        let next = self.line_state.get_next_station_id();
        let current = self.state.get_station_id();
        // println!(
        //     "MARKER | departing after set: {}, {}, {} | pod_id: {}",
        //     next,
        //     current,
        //     self.line_state.get_station_id(),
        //     self.id
        // );
        let maybe_connection = self.line_state.try_get_connection(current, next);
        match maybe_connection {
            Some(connection) => {
                if connection.station_ids == HashSet::from([641, 650]) {
                    // println!("is blocked: {}", connection.is_blocked)
                }
                if !connection.is_blocked {
                    let maybe_platform = net.try_get_platform(
                        current,
                        &self.line_state.line.name,
                        self.line_state.get_direction(),
                    );

                    match maybe_platform {
                        Some(platform) => platform.deregister_pod(self.id),
                        None => panic!("There is no station with id: {}", current),
                    }
                    self.state = self.state.to_between_stations(next, connection.travel_time);
                }
            }
            None => panic!("There is no connection between: {} and {}", current, next),
        }
    }

    pub fn try_register_person(&mut self, person_id: i32) -> bool {
        // println!("------------------------------------------------------");
        // println!("self.people_in_pod.len(): {}", self.people_in_pod.len());
        // println!("self.capacity: {}", self.capacity);
        if self.people_in_pod.len() >= self.capacity as usize {
            return false;
        }
        self.people_in_pod.insert(person_id);
        return true;
    }

    pub fn get_coordinates(&self) -> (f32, f32) {
        return self.coordinates;
    }

    pub fn deregister_person(&mut self, person_id: &i32) {
        self.people_in_pod.remove(person_id);
    }

    pub fn is_in_just_arrived_state(&self) -> bool {
        match self.state {
            PodState::JustArrived { station_id: _ } => true,
            _ => false,
        }
    }

    pub fn set_coordinates(&mut self, network: &Network, config: &Config) {
        match self.state {
            PodState::BetweenStations {
                station_id_from,
                station_id_to,
                time_to_next_station,
            } => {
                let travel_time = self
                    .line_state
                    .try_get_connection(station_id_from, station_id_to)
                    .unwrap()
                    .travel_time;
                let station_from = network
                    .try_get_station_by_id_unmut(station_id_from)
                    .unwrap();
                let station_to = network.try_get_station_by_id_unmut(station_id_to).unwrap();
                let coordinates_from = get_screen_coordinates(station_from.coordinates, config);
                let coordinates_to = get_screen_coordinates(station_to.coordinates, config);
                let x = coordinates_from.0
                    + (coordinates_to.0 - coordinates_from.0)
                        * ((travel_time as f32 - time_to_next_station as f32) / travel_time as f32);
                let y = coordinates_from.1
                    + (coordinates_to.1 - coordinates_from.1)
                        * ((travel_time as f32 - time_to_next_station as f32) / travel_time as f32);
                let real_x = x;
                let real_y = y;
                self.coordinates = (real_x, real_y);
            }
            PodState::InQueue { station_id } => {
                let station = network.try_get_station_by_id_unmut(station_id).unwrap();
                self.coordinates = get_screen_coordinates(station.coordinates, config);
            }
            PodState::InStation {
                station_id,
                time_in_station: _,
            } => {
                let station = network.try_get_station_by_id_unmut(station_id).unwrap();
                self.coordinates = get_screen_coordinates(station.coordinates, config);
            }
            PodState::JustArrived { station_id } => {
                let station = network.try_get_station_by_id_unmut(station_id).unwrap();
                self.coordinates = get_screen_coordinates(station.coordinates, config);
            }
            _ => {}
        }
    }

    pub fn get_station_id(&self) -> i32 {
        self.state.get_station_id()
    }
}
