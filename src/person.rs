use crate::config::{MAX_XY, OFFSET, SCREEN_SIZE, SIDELEN_STATION, WIDTH_LINE, WIDTH_POD};
use crate::helper::get_real_coordinates;
use crate::line::LineState;
use crate::network::Network;
use crate::pathstate::PathState;
use crate::pod::PodsBox;
use ggez::graphics::Rect;
use ggez::{graphics, Context, GameResult};
use petgraph::graph::UnGraph;
use rand::Rng;

// TODO: implement destinations
#[derive(Clone, Debug)]
pub struct PeopleBox {
    pub people: Vec<Person>,
}

impl PeopleBox {
    pub fn print_state(&self) {
        // Get people who need to get out first
        for person in &self.people {
            let maybe_station_id = person.try_get_station_id();
            let station_id;
            match maybe_station_id {
                Some(_station_id) => station_id = _station_id.to_string(),
                None => station_id = String::from("None"),
            }
            let maybe_pod_id = person.try_get_pod_id();
            let pod_id;
            match maybe_pod_id {
                Some(_pod_id) => pod_id = _pod_id.to_string(),
                None => pod_id = String::from("None"),
            }

            println!(
                "Person: {} | Station: {} | Pod: {} | State: {:?}",
                person.id, station_id, pod_id, person.state
            )
        }
    }

    // people are not drawn explicitely but only as numbers and circles with colors
    // on pods and in stations.
    // pub fn draw(&self, ctx: &mut Context, network: &Network) {
    //     for pod in &self.people {
    //         let _res = pod.draw(ctx);
    //     }
    // }

    pub fn update(&mut self, pods_box: &mut PodsBox, network: &mut Network) {
        for person in &mut self.people {
            person.get_out_if_needed(pods_box, network);
        }
        for person in &mut self.people {
            person.update_state(pods_box, network);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Person {
    pub id: i32,
    transition_time: i32,
    real_coordinates: (f32, f32),
    state: PersonState,
    pub path_state: PathState,
}

impl Person {
    pub fn new(id: i32, transition_time: i32, network: &Network, start: i32, finish: i32) -> Self {
        let mut person = Person {
            id: id,
            transition_time: transition_time,
            real_coordinates: (0., 0.),
            state: PersonState::Transitioning {
                station_id: start,
                previous_pod_id: -1,
                time_in_station: transition_time - 1,
            },
            path_state: PathState::new(&network.graph, start as u32, finish as u32),
        };
        person.set_real_coordinates(0, network);
        // println!("{:?}", person.path_state);
        person
    }

    pub fn new_path(&mut self, graph: &UnGraph<u32, u32>, start: u32, finish: u32) {
        self.path_state = PathState::new(graph, start, finish);
        // println!("{:?}", self.path_state);
    }

    // fn draw(&self, ctx: &mut Context) -> GameResult<()> {
    //     let color = [1.0, 0.2, 0.2, 1.0].into();
    //     let mut res: GameResult<()> = std::result::Result::Ok(());
    //     let mut draw_in_station = || -> GameResult<()> {
    //         // println!("real: {:?}", self.real_coordinates);
    //         let station_rect = Rect {
    //             x: self.real_coordinates.0,
    //             y: self.real_coordinates.1,
    //             w: 5.,
    //             h: 5.,
    //         };
    //         let rectangle = graphics::Mesh::new_rectangle(
    //             ctx,
    //             graphics::DrawMode::fill(),
    //             station_rect,
    //             color,
    //         )?;
    //         let rez = graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
    //         return rez;
    //     };

    //     match &self.state {
    //         PersonState::ReadyToTakePod { station_id: _ } => {
    //             res = draw_in_station();
    //         }
    //         PersonState::RidingPod { pod_id: _ } => {}
    //         PersonState::JustArrived {
    //             pod_id: _,
    //             station_id: _,
    //         } => {}
    //         PersonState::Transitioning {
    //             station_id: _,
    //             previous_pod_id: _,
    //             time_in_station: _,
    //         } => {
    //             res = draw_in_station();
    //         }
    //         PersonState::InvalidState { reason: _ } => {}
    //     }

    //     res
    // }

    // TODO: move logic of people from main to this function
    pub fn update_state(&mut self, pods_box: &mut PodsBox, network: &mut Network) {
        // println!("person state: {:?}", self.state);
        match &self.state {
            PersonState::ReadyToTakePod { station_id } => {
                // println!("person in ready state");
                // Assign first instead of using directly because:
                // https://github.com/rust-lang/rust/issues/59159
                let station_id_deref = *station_id;
                self.try_to_take_next_pod(pods_box, network, station_id_deref);
            }
            PersonState::RidingPod { pod_id } => {
                // println!("person in riding state");
                let pod_id_deref = *pod_id;
                self.ride_pod(pods_box, pod_id_deref);
            }
            PersonState::JustArrived {
                pod_id,
                station_id: _,
            } => {} // This case is handled in get out if needed
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
            PersonState::InvalidState { reason } => {
                panic!("Person {} is in invalid state. Reason: {}", self.id, reason);
            }
        }
    }

    pub fn get_out_if_needed(&mut self, pods_box: &mut PodsBox, network: &mut Network) {
        match &self.state {
            PersonState::JustArrived {
                pod_id,
                station_id: _,
            } => {
                let pod_id_deref = *pod_id;
                self.decide_on_arrival(pods_box, network, pod_id_deref);
                let maybe_station_id = self.try_get_station_id();
                match maybe_station_id {
                    Some(station_id) => {
                        self.set_real_coordinates(station_id, network);
                    }
                    None => {}
                }
            }
            PersonState::InvalidState { reason } => {
                panic!("Person {} is in invalid state. Reason: {}", self.id, reason);
            }
            _ => {}
        }
    }

    fn try_to_take_next_pod(
        &mut self,
        pods_box: &mut PodsBox,
        network: &mut Network,
        station_id: i32,
    ) {
        let mut rng = rand::thread_rng();
        let maybe_next_station_id = self.path_state.get_next_station_id();
        match maybe_next_station_id {
            Some(next_station_id) => {
                let station = network.try_get_station_by_id(station_id).unwrap();
                let maybe_pod_ids: Option<Vec<i32>> = station.get_pod_ids_in_station_as_vec();
                // println!("maybe_pod_ids: {:?}", maybe_pod_ids);
                match maybe_pod_ids {
                    Some(pod_ids) => {
                        // println!(
                        //     "{}, {}",
                        //     station_id,
                        //     self.path_state.get_current_station_id().unwrap()
                        // );

                        for pod_id in pod_ids {
                            let pod = pods_box.get_pod_by_id(pod_id).unwrap();
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
                let station_ids: Vec<&i32> =
                    network.config.network.coordinates_map.keys().collect();
                let end_ix = rng.gen_range(0..station_ids.len());
                let finish = station_ids[end_ix];
                self.new_path(&network.graph, station_id as u32, *finish as u32);
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

    fn try_to_take_a_pod(
        &mut self,
        pods_box: &mut PodsBox,
        network: &mut Network,
        station_id: i32,
    ) {
        let mut rng = rand::thread_rng();
        let station = network.try_get_station_by_id(station_id).unwrap();
        let maybe_pod_ids: Option<Vec<i32>> = station.get_pod_ids_in_station_as_vec();
        match maybe_pod_ids {
            Some(pod_ids) => {
                let range = rng.gen_range(0..pod_ids.len());
                // println!("the random range: {:?}", range);
                let pod_id_to_take = pod_ids[range];
                let maybe_pod = pods_box.get_pod_by_id(pod_id_to_take);
                match maybe_pod {
                    Some(pod) => {
                        let got_in = pod.try_register_person(self.id);
                        if got_in {
                            // println!("Getting into pod with id: {} now", pod_id_to_take);
                            self.state = self.state.to_riding(pod_id_to_take);
                            let station = network.try_get_station_by_id(station_id).unwrap();
                            station.deregister_person(self.id);
                        } else {
                            // println!(
                            //     "Couldn't get into pod with id: {} - it's full.",
                            //     pod_id_to_take
                            // );
                        }
                    }
                    None => println!("Pod with id: {}, does not exist.", pod_id_to_take),
                }
            }
            None => {} // None => println!("Can't leave the station, no pod here."),
        }
    }

    fn ride_pod(&mut self, pods_box: &mut PodsBox, pod_id: i32) {
        let maybe_pod = pods_box.get_pod_by_id(pod_id);
        match maybe_pod {
            Some(pod) => {
                if pod.is_in_just_arrived_state() {
                    self.state = self.state.to_just_arrived(pod.get_station_id());
                }
            }
            None => panic!("Pod with id: {} does not exist.", pod_id),
        }
    }

    fn decide_on_arrival(&mut self, pods_box: &mut PodsBox, network: &mut Network, pod_id: i32) {
        self.path_state.arrive();
        // println!("self.path_state: {:?}", self.path_state);
        let pod = pods_box.get_pod_by_id(pod_id).unwrap();
        let line_next_station_id = pod.line_state.get_next_station_id();
        let maybe_next_station_id = self.path_state.get_next_station_id();
        match maybe_next_station_id {
            Some(desired_next_station_id) => {
                if line_next_station_id != desired_next_station_id as i32 {
                    self.state = self.state.to_transitioning();
                    let station = network
                        .try_get_station_by_id(pod.line_state.get_station_id())
                        .unwrap();
                    station.register_person(self.id);
                    let pod = pods_box.get_pod_by_id(pod_id).unwrap();
                    pod.deregister_person(&self.id);
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
                    let pod = pods_box.get_pod_by_id(pod_id).unwrap();
                    pod.deregister_person(&self.id);
                }
            }
        }
    }

    fn make_on_arrival_descission(&mut self, pods_box: &mut PodsBox, pod_id: i32) {
        let mut rng = rand::thread_rng();
        let get_out = rng.gen_bool(0.5);
        // println!("get_out: {}", get_out);
        if get_out {
            // println!("Person {} wants to get out", self.id);
            self.state = self.state.to_transitioning();
            let maybe_pod = pods_box.get_pod_by_id(pod_id);
            match maybe_pod {
                Some(pod) => {
                    pod.deregister_person(&self.id);
                }
                None => panic!("Pod with id: {} does not exist.", pod_id),
            }
        } else {
            self.state = self.state.to_riding(pod_id); // pod_id is ignored in this case
        }
    }

    fn set_real_coordinates(&mut self, station_id: i32, network: &Network) {
        // println!("set real coords");
        let station = network.try_get_station_by_id_unmut(station_id).unwrap();
        let coords_station = get_real_coordinates(station.coordinates);
        let mut rng = rand::thread_rng();
        let x_rnd: f32 = rng.gen();
        let y_rnd: f32 = rng.gen();
        let x_shift: f32 = x_rnd * WIDTH_POD * 2.;
        let y_shift: f32 = y_rnd * WIDTH_POD * 2.;

        self.real_coordinates = (coords_station.0 + x_shift, coords_station.1 + y_shift)
    }

    pub fn try_get_station_id(&self) -> Option<i32> {
        self.state.try_get_station_id()
    }

    pub fn try_get_pod_id(&self) -> Option<i32> {
        self.state.try_get_pod_id()
    }
}

// Person State Machine:
//      +-------------------+------> InvalidState <---------+
//      |                   |               ^               |
//      |                   |               |               |
// ReadyToTakePod ---> RidingPod ---> JustArrived ---> Transitioning ---+
//      ^                    ^                |             |    ^      |
//      |                    +----------------+             |    |      |
//      +---------------------------------------------------+    +------+

#[derive(Debug, Clone, PartialEq)]
pub enum PersonState {
    ReadyToTakePod {
        station_id: i32,
    },
    RidingPod {
        pod_id: i32,
    },
    JustArrived {
        pod_id: i32,
        station_id: i32,
    },
    Transitioning {
        station_id: i32,
        previous_pod_id: i32,
        time_in_station: i32,
    },
    InvalidState {
        reason: String,
    },
}

// State Transitions
impl PersonState {
    fn to_riding(&self, pod_id: i32) -> PersonState {
        match self {
            PersonState::ReadyToTakePod { station_id: _ } => {
                PersonState::RidingPod { pod_id: pod_id }
            }
            PersonState::JustArrived {
                pod_id,
                station_id: _,
            } => PersonState::RidingPod { pod_id: *pod_id },
            // _ => panic!("Person can only take a pod from ReadyToTakePod state.")
            _ => PersonState::InvalidState {
                reason: String::from("Person can only take a pod from ReadyToTakePod state."),
            },
        }
    }

    fn to_just_arrived(&self, station_id: i32) -> PersonState {
        match self {
            PersonState::RidingPod { pod_id } => PersonState::JustArrived {
                pod_id: *pod_id,
                station_id: station_id,
            },
            _ => PersonState::InvalidState {
                reason: String::from("Person can only arrive if in RidingPod state."),
            },
        }
    }

    fn to_transitioning(&self) -> PersonState {
        match self {
            PersonState::JustArrived { pod_id, station_id } => PersonState::Transitioning {
                previous_pod_id: *pod_id,
                station_id: *station_id,
                time_in_station: 0,
            },
            _ => PersonState::InvalidState {
                reason: String::from("Person can only transition if in JustArrived state."),
            },
        }
    }

    fn to_ready(&self) -> PersonState {
        match self {
            PersonState::Transitioning {
                previous_pod_id: _,
                station_id,
                time_in_station: _,
            } => PersonState::ReadyToTakePod {
                station_id: *station_id,
            },
            _ => PersonState::InvalidState {
                reason: String::from(
                    "Person can only get ready to take a pod if in Transitioning state.",
                ),
            },
        }
    }

    fn wait_a_sec(&self) -> PersonState {
        match self {
            PersonState::Transitioning {
                previous_pod_id,
                station_id,
                time_in_station,
            } => PersonState::Transitioning {
                previous_pod_id: *previous_pod_id,
                station_id: *station_id,
                time_in_station: time_in_station + 1,
            },
            _ => PersonState::InvalidState {
                reason: String::from("Person can only wait if in Transitioning state"),
            },
        }
    }

    fn try_get_station_id(&self) -> Option<i32> {
        match self {
            PersonState::ReadyToTakePod { station_id } => Some(*station_id),
            PersonState::JustArrived {
                pod_id: _,
                station_id,
            } => Some(*station_id),
            PersonState::Transitioning {
                station_id,
                previous_pod_id: _,
                time_in_station: _,
            } => Some(*station_id),
            _ => None,
        }
    }

    fn try_get_pod_id(&self) -> Option<i32> {
        match self {
            PersonState::RidingPod { pod_id } => Some(*pod_id),
            PersonState::JustArrived {
                pod_id,
                station_id: _,
            } => Some(*pod_id),
            _ => None,
        }
    }
}
