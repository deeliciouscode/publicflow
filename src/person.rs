use crate::action::{Actions, GetAction, SetAction};
use crate::config::Config;
use crate::helper::{get_random_station_id, get_screen_coordinates};
use crate::network::Network;
use crate::pathstate::PathState;
use crate::pod::{Pod, PodsBox};
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

    pub fn try_get_person_by_id_unmut(&self, id: i32) -> Option<&Person> {
        for person in &self.people {
            if person.id == id {
                return Some(person);
            }
        }
        None
    }

    pub fn update(
        &mut self,
        pods_box: &mut PodsBox,
        network: &mut Network,
        set_actions: &Vec<SetAction>,
        config: &Config,
    ) {
        for action in set_actions {
            match action {
                // TODO: differentiate between follow and not
                SetAction::ShowPerson { id, follow } => {
                    for person in &mut self.people {
                        if person.id == *id {
                            if *follow {
                                person.visualize = true;
                            } else {
                                person.visualize = true;
                            }
                        }
                    }
                }
                SetAction::HidePerson { id } => {
                    for person in &mut self.people {
                        if person.id == *id {
                            person.visualize = false;
                        }
                    }
                }
                SetAction::RoutePerson {
                    id,
                    station_id,
                    random_station,
                } => {
                    for person in &mut self.people {
                        if person.id == *id {
                            person.action_on_arrival = Some(action.clone())
                        }
                    }
                }
                _ => {}
            }
        }
        for person in &mut self.people {
            person.get_out_if_needed(pods_box, network, config);
        }
        for person in &mut self.people {
            person.update(pods_box, network, config);
        }
    }

    pub fn draw(&self, ctx: &mut Context, network: &Network, config: &Config) {
        for person in &self.people {
            if person.visualize {
                let _res = person.draw(ctx, network);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Person {
    pub id: i32,
    visualize: bool,
    transition_time: i32,
    pub real_coordinates: (f32, f32),
    state: PersonState,
    pub path_state: PathState,
    action_on_arrival: Option<SetAction>,
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
            transition_time: transition_time,
            real_coordinates: (0., 0.),
            state: PersonState::Transitioning {
                station_id: start,
                previous_pod_id: -1,
                time_in_station: transition_time - 1,
            },
            path_state: PathState::new(&network.graph, start as u32, finish as u32, network),
            action_on_arrival: None,
        };
        person.set_coordinates_of_station(
            person.path_state.try_get_current_station_id().unwrap() as i32,
            network,
            config,
        );
        // println!("{:?}", person.path_state);
        person
    }

    pub fn new_path(
        &mut self,
        graph: &UnGraph<u32, u32>,
        start: u32,
        finish: u32,
        network: &Network,
    ) {
        self.path_state = PathState::new(graph, start, finish, network);
        // println!("{:?}", self.path_state);
    }

    fn draw(&self, ctx: &mut Context, network: &Network) -> GameResult<()> {
        let color = [0.0, 1.0, 1.0, 1.0].into();

        let mut res: GameResult<()> = std::result::Result::Ok(());

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

        res = graphics::draw(ctx, &circle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));

        match res {
            Err(err) => panic!("Error 3: {}", err),
            Ok(m) => {
                // println!("No error at 3: {:?}", m);
                return res;
            }
        }
    }

    pub fn update(&mut self, pods_box: &mut PodsBox, network: &mut Network, config: &Config) {
        // println!("person state: {:?}", self.state);
        match &self.state {
            PersonState::ReadyToTakePod { station_id } => {
                // println!("person in ready state");
                // Assign first instead of using directly because:
                // https://github.com/rust-lang/rust/issues/59159
                let station_id_deref = *station_id;
                self.try_to_take_next_pod(pods_box, network, station_id_deref, config);
            }
            PersonState::RidingPod { pod_id } => {
                // println!("person in riding state");
                let pod_id_deref = *pod_id;
                self.ride_pod(pods_box, pod_id_deref);
            }
            PersonState::JustArrived {
                pod_id: _,
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

    fn process_action_on_arrival(
        &mut self,
        current_station_id: u32,
        network: &mut Network,
        config: &Config,
    ) {
        match &self.action_on_arrival {
            Some(action) => match action {
                SetAction::RoutePerson {
                    id: _,
                    station_id,
                    random_station,
                } => {
                    if *random_station {
                        let random_station_id = get_random_station_id(network, config);
                        self.new_path(
                            &network.graph,
                            current_station_id,
                            random_station_id,
                            network,
                        )
                    } else {
                        let station_id_finish = *station_id;
                        self.new_path(
                            &network.graph,
                            current_station_id,
                            station_id_finish,
                            network,
                        )
                    }
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
        config: &Config,
    ) {
        let mut rng = rand::thread_rng();
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
                let finish = get_random_station_id(network, config);
                self.new_path(&network.graph, station_id as u32, finish, network);
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
        let maybe_pod_ids: Option<Vec<i32>> = station.try_get_pod_ids_in_station_as_vec();
        match maybe_pod_ids {
            Some(pod_ids) => {
                let range = rng.gen_range(0..pod_ids.len());
                // println!("the random range: {:?}", range);
                let pod_id_to_take = pod_ids[range];
                let maybe_pod = pods_box.try_get_pod_by_id_mut(pod_id_to_take);
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
        let maybe_pod = pods_box.try_get_pod_by_id_mut(pod_id);
        match maybe_pod {
            Some(pod) => {
                if self.visualize {
                    self.set_coordinates_of_pod(pod)
                }
                if pod.is_in_just_arrived_state() {
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
                    || self.action_on_arrival.is_some()
                {
                    self.state = self.state.to_transitioning();
                    let station = network
                        .try_get_station_by_id(pod.line_state.get_station_id())
                        .unwrap();
                    station.register_person(self.id);
                    let pod = pods_box.try_get_pod_by_id_mut(pod_id).unwrap();
                    pod.deregister_person(&self.id);
                    self.process_action_on_arrival(station.id as u32, network, config);
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
                    self.process_action_on_arrival(station.id as u32, network, config);
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
            let maybe_pod = pods_box.try_get_pod_by_id_mut(pod_id);
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
