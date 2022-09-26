use crate::config::structs::Config;
use crate::control::action::Action;
use crate::helper::functions::{get_random_station_id, get_screen_coordinates};
use crate::metrics::components::person::PersonMetrics;
use crate::metrics::timeseries::TimeSeries;
use crate::network::Network;
use crate::pathstate::PathState;
use crate::person::personstate::PersonState;
use crate::pod::pod::Pod;
use crate::pod::podsbox::PodsBox;
use ggez::{graphics, Context, GameResult};
use petgraph::graph::UnGraph;

#[derive(Clone, Debug, Default)]
pub struct Person {
    pub id: i32,
    pub visualize: bool,
    pub gather_metrics: bool,
    transition_time: i32,
    pub metrics: PersonMetrics,
    pub time_series: TimeSeries<PersonMetrics>,
    pub real_coordinates: (f32, f32),
    pub state: PersonState,
    pub stay_at_station_id: Option<u32>,
    pub path_state: PathState,
    pub action_to_process: Option<Action>,
}

impl Person {
    pub fn new(
        id: i32,
        transition_time: i32,
        network: &Network,
        start: i32,
        finish: i32,
        config: &Config,
    ) -> Self {
        let mut person = Person {
            id: id,
            visualize: false,
            gather_metrics: false,
            transition_time: transition_time,
            time_series: TimeSeries::new(),
            metrics: PersonMetrics::new(),
            real_coordinates: (0., 0.),
            state: PersonState::Transitioning {
                station_id: start,
                previous_pod_id: -1,
                time_in_station: transition_time - 1,
            },
            stay_at_station_id: None,
            path_state: PathState::new(
                &network.graph,
                start as u32,
                finish as u32,
                network,
                config,
            ),
            action_to_process: None,
        };
        person.set_coordinates_of_station(
            person.path_state.try_get_current_station_id().unwrap() as i32,
            network,
            config,
        );
        // println!("{:?}", person.path_state);
        person
    }

    pub fn update(
        &mut self,
        pods_box: &mut PodsBox,
        network: &mut Network,
        config: &Config,
        time_passed: u32,
    ) {
        if self.gather_metrics {
            // println!("gather shit");
            self.do_gather_metrics(time_passed)
        }
        // println!("person state: {:?}", self.state);
        match &self.state {
            PersonState::ReadyToTakePod { station_id } => {
                // println!("person in ready state");
                // Assign first instead of using directly because:
                // https://github.com/rust-lang/rust/issues/59159
                let station_id_deref = *station_id;
                self.try_to_take_next_pod(pods_box, network, station_id_deref, config);
            }
            PersonState::RidingPod {
                pod_id,
                just_got_in: _,
            } => {
                // println!("person in riding state");
                let pod_id_deref = *pod_id;
                self.ride_pod(pods_box, pod_id_deref);
            }
            PersonState::JustArrived {
                pod_id: _,
                station_id: _,
            } => {} // This case is handled in get_out_if_needed()
            PersonState::Transitioning {
                station_id: _,
                previous_pod_id: _,
                time_in_station,
            } => {
                if *time_in_station < self.transition_time {
                    // println!("person in transitioning state and not ready.");
                    self.state = self.state.wait_a_sec();
                } else {
                    // println!("person in transitioning state and going to ready state.");
                    self.state = self.state.to_ready();
                }
            }
        }
    }

    pub fn start_gather_metrics(&mut self) {
        self.gather_metrics = true;
    }

    pub fn do_gather_metrics(&mut self, time_passed: u32) {
        match &self.state {
            PersonState::ReadyToTakePod { station_id: _ } => {
                self.metrics.increase_time_in_station();
            }
            PersonState::RidingPod {
                pod_id: _,
                just_got_in,
            } => {
                if *just_got_in {
                    self.metrics.increase_number_of_pods();
                    self.state = self.state.remove_just_got_in();
                }
                self.metrics.increase_time_in_pods();
            }
            PersonState::JustArrived {
                pod_id: _,
                station_id: _,
            } => {} // This case is handled in get out if needed, by necessity
            PersonState::Transitioning {
                station_id: _,
                previous_pod_id: _,
                time_in_station: _,
            } => {
                self.metrics.increase_time_in_station();
            }
        }
        self.time_series
            .add_timestamp(time_passed, self.metrics.clone());
    }

    pub fn new_path(
        &mut self,
        graph: &UnGraph<u32, u32>,
        start: u32,
        finish: u32,
        network: &Network,
        config: &Config,
    ) {
        self.path_state = PathState::new(graph, start, finish, network, config);
        // println!("{:?}", self.path_state);
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let color = [0.0, 1.0, 1.0, 1.0].into();

        let mut _res: GameResult<()> = std::result::Result::Ok(());

        let (real_x, real_y) = self.real_coordinates;

        let circle = graphics::Mesh::new_circle(
            ctx,
            // graphics::DrawMode::stroke(2.),
            graphics::DrawMode::stroke(4.),
            ggez::mint::Point2 {
                x: real_x,
                y: real_y,
            },
            8.,
            1.,
            color,
        )?;

        _res = graphics::draw(ctx, &circle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));

        match _res {
            Err(err) => panic!("Error 3: {}", err),
            Ok(_m) => {
                // println!("No error at 3: {:?}", m);
                return _res;
            }
        }
    }

    fn try_process_action(
        &mut self,
        current_station_id: u32,
        network: &mut Network,
        config: &Config,
    ) {
        match &mut self.action_to_process {
            Some(action) => match action {
                Action::RoutePerson {
                    id: _,
                    station_id,
                    stay_there,
                    random_station,
                } => {
                    if *random_station {
                        self.stay_at_station_id = None;
                        let random_station_id = get_random_station_id(config);
                        self.new_path(
                            &network.graph,
                            current_station_id,
                            random_station_id,
                            network,
                            config,
                        )
                    } else {
                        if *stay_there {
                            self.stay_at_station_id = Some(*station_id);
                        } else {
                            self.stay_at_station_id = None;
                        }
                        let station_id_finish = *station_id;
                        self.new_path(
                            &network.graph,
                            current_station_id,
                            station_id_finish,
                            network,
                            config,
                        );
                    }
                    self.action_to_process = None;
                }
                _ => {}
            },
            None => {}
        }
    }

    pub fn get_out_if_needed(
        &mut self,
        pods_box: &mut PodsBox,
        network: &mut Network,
        config: &Config,
    ) {
        match &self.state {
            PersonState::JustArrived {
                pod_id,
                station_id: _,
            } => {
                let pod_id_deref = *pod_id;
                self.decide_on_arrival(pods_box, network, pod_id_deref, config);
                let maybe_station_id = self.try_get_station_id();
                match maybe_station_id {
                    Some(station_id) => {
                        self.set_coordinates_of_station(station_id, network, config);
                    }
                    None => {}
                }
            }
            _ => {}
        }
    }

    fn try_to_take_next_pod(
        &mut self,
        pods_box: &mut PodsBox,
        network: &mut Network,
        station_id: i32,
        config: &Config,
    ) {
        if let Some(station_id_stay) = self.stay_at_station_id {
            if station_id as u32 == station_id_stay {
                self.try_process_action(station_id_stay, network, config);
                return;
            }
        }

        let maybe_next_station_id = self.path_state.try_get_next_station_id();
        match maybe_next_station_id {
            Some(next_station_id) => {
                let station = network.try_get_station_by_id(station_id).unwrap();
                let maybe_pod_ids: Option<Vec<i32>> = station.try_get_pod_ids_in_station_as_vec();
                // println!("maybe_pod_ids: {:?}", maybe_pod_ids);
                match maybe_pod_ids {
                    Some(pod_ids) => {
                        // println!(
                        //     "{}, {}",
                        //     station_id,
                        //     self.path_state.get_current_station_id().unwrap()
                        // );

                        for pod_id in pod_ids {
                            let pod = pods_box.try_get_pod_by_id_mut(pod_id).unwrap();
                            // println!(
                            //     "next_station ids: {}, {}",
                            //     pod.line_state.get_next_station_id(),
                            //     next_station_id
                            // );
                            if pod.line_state.get_next_station_id() == next_station_id as i32 {
                                let got_in = pod.try_register_person(self.id);
                                // println!("got_in: {}", got_in);
                                if got_in {
                                    // println!("Getting into pod with id: {} now", pod_id);
                                    self.state = self.state.to_riding(pod_id);
                                    let station =
                                        network.try_get_station_by_id(station_id).unwrap();
                                    station.deregister_person(self.id);
                                    break;
                                }
                            }
                        }
                    }
                    None => {} // None => println!("Can't leave the station, no pod here."),
                }
            }
            None => {
                let finish = get_random_station_id(config);
                self.new_path(&network.graph, station_id as u32, finish, network, config);
                // println!(
                //     "person {} is at {} and will go to {} next, taking path {:?}.",
                //     self.id,
                //     self.state.try_get_station_id().unwrap(),
                //     finish,
                //     self.path_state
                // );
                return; // TODO: remove the 1 second delay that is happening when the new_path = old_path
            }
        }
    }

    fn ride_pod(&mut self, pods_box: &mut PodsBox, pod_id: i32) {
        let maybe_pod = pods_box.try_get_pod_by_id_mut(pod_id);
        match maybe_pod {
            Some(pod) => {
                if self.visualize {
                    self.set_coordinates_of_pod(pod)
                }
                if pod.is_in_just_arrived_state() {
                    // TODO: meters increase dependent on connection
                    if self.gather_metrics {
                        self.metrics
                            .increase_meters_traveled(pod.state.get_distance_travelled() as f32);
                    }
                    self.state = self.state.to_just_arrived(pod.get_station_id());
                }
            }
            None => panic!("Pod with id: {} does not exist.", pod_id),
        }
    }

    fn decide_on_arrival(
        &mut self,
        pods_box: &mut PodsBox,
        network: &mut Network,
        pod_id: i32,
        config: &Config,
    ) {
        self.path_state.arrive();
        // println!("self.path_state: {:?}", self.path_state);
        let pod = pods_box.try_get_pod_by_id_mut(pod_id).unwrap();
        let line_next_station_id = pod.line_state.get_next_station_id();
        let maybe_next_station_id = self.path_state.try_get_next_station_id();
        match maybe_next_station_id {
            Some(desired_next_station_id) => {
                if line_next_station_id != desired_next_station_id as i32
                    || self.action_to_process.is_some()
                {
                    self.state = self.state.to_transitioning();
                    let station = network
                        .try_get_station_by_id(pod.line_state.get_station_id())
                        .unwrap();
                    station.register_person(self.id);
                    let pod = pods_box.try_get_pod_by_id_mut(pod_id).unwrap();
                    pod.deregister_person(&self.id);
                    self.try_process_action(station.id as u32, network, config);
                } else {
                    self.state = self.state.to_riding(pod_id);
                }
            }
            None => {
                // println!(
                //     "self.path_state.finished_journey(): {}",
                //     self.path_state.finished_journey()
                // );
                if self.path_state.finished_journey() {
                    self.state = self.state.to_transitioning();
                    let station = network
                        .try_get_station_by_id(pod.line_state.get_station_id())
                        .unwrap();
                    station.register_person(self.id);
                    let pod = pods_box.try_get_pod_by_id_mut(pod_id).unwrap();
                    pod.deregister_person(&self.id);
                    self.try_process_action(station.id as u32, network, config);
                }
            }
        }
    }

    fn set_coordinates_of_station(&mut self, station_id: i32, network: &Network, config: &Config) {
        // println!("set real coords");
        let station = network.try_get_station_by_id_unmut(station_id).unwrap();
        let coords_station = get_screen_coordinates(station.coordinates, config);
        self.real_coordinates = (coords_station.0, coords_station.1)
    }

    fn set_coordinates_of_pod(&mut self, pod: &Pod) {
        // println!("set real coords");
        let coords_station = pod.get_coordinates();
        self.real_coordinates = (coords_station.0, coords_station.1)
    }

    pub fn try_get_station_id(&self) -> Option<i32> {
        self.state.try_get_station_id()
    }
}
