use crate::action::{Actions, GetAction, SetAction};
use crate::cli::recv_queries;
use crate::config::Config;
use crate::helper::{apply_zoom, format_seconds};
use crate::line::{Line, LineState};
use crate::network::Network;
use crate::person::{PeopleBox, Person};
use crate::pod::{Pod, PodsBox};
use crate::station::{Platform, Station};
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
    rx: mpsc::Receiver<Actions>,
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

        self.network.update(&set_actions, &self.config);
        self.pods_box
            .update(&mut self.network, &set_actions, &self.config);
        self.people_box.update(
            &mut self.pods_box,
            &mut self.network,
            &set_actions,
            &self.config,
        );
    }

    fn handle_get_actions(&self, get_actions: Vec<GetAction>) {
        // if get_actions.len() != 0 {
        //     println!("get_actions: {:?}", get_actions);
        // }

        for get_action in get_actions {
            match get_action {
                GetAction::GetStation { id } => {
                    let maybe_station = self.network.try_get_station_by_id_unmut(id);
                    match maybe_station {
                        Some(station) => {
                            println!("----------------------");
                            println!("Id: {}", station.id);
                            println!("Name: {}", station.name);
                            println!("City: {}", station.city);
                            // println!("Since Last Pod: {}", station.since_last_pod);
                            println!("Platforms: {}", station.stringify_platforms());
                            println!("Edges To: {:?}", station.edges_to);
                            println!("Pods in Station: {:?}", station.pods_in_station);
                            println!("People in Station: {}", station.people_in_station.len());
                            println!("Coordinates: {:?}", station.coordinates);
                            println!("----------------------");
                        }
                        None => {
                            println!("No station with id {} exists", id)
                        }
                    }
                }
                GetAction::GetPerson { id } => {
                    let maybe_person = self.people_box.try_get_person_by_id_unmut(id);
                    match maybe_person {
                        Some(person) => {
                            println!("----------------------");
                            println!("Id: {}", person.id);
                            println!("Coordinates: {:?}", person.real_coordinates);
                            println!("Path: {:?}", person.path_state.path);
                            println!("----------------------");
                        }
                        None => {
                            println!("No person with id {} exists", id)
                        }
                    }
                }
                GetAction::GetPod { id } => {
                    let maybe_pod = self.pods_box.try_get_pod_by_id_unmut(id);
                    match maybe_pod {
                        Some(pod) => {
                            println!("----------------------");
                            println!("Id: {}", pod.id);
                            println!("Capacity: {:?}", pod.capacity);
                            println!("People in Pod: {:?}", pod.people_in_pod.len());
                            println!("Last Station: {:?}", pod.line_state.line_ix);
                            println!("Next Station: {:?}", pod.line_state.next_ix);
                            println!("----------------------");
                        }
                        None => {
                            println!("No pod with id {} exists", id)
                        }
                    }
                }
            }
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let mut time_passed = Text::new(String::from(format!(
            "Time passed: {}",
            format_seconds(self.time_passed)
        )));
        time_passed.set_font(Font::default(), PxScale::from(40.));
        let draw_param = DrawParam::new().offset([-10., -10.]).color(Color::WHITE);
        let res = graphics::draw(ctx, &time_passed, draw_param);
        match res {
            Ok(_) => {}
            Err(err) => panic!("{:?}", err),
        }

        let last_mouse_left = self.config.visual.last_mouse_left;

        if self.config.logic.on_pause {
            let maybe_station_group = self
                .network
                .try_retrieve_station_group(last_mouse_left, &self.config);
            match maybe_station_group {
                Some(station) => {
                    let mut name = Text::new(String::from(format!("Name: {}", station.name)));
                    name.set_font(Font::default(), PxScale::from(18.));
                    let mut id = Text::new(String::from(format!("ID: {}", station.id)));
                    id.set_font(Font::default(), PxScale::from(18.));
                    let mut count = Text::new(String::from(format!(
                        "People Count: {}",
                        station.people_in_station.len()
                    )));
                    count.set_font(Font::default(), PxScale::from(18.));
                    let mut pods =
                        Text::new(String::from(format!("Pods: {:?}", station.pods_in_station)));
                    pods.set_font(Font::default(), PxScale::from(18.));

                    let draw_param_name = DrawParam::new()
                        .offset([-last_mouse_left.0 - 10., -last_mouse_left.1 - 10.])
                        .color(Color::WHITE);
                    let res = graphics::draw(ctx, &name, draw_param_name);
                    let draw_param_name = DrawParam::new()
                        .offset([-last_mouse_left.0 - 10., -last_mouse_left.1 - 30.])
                        .color(Color::WHITE);
                    let res = graphics::draw(ctx, &id, draw_param_name);
                    let draw_param_count = DrawParam::new()
                        .offset([-last_mouse_left.0 - 10., -last_mouse_left.1 - 50.])
                        .color(Color::WHITE);
                    let res = graphics::draw(ctx, &count, draw_param_count);
                    let draw_param_pods = DrawParam::new()
                        .offset([-last_mouse_left.0 - 10., -last_mouse_left.1 - 70.])
                        .color(Color::WHITE);
                    let res = graphics::draw(ctx, &pods, draw_param_pods);
                }
                None => {
                    let maybe_pod = self.pods_box.try_retrieve_pod(last_mouse_left);
                    match maybe_pod {
                        Some(pod) => {
                            let mut id = Text::new(String::from(format!("ID: {}", pod.id)));
                            id.set_font(Font::default(), PxScale::from(18.));
                            let mut line = Text::new(String::from(format!(
                                "Line: {}",
                                pod.line_state.line.name
                            )));
                            line.set_font(Font::default(), PxScale::from(18.));
                            let mut count = Text::new(String::from(format!(
                                "Passengers: {}",
                                pod.people_in_pod.len()
                            )));
                            count.set_font(Font::default(), PxScale::from(18.));
                            let mut capacity =
                                Text::new(String::from(format!("Capacity: {}", pod.capacity)));
                            capacity.set_font(Font::default(), PxScale::from(18.));

                            let draw_param_id = DrawParam::new()
                                .offset([-last_mouse_left.0 - 10., -last_mouse_left.1 - 10.])
                                .color(Color::WHITE);
                            let res = graphics::draw(ctx, &id, draw_param_id);
                            let draw_param_line = DrawParam::new()
                                .offset([-last_mouse_left.0 - 10., -last_mouse_left.1 - 30.])
                                .color(Color::WHITE);
                            let res = graphics::draw(ctx, &line, draw_param_line);
                            let draw_param_count = DrawParam::new()
                                .offset([-last_mouse_left.0 - 10., -last_mouse_left.1 - 50.])
                                .color(Color::WHITE);
                            let res = graphics::draw(ctx, &count, draw_param_count);
                            let draw_param_count = DrawParam::new()
                                .offset([-last_mouse_left.0 - 10., -last_mouse_left.1 - 70.])
                                .color(Color::WHITE);
                            let res = graphics::draw(ctx, &capacity, draw_param_count);
                        }
                        None => {}
                    }
                }
            }
        }
    }

    // TODO:PRIO: implement spwaning of pods at a given rate till there are enough
    // as a next step spawn / divert pods dynamically
    pub fn new(config: Config, rx: mpsc::Receiver<Actions>) -> Self {
        let mut stations: Vec<Station> = vec![];
        for abstract_station in config.network.coordinates_map_stations.iter() {
            let station_id = abstract_station.0;
            let (name, city, (lat, lon)) = abstract_station.1;

            // println!("{:?}", config.network.edge_map.get(&station_id).unwrap());
            let abstract_platforms = config
                .network
                .platforms_to_stations
                .get(station_id)
                .unwrap();
            let mut platforms = vec![];

            for abstract_platform in abstract_platforms {
                platforms.push(Platform::new(
                    abstract_platform.0,
                    &abstract_platform.1,
                    &abstract_platform.2,
                ))
            }

            stations.push(Station {
                id: *station_id,
                visualize: false,
                name: name.clone(),
                city: city.clone(),
                edges_to: config.network.edge_map.get(&station_id).unwrap().clone(),
                pods_in_station: HashSet::from([]), // The pods will register themselves later
                people_in_station: HashSet::from([]),
                coordinates: (*lat as f32, *lon as f32),
                platforms: platforms,
            })
        }

        // println!("{:?}", stations);

        let mut network = Network::new(stations, &config);

        let people_box = PeopleBox { people: vec![] };

        let pods_box = PodsBox { pods: vec![] };

        let state = State {
            network: network,
            people_box: people_box,
            pods_box: pods_box,
            time_passed: 0,
            config: config,
            rx: rx,
        };

        return state;
    }

    pub fn add_pods(mut self) -> Self {
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

            for lineref in &self.network.lines {
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

        let mut pods: Vec<Pod> = vec![];
        for pod_id in 0..self.config.logic.number_of_pods {
            pods.push(Pod::new(
                pod_id,
                10,
                self.config.logic.pod_capacity,
                calc_line_state(&pod_id),
            ));
        }

        let pods_box = PodsBox { pods: pods };

        self.pods_box = pods_box;

        self
    }

    pub fn add_people(mut self) -> Self {
        let mut rng = rand::thread_rng();
        let station_ids: Vec<&i32> = self
            .config
            .network
            .coordinates_map_stations
            .keys()
            .collect();
        // println!("{:?}", station_ids);

        let mut people: Vec<Person> = vec![];
        for person_id in 0..self.config.logic.number_of_people {
            let start_ix = rng.gen_range(0..station_ids.len());
            let end_ix = rng.gen_range(0..station_ids.len());
            let start = station_ids[start_ix];
            let end = station_ids[end_ix];
            people.push(Person::new(
                person_id,
                self.config.logic.transition_time,
                &self.network,
                *start,
                *end,
                &self.config,
            ));
        }

        for person in &people {
            let station = self
                .network
                .try_get_station_by_id(person.path_state.get_current_station_id().unwrap() as i32)
                .unwrap();
            station.register_person(person.id);
        }

        let people_box = PeopleBox { people: people };

        self.people_box = people_box;

        self
    }
}

impl EventHandler for State {
    // if this will be implmented do not forget to implment ESC -> exit, since that has to be done manually
    // fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, repeat: bool) {}

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.config.visual.last_mouse = (x, y)
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) {
        let x_rel = self.config.visual.last_mouse.0 / self.config.visual.screen_size.0;
        let y_rel = self.config.visual.last_mouse.1 / self.config.visual.screen_size.1;

        self.config.visual.last_mouse_while_zooming_relative = (x_rel, y_rel);
        apply_zoom(&mut self.config, y);
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            self.config.visual.last_mouse_left = (x, y)
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        if keycode == KeyCode::Space {
            self.config.logic.on_pause = !self.config.logic.on_pause;
        }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        while timer::check_update_time(ctx, self.config.visual.desired_fps) {
            // println!("fps: {}", timer::fps(ctx));
            let actions = recv_queries(&self, &self.rx);

            self.handle_get_actions(actions.get_actions);

            if !self.config.logic.on_pause {
                self.time_passed += 1;
                self.update(actions.set_actions);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let bg_color = Color::new(0.15, 0.15, 0.15, 1.0);
        graphics::clear(ctx, bg_color);

        self.network.draw(ctx, &self.config);
        self.pods_box.draw(ctx, &self.network, &self.config);
        self.people_box.draw(ctx, &self.network, &self.config);
        self.draw(ctx);

        graphics::present(ctx)
    }
}
