use crate::action::{Actions, GetAction, SetAction};
use crate::config::Config;
use crate::enums::LineName;
use crate::helper::get_screen_coordinates;
use crate::line::LineState;
use crate::network::Network;
use ggez::{graphics, Context, GameResult};
use std::collections::HashSet;
// use rayon::prelude::*; // For Parralelism

#[derive(Clone, Debug)]
pub struct PodsBox {
    pub pods: Vec<Pod>,
}

impl PodsBox {
    pub fn get_available_pods(&self, station_id: i32) -> Vec<&Pod> {
        let mut pods: Vec<&Pod> = vec![];
        for pod in &self.pods {
            if pod.line_state.get_station_id() == station_id {
                pods.push(pod)
            }
        }
        return pods;
    }
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

    pub fn print_state(&self) {
        for pod in &self.pods {
            let maybe_station_id = pod.try_get_station_id();
            let station_id;
            match maybe_station_id {
                Some(_station_id) => station_id = _station_id.to_string(),
                None => station_id = String::from("None"),
            }
            println!(
                "Pod: {} | Station: {} | Passengers: {:?} | State: {:?}",
                pod.id, station_id, pod.people_in_pod, pod.state
            )
        }
    }

    pub fn draw(&self, ctx: &mut Context, network: &Network, config: &Config) {
        for pod in &self.pods {
            let _res = pod.draw(ctx, network, config);
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

#[derive(Clone, Debug)]
pub struct Pod {
    pub id: i32,
    visualize: bool,
    pub in_station_for: i32,
    pub capacity: i32,
    pub people_in_pod: HashSet<i32>,
    pub line_state: LineState,
    state: PodState,
}

impl Pod {
    pub fn new(id: i32, in_station_for: i32, capacity: i32, line_state: LineState) -> Self {
        let station_id = line_state.get_station_id();
        Pod {
            id: id,
            visualize: false,
            in_station_for: in_station_for,
            capacity: capacity,
            people_in_pod: HashSet::new(),
            line_state: line_state,
            state: PodState::InStation {
                station_id: station_id,
                time_in_station: 0,
                coordinates: (0., 0.),
            },
        }
    }

    fn draw(&self, ctx: &mut Context, network: &Network, config: &Config) -> GameResult<()> {
        // let red = self.people_in_pod.len() as f32 / POD_CAPACITY as f32;
        // let green = 1. - red;

        let color = self.get_rgba().into();

        let mut res: GameResult<()> = std::result::Result::Ok(());

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

        res = graphics::draw(ctx, &circle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));

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

            res = graphics::draw(ctx, &circle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
        }

        match res {
            Err(err) => panic!("Error 3: {}", err),
            Ok(m) => {
                // println!("No error at 3: {:?}", m);
                return res;
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

    // TODO: remove unused stuff
    pub fn update(&mut self, network: &mut Network, config: &Config) {
        match &self.state {
            PodState::BetweenStations {
                station_id_from: _,
                station_id_to: _,
                time_to_next_station,
                coordinates: _,
            } => {
                // println!("Pod in BetweenStations State");
                if *time_to_next_station > 0 {
                    self.state = self.state.drive_a_sec(self, network, config);
                } else {
                    self.arrive_in_station(network);
                }
            }
            PodState::JustArrived {
                station_id: _,
                coordinates: _,
            } => {
                // println!("Pod in JustArrived State");
                self.state = self.state.to_in_station();
            }
            PodState::InStation {
                station_id: _,
                time_in_station,
                coordinates: _,
            } => {
                // println!("Pod in InStation state");
                if self.in_station_for > *time_in_station {
                    self.state = self.state.wait_a_sec();
                } else {
                    self.depart_from_station(network);
                }
            }
            PodState::InQueue {
                station_id,
                coordinates,
            } => {}
            PodState::InvalidState { reason } => {
                panic!("Pod {} is in invalid state. Reason: {}", self.id, reason)
            }
        }
    }

    fn arrive_in_station(&mut self, net: &mut Network) {
        self.line_state.update_line_ix();
        self.line_state.set_next_station_ix();
        let station_id_to = self.state.get_station_id_to();
        let maybe_platform = net.try_get_platform_by_station_id_and_line_name(
            station_id_to,
            &self.line_state.line.name,
        );

        match maybe_platform {
            Some(platform) => {
                if platform.is_operational() {
                    self.state = self.state.to_just_arrived();
                    platform.register_pod(self.id);
                } else if platform.is_queuable() {
                    self.state = self.state.to_in_queue();
                    platform.queue_pod(self.id);
                }
                // println!("Platform: {:?}", platform)
            }
            None => {
                println!("Got no platform back")
            }
        }

        // let arrived_in_id = self.state.get_station_id();
        // match maybe_station {
        //     Some(station) => station.register_pod(self.id),
        //     None => panic!("There is no station with id: {}", arrived_in_id),
        // }
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
                    self.state = self.state.to_between_stations(next, connection.travel_time);
                }
            }
            None => panic!("There is no connection between: {} and {}", current, next),
        }

        let maybe_platform =
            net.try_get_platform_by_station_id_and_line_name(current, &self.line_state.line.name);

        match maybe_platform {
            Some(platform) => platform.deregister_pod(self.id),
            None => panic!("There is no station with id: {}", current),
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
        let coordinates = self.state.try_get_coordinates().unwrap();
        return coordinates;
    }

    pub fn deregister_person(&mut self, person_id: &i32) {
        self.people_in_pod.remove(person_id);
    }

    pub fn is_in_just_arrived_state(&self) -> bool {
        match self.state {
            PodState::JustArrived {
                station_id: _,
                coordinates: _,
            } => true,
            _ => false,
        }
    }

    pub fn get_station_id(&self) -> i32 {
        self.state.get_station_id()
    }

    pub fn try_get_station_id(&self) -> Option<i32> {
        self.state.try_get_station_id()
    }
}

// Pod State Machine:
//      +-------------------+------> InvalidState
//      |                   |                 ^
//      |                   |                 |
// BetweenStations ---> JustArrived ---> InStation <--+
//      ^    ^   |                            |  |    |
//      |    +---+                            |  +----+
//      +-------------------------------------+

// Can add defects and stuff like that as a state
#[derive(Debug, Clone, PartialEq)]
pub enum PodState {
    BetweenStations {
        station_id_from: i32,
        station_id_to: i32,
        time_to_next_station: i32,
        coordinates: (f32, f32),
    },
    InQueue {
        station_id: i32,
        coordinates: (f32, f32),
    },
    JustArrived {
        station_id: i32,
        coordinates: (f32, f32),
    },
    InStation {
        station_id: i32,
        time_in_station: i32,
        coordinates: (f32, f32),
    },
    InvalidState {
        reason: String,
    },
}

// State Transitions
impl PodState {
    fn to_between_stations(&self, to_pod_id: i32, time_to_next_station: i32) -> PodState {
        match self {
            PodState::InStation {
                station_id,
                time_in_station: _,
                coordinates,
            } => {
                PodState::BetweenStations {
                    station_id_from: *station_id,
                    station_id_to: to_pod_id,
                    time_to_next_station: time_to_next_station,
                    coordinates: *coordinates,
                } // TODO to
            }
            _ => PodState::InvalidState {
                reason: String::from("Pod can only appart from InStation state."),
            },
        }
    }

    fn to_in_queue(&self) -> PodState {
        match self {
            PodState::BetweenStations {
                station_id_from: _,
                station_id_to,
                time_to_next_station: _,
                coordinates,
            } => PodState::InQueue {
                station_id: *station_id_to,
                coordinates: *coordinates,
            },
            _ => PodState::InvalidState {
                reason: String::from("Pod can only arrive if in BetweenStations state."),
            },
        }
    }

    fn to_just_arrived(&self) -> PodState {
        match self {
            PodState::BetweenStations {
                station_id_from: _,
                station_id_to,
                time_to_next_station: _,
                coordinates,
            } => PodState::JustArrived {
                station_id: *station_id_to,
                coordinates: *coordinates,
            },
            _ => PodState::InvalidState {
                reason: String::from("Pod can only arrive if in BetweenStations state."),
            },
        }
    }

    fn to_in_station(&self) -> PodState {
        match self {
            PodState::JustArrived {
                station_id,
                coordinates,
            } => PodState::InStation {
                station_id: *station_id,
                time_in_station: 0,
                coordinates: *coordinates,
            },
            _ => PodState::InvalidState {
                reason: String::from("Pod can only get to InStation if in JustArrived state."),
            },
        }
    }

    fn wait_a_sec(&self) -> PodState {
        match self {
            PodState::InStation {
                station_id,
                time_in_station,
                coordinates,
            } => PodState::InStation {
                station_id: *station_id,
                time_in_station: time_in_station + 1,
                coordinates: *coordinates,
            },
            _ => PodState::InvalidState {
                reason: String::from("Pod can only wait if in InStation state"),
            },
        }
    }

    fn drive_a_sec(&self, pod: &Pod, network: &Network, config: &Config) -> PodState {
        match self {
            PodState::BetweenStations {
                station_id_from,
                station_id_to,
                time_to_next_station,
                coordinates: _,
            } => {
                let travel_time = pod
                    .line_state
                    .try_get_connection(*station_id_from, *station_id_to)
                    .unwrap()
                    .travel_time;

                let station_from = network
                    .try_get_station_by_id_unmut(*station_id_from)
                    .unwrap();
                let station_to = network.try_get_station_by_id_unmut(*station_id_to).unwrap();

                let coordinates_from = get_screen_coordinates(station_from.coordinates, config);
                let coordinates_to = get_screen_coordinates(station_to.coordinates, config);
                let x = coordinates_from.0
                    + (coordinates_to.0 - coordinates_from.0)
                        * ((travel_time as f32 - *time_to_next_station as f32)
                            / travel_time as f32);

                let y = coordinates_from.1
                    + (coordinates_to.1 - coordinates_from.1)
                        * ((travel_time as f32 - *time_to_next_station as f32)
                            / travel_time as f32);

                let real_x = x;
                let real_y = y;

                PodState::BetweenStations {
                    station_id_from: *station_id_from,
                    station_id_to: *station_id_to,
                    time_to_next_station: time_to_next_station - 1,
                    coordinates: (real_x, real_y),
                }
            }
            _ => PodState::InvalidState {
                reason: String::from("Pod can only drive if in BetweenStations state"),
            },
        }
    }

    fn get_station_id(&self) -> i32 {
        match self {
            PodState::JustArrived {
                station_id,
                coordinates: _
            } => *station_id,
            PodState::InStation {
                time_in_station: _,
                station_id,
                coordinates: _
            } => *station_id,
            _ => panic!("Can only get id of station in which pod arrives if in JustArrived or InStation state")
        }
    }

    fn try_get_station_id(&self) -> Option<i32> {
        match self {
            PodState::JustArrived {
                station_id,
                coordinates: _,
            } => Some(*station_id),
            PodState::InStation {
                time_in_station: _,
                station_id,
                coordinates: _,
            } => Some(*station_id),
            _ => None,
        }
    }

    fn get_station_id_to(&self) -> i32 {
        match self {
            PodState::BetweenStations {
                station_id_from: _,
                station_id_to,
                time_to_next_station: _,
                coordinates: _
            } => *station_id_to,
            _ => panic!("Can only get id of station that the pod is driving towards if in BetweenStations state")
        }
    }

    fn try_get_coordinates(&self) -> Option<(f32, f32)> {
        match self {
            PodState::JustArrived {
                station_id: _,
                coordinates,
            } => Some(*coordinates),
            PodState::InQueue {
                station_id: _,
                coordinates,
            } => Some(*coordinates),
            PodState::InStation {
                time_in_station: _,
                station_id: _,
                coordinates,
            } => Some(*coordinates),
            PodState::BetweenStations {
                station_id_from: _,
                station_id_to: _,
                time_to_next_station: _,
                coordinates,
            } => Some(*coordinates),
            _ => None,
        }
    }
}
