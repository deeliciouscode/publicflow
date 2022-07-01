use crate::config::{
    LENGTH_POD, MAX_XY, OFFSET, POD_CAPACITY, RADIUS_POD, SCREEN_SIZE, SIDELEN_STATION, WIDTH_LINE,
    WIDTH_POD,
};
use crate::helper::get_real_coordinates;
use crate::line::LineState;
use crate::network::Network;
use ggez::graphics::{Font, Rect, Text};
use ggez::{graphics, Context, GameResult};
use std::collections::HashSet;

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
    pub fn get_pod_by_id(&mut self, pod_id: i32) -> Option<&mut Pod> {
        for pod in &mut self.pods {
            if pod.id == pod_id {
                return Some(pod);
            }
        }
        return None;
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

    pub fn draw(&self, ctx: &mut Context, network: &Network) {
        for pod in &self.pods {
            let _res = pod.draw(ctx, network);
        }
    }

    pub fn update(&mut self, network: &mut Network) {
        for pod in &mut self.pods {
            pod.update_state(network)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Pod {
    pub id: i32,
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
            in_station_for: in_station_for,
            capacity: capacity,
            people_in_pod: HashSet::new(),
            line_state: line_state,
            state: PodState::InStation {
                station_id: station_id,
                time_in_station: 0,
                coords: (0., 0.),
            },
        }
    }

    fn draw(&self, ctx: &mut Context, network: &Network) -> GameResult<()> {
        let red = self.people_in_pod.len() as f32 / POD_CAPACITY as f32;
        let green = 1. - red;

        // let test = [red - 0.2, green - 0.2, 0., 1.0, 2.0];
        // let color = test.into();

        let color = self.get_rgba().into();

        let mut res: GameResult<()> = std::result::Result::Ok(());

        let x_shift: f32;
        let y_shift: f32;

        let (real_x, real_y) = self.state.try_get_coords().unwrap();

        // if self.id == 7 {
        //     println!("from: {}, to: {}, coords: {:?}", station_id_from, station_id_to, (x,y))
        // }

        let circle = graphics::Mesh::new_circle(
            ctx,
            // graphics::DrawMode::stroke(2.),
            graphics::DrawMode::fill(),
            ggez::mint::Point2 {
                x: real_x,
                y: real_y,
            },
            RADIUS_POD,
            1.,
            color,
        )?;

        res = graphics::draw(ctx, &circle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));

        match res {
            Err(err) => panic!("Error 3: {}", err),
            Ok(m) => {
                // println!("No error at 3: {:?}", m);
                return res;
            }
        }

        // let station_rect = Rect {
        //     x: real_x,
        //     y: real_y,
        //     w: SIDELEN_POD,
        //     h: SIDELEN_POD,
        // };
        // let rectangle =
        //     graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), station_rect, color)?;

        // res = graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));

        // let text = Text::new(String::from("1"));
        // let id_text = Text::new(String::from(self.id.to_string()));
        // let people_inside_text = Text::new(String::from(self.people_in_pod.len().to_string()));
        // res = graphics::draw(
        //     ctx,
        //     &id_text,
        //     (ggez::mint::Point2 {
        //         x: real_x,
        //         y: real_y,
        //     },),
        // );

        // let people_inside_text_x = real_x + LENGTH_POD / 2. - Font::DEFAULT_FONT_SCALE / 2.;
        // let people_inside_text_y = real_y + LENGTH_POD / 2. - Font::DEFAULT_FONT_SCALE / 2.;

        // res = graphics::draw(
        //     ctx,
        //     &people_inside_text,
        //     (ggez::mint::Point2 {
        //         x: people_inside_text_x,
        //         y: people_inside_text_y,
        //     },),
        // );
    }

    fn get_rgba(&self) -> [f32; 4] {
        match self.line_state.line.name.as_str() {
            "u1" => return [0.0, 1.0, 0.0, 1.0],
            "u2" => return [1.0, 0.0, 0.0, 1.0],
            "u3" => return [0.99, 0.63, 0.01, 1.0],
            "u4" => return [0.13, 0.74, 0.69, 1.0],
            "u5" => return [0.82, 0.73, 0.06, 1.0],
            "u6" => return [0.0, 0.0, 1.0, 1.0],
            "u7" => return [0.77, 0.75, 0.43, 1.0],
            "u8" => return [0.68, 0.67, 0.55, 1.0],
            any => panic!("The line: {} is not defined.", any),
        };
    }

    // TODO: remove unused stuff
    pub fn update_state(&mut self, network: &mut Network) {
        match &self.state {
            PodState::BetweenStations {
                station_id_from: _,
                station_id_to: _,
                time_to_next_station,
                coords: _,
            } => {
                // println!("Pod in BetweenStations State");
                if *time_to_next_station > 0 {
                    self.state = self.state.drive_a_sec(self, network);
                } else {
                    self.arrive_in_station(network);
                }
            }
            PodState::JustArrived {
                station_id: _,
                coords: _,
            } => {
                // println!("Pod in JustArrived State");
                self.state = self.state.to_in_station();
            }
            PodState::InStation {
                station_id: _,
                time_in_station,
                coords: _,
            } => {
                // println!("Pod in InStation state");
                if self.in_station_for > *time_in_station {
                    self.state = self.state.wait_a_sec();
                } else {
                    self.depart_from_station(network);
                }
            }
            PodState::InvalidState { reason } => {
                panic!("Pod {} is in invalid state. Reason: {}", self.id, reason)
            }
        }
    }

    fn arrive_in_station(&mut self, net: &mut Network) {
        self.line_state.update_line_ix();
        self.line_state.set_next_station_ix();
        self.state = self.state.to_just_arrived();
        let arrived_in_id = self.state.get_station_id();
        let maybe_station = net.try_get_station_by_id(arrived_in_id);
        match maybe_station {
            Some(station) => station.register_pod(self.id),
            None => panic!("There is no station with id: {}", arrived_in_id),
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
        let maybe_connection = self.line_state.get_connection(current, next);
        match maybe_connection {
            Some(connection) => {
                self.state = self.state.to_between_stations(next, connection.travel_time);
            }
            None => panic!("There is no connection between: {} and {}", current, next),
        }

        let maybe_station = net.try_get_station_by_id(current);
        match maybe_station {
            Some(station) => station.deregister_pod(self.id),
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

    pub fn deregister_person(&mut self, person_id: &i32) {
        self.people_in_pod.remove(person_id);
    }

    pub fn is_in_just_arrived_state(&self) -> bool {
        match self.state {
            PodState::JustArrived {
                station_id: _,
                coords: _,
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
        coords: (f32, f32),
    },
    JustArrived {
        station_id: i32,
        coords: (f32, f32),
    },
    InStation {
        station_id: i32,
        time_in_station: i32,
        coords: (f32, f32),
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
                coords,
            } => {
                PodState::BetweenStations {
                    station_id_from: *station_id,
                    station_id_to: to_pod_id,
                    time_to_next_station: time_to_next_station,
                    coords: *coords,
                } // TODO to
            }
            _ => PodState::InvalidState {
                reason: String::from("Pod can only appart from InStation state."),
            },
        }
    }

    fn to_just_arrived(&self) -> PodState {
        match self {
            PodState::BetweenStations {
                station_id_from: _,
                station_id_to,
                time_to_next_station: _,
                coords,
            } => PodState::JustArrived {
                station_id: *station_id_to,
                coords: *coords,
            },
            _ => PodState::InvalidState {
                reason: String::from("Pod can only arrive if in BetweenStations state."),
            },
        }
    }

    fn to_in_station(&self) -> PodState {
        match self {
            PodState::JustArrived { station_id, coords } => PodState::InStation {
                station_id: *station_id,
                time_in_station: 0,
                coords: *coords,
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
                coords,
            } => PodState::InStation {
                station_id: *station_id,
                time_in_station: time_in_station + 1,
                coords: *coords,
            },
            _ => PodState::InvalidState {
                reason: String::from("Pod can only wait if in InStation state"),
            },
        }
    }

    fn drive_a_sec(&self, pod: &Pod, network: &Network) -> PodState {
        match self {
            PodState::BetweenStations {
                station_id_from,
                station_id_to,
                time_to_next_station,
                coords: _,
            } => {
                let travel_time = pod
                    .line_state
                    .get_connection(*station_id_from, *station_id_to)
                    .unwrap()
                    .travel_time;

                let station_from = network
                    .try_get_station_by_id_unmut(*station_id_from)
                    .unwrap();
                let station_to = network.try_get_station_by_id_unmut(*station_id_to).unwrap();

                let coords_from = get_real_coordinates(station_from.coordinates);
                let coords_to = get_real_coordinates(station_to.coordinates);
                let x = coords_from.0
                    + (coords_to.0 - coords_from.0)
                        * ((travel_time as f32 - *time_to_next_station as f32)
                            / travel_time as f32);

                let y = coords_from.1
                    + (coords_to.1 - coords_from.1)
                        * ((travel_time as f32 - *time_to_next_station as f32)
                            / travel_time as f32);

                let real_x = x;
                let real_y = y;

                PodState::BetweenStations {
                    station_id_from: *station_id_from,
                    station_id_to: *station_id_to,
                    time_to_next_station: time_to_next_station - 1,
                    coords: (real_x, real_y),
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
                coords: _
            } => *station_id,
            PodState::InStation {
                time_in_station: _,
                station_id,
                coords: _
            } => *station_id,
            _ => panic!("Can only get id of station in which pod arrives if in JustArrived or InStation state")
        }
    }

    fn try_get_station_id(&self) -> Option<i32> {
        match self {
            PodState::JustArrived {
                station_id,
                coords: _,
            } => Some(*station_id),
            PodState::InStation {
                time_in_station: _,
                station_id,
                coords: _,
            } => Some(*station_id),
            _ => None,
        }
    }

    fn try_get_coords(&self) -> Option<(f32, f32)> {
        match self {
            PodState::JustArrived {
                station_id: _,
                coords,
            } => Some(*coords),
            PodState::InStation {
                time_in_station: _,
                station_id: _,
                coords,
            } => Some(*coords),
            PodState::BetweenStations {
                station_id_from: _,
                station_id_to: _,
                time_to_next_station: _,
                coords,
            } => Some(*coords),
            _ => None,
        }
    }
}
