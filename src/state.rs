use crate::action::{GetAction, SetAction};
use crate::cli::handle_queries;
use crate::config::{Config, DESIRED_FPS, POD_CAPACITY, TRANSITION_TIME};
use crate::helper::format_seconds;
use crate::line::{Line, LineState};
use crate::network::Network;
use crate::person::{PeopleBox, Person};
use crate::pod::{Pod, PodsBox};
use crate::station::Station;
use ggez::event::{EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, Color, DrawParam, Font, PxScale, Text};
use ggez::{timer, Context, GameResult};
use rand::Rng;
use std::collections::HashSet;
use std::sync::mpsc;

#[derive(Debug)]
pub struct State {
    pub network: Network,
    pub pods_box: PodsBox,
    pub people_box: PeopleBox,
    pub time_passed: u32,
    pub config: Config,
    pub on_pause: bool,
    pub last_mouse_left: (f32, f32),
    rx: mpsc::Receiver<String>,
}

impl State {
    pub fn print_state(&self) {
        self.network.print_state();
        self.pods_box.print_state();
        self.people_box.print_state();
    }

    pub fn update(&mut self, set_actions: Vec<SetAction>) {
        if set_actions.len() != 0 {
            println!("set_actions: {:?}", set_actions);
        }

        self.network.update();
        self.pods_box.update(&mut self.network);
        self.people_box
            .update(&mut self.pods_box, &mut self.network);
    }

    // TODO:PRIO make this shit appear
    pub fn draw(&mut self, ctx: &mut Context) {
        // println!("Draw State");
        let mut time_passed = Text::new(String::from(format!(
            "Time passed: {}",
            format_seconds(self.time_passed)
        )));
        time_passed.set_font(Font::default(), PxScale::from(40.));
        // println!("Time Passed: {}", self.time_passed);
        // let color = [0.2, 0.2, 0.2, 1.0].into();
        let draw_param = DrawParam::new().offset([-10., -10.]).color(Color::BLACK);
        let res = graphics::draw(ctx, &time_passed, draw_param);
        match res {
            Ok(ok) => {
                // println!("{:?}", ok)
            }
            Err(err) => panic!("{:?}", err),
        }

        if self.on_pause {
            let maybe_station = self.network.try_retrieve_station(self.last_mouse_left);
            match maybe_station {
                Some(station) => {
                    let mut name = Text::new(String::from(format!("Name: {}", station.name)));
                    name.set_font(Font::default(), PxScale::from(18.));
                    let mut count = Text::new(String::from(format!(
                        "People Count: {}",
                        station.people_in_station.len()
                    )));
                    count.set_font(Font::default(), PxScale::from(18.));
                    let mut pods =
                        Text::new(String::from(format!("Pods: {:?}", station.pods_in_station)));
                    pods.set_font(Font::default(), PxScale::from(18.));

                    // println!("Time Passed: {}", self.time_passed);
                    // let color = [0.2, 0.2, 0.2, 1.0].into();
                    let draw_param_name = DrawParam::new()
                        .offset([-self.last_mouse_left.0 - 10., -self.last_mouse_left.1 - 10.])
                        .color(Color::BLACK);
                    let res = graphics::draw(ctx, &name, draw_param_name);
                    let draw_param_count = DrawParam::new()
                        .offset([-self.last_mouse_left.0 - 10., -self.last_mouse_left.1 - 30.])
                        .color(Color::BLACK);
                    let res = graphics::draw(ctx, &count, draw_param_count);
                    let draw_param_pods = DrawParam::new()
                        .offset([-self.last_mouse_left.0 - 10., -self.last_mouse_left.1 - 50.])
                        .color(Color::BLACK);
                    let res = graphics::draw(ctx, &pods, draw_param_pods);

                    // println!("station: {:?}", station);
                }
                None => {
                    let maybe_pod = self.pods_box.try_retrieve_pod(self.last_mouse_left);
                    match maybe_pod {
                        Some(pod) => {
                            let mut id = Text::new(String::from(format!("ID: {}", pod.id)));
                            id.set_font(Font::default(), PxScale::from(18.));
                            let mut count = Text::new(String::from(format!(
                                "Passengers: {}",
                                pod.people_in_pod.len()
                            )));
                            count.set_font(Font::default(), PxScale::from(18.));
                            let mut capacity =
                                Text::new(String::from(format!("Capacity: {}", pod.capacity)));
                            capacity.set_font(Font::default(), PxScale::from(18.));

                            // println!("Time Passed: {}", self.time_passed);
                            // let color = [0.2, 0.2, 0.2, 1.0].into();
                            let draw_param_name = DrawParam::new()
                                .offset([
                                    -self.last_mouse_left.0 - 10.,
                                    -self.last_mouse_left.1 - 10.,
                                ])
                                .color(Color::BLACK);
                            let res = graphics::draw(ctx, &id, draw_param_name);
                            let draw_param_count = DrawParam::new()
                                .offset([
                                    -self.last_mouse_left.0 - 10.,
                                    -self.last_mouse_left.1 - 30.,
                                ])
                                .color(Color::BLACK);
                            let res = graphics::draw(ctx, &count, draw_param_count);
                            let draw_param_count = DrawParam::new()
                                .offset([
                                    -self.last_mouse_left.0 - 10.,
                                    -self.last_mouse_left.1 - 50.,
                                ])
                                .color(Color::BLACK);
                            let res = graphics::draw(ctx, &capacity, draw_param_count);
                            // println!("pod: {:?}", pod);
                        }
                        None => {}
                    }
                }
            }
        }
    }

    // TODO:PRIO: implement spwaning of pods at a given rate till there are enough
    // as a next step spawn / divert pods dynamically

    pub fn new(config: &Config, rx: mpsc::Receiver<String>) -> Self {
        let mut rng = rand::thread_rng();
        let mut stations: Vec<Station> = vec![];
        for abstract_station in config.network.coordinates_map.iter() {
            let station_id = abstract_station.0;
            let (name, city, (lat, lon)) = abstract_station.1;

            stations.push(Station {
                id: *station_id,
                name: name.clone(),
                city: city.clone(),
                since_last_pod: 0,
                edges_to: HashSet::new(), // config.network.edge_map.get(&station_id).unwrap().clone(),
                pods_in_station: HashSet::from([]), // The pods will register themselves later
                people_in_station: HashSet::from([]),
                coordinates: (*lat as f32, *lon as f32),
                config: config.clone(),
            })
        }

        let mut network = Network::new(stations, config);

        // let mut stations_occupied: Vec<i32> = vec![];
        let calc_line_state = |pod_id: &i32| -> LineState {
            let mut rng = rand::thread_rng();
            let mut n_stations_skipped = 0;
            // default, needed so Line can never be nothing
            let mut line: Line = Line {
                name: "placeholder".to_string(),
                stations: vec![],
                distances: vec![],
                circular: true,
                connections: vec![],
            };
            let mut line_ix: i32 = -1;
            // let mut station_id: i32 = -1;
            let mut direction: i32 = 1;

            for lineref in &config.network.lines {
                // println!("{}, {}", pod_id, n_stations_skipped);
                if *pod_id > n_stations_skipped + (lineref.stations.len() as i32 - 1) {
                    n_stations_skipped += lineref.stations.len() as i32;
                    continue;
                }

                line_ix = pod_id - n_stations_skipped;
                line = lineref.clone();
                // station_id = lineref.stations[line_ix as usize];
                direction = if rng.gen_bool(0.5) { 1 } else { -1 };
                break;
            }

            if line.stations.is_empty() {
                panic!("Something went wrong, stations should not be empty. Probably the number of pods does not match the expected number.")
            }

            let mut line_state = LineState {
                line: line,
                line_ix: line_ix,
                next_ix: -1,
                direction: direction,
            };

            line_state.set_next_station_ix();

            // println!("-------------> {:?}", line_state);

            return line_state;
        };

        let station_ids: Vec<&i32> = config.network.coordinates_map.keys().collect();
        // println!("{:?}", station_ids);

        let mut people: Vec<Person> = vec![];
        for person_id in 0..config.people.n_people {
            let start_ix = rng.gen_range(0..station_ids.len());
            let end_ix = rng.gen_range(0..station_ids.len());
            let start = station_ids[start_ix];
            let end = station_ids[end_ix];
            people.push(Person::new(
                person_id,
                TRANSITION_TIME,
                &network,
                *start,
                *end,
            ));
        }

        for person in &people {
            let station = network
                .try_get_station_by_id(person.path_state.get_current_station_id().unwrap() as i32)
                .unwrap();
            station.register_person(person.id);
        }

        let people_box = PeopleBox { people: people };

        let mut pods: Vec<Pod> = vec![];
        for pod_id in 0..config.network.pods.n_pods {
            pods.push(Pod::new(pod_id, 10, POD_CAPACITY, calc_line_state(&pod_id)));
        }

        let pods_box = PodsBox { pods: pods };

        let state = State {
            network: network,
            people_box: people_box,
            pods_box: pods_box,
            time_passed: 0,
            config: config.clone(),
            on_pause: false,
            last_mouse_left: (0., 0.),
            rx: rx,
        };

        // println!("{:?}", state);
        // println!("conns: {:?}", state.network.lines);
        return state;
    }
}

impl EventHandler for State {
    // fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, repeat: bool) {

    // }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            self.last_mouse_left = (x, y)
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        if keycode == KeyCode::Space {
            self.on_pause = !self.on_pause;
        }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        while timer::check_update_time(ctx, DESIRED_FPS) {
            // println!("fps: {}", timer::fps(ctx));
            let (get_actions, set_actions) = handle_queries(&self, &self.rx);

            if get_actions.len() != 0 {
                println!("get_actions: {:?}", get_actions);
            }

            for get_action in get_actions {
                match get_action {
                    GetAction::GetStation { station_id } => {
                        let maybe_station = self.network.try_get_station_by_id(station_id);
                        match maybe_station {
                            Some(station) => {
                                println!("The station you are looking for: {:?}", station.name)
                            }
                            None => {
                                println!("No station with id {} exists", station_id)
                            }
                        }
                    }
                }
            }

            if !self.on_pause {
                self.time_passed += 1;
                self.update(set_actions);
            }

            if self.time_passed % 25 == 0 {
                // println!("-------------------------------");
                // println!("time passed: {}", self.time_passed);
                // self.print_state();
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);

        self.network.draw(ctx);
        self.pods_box.draw(ctx, &self.network);
        self.draw(ctx);
        // self.people_box.draw(ctx, &self.network);

        graphics::present(ctx)
    }
}
