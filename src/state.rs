use crate::config::constants::{CONFIG_NAME, CONFIG_ROOT};
use crate::config::structs::Config;
use crate::control::action::{Action, Actions};
use crate::control::proxy::recv_actions;
use crate::helper::enums::Direction;
use crate::helper::functions::calc_graph;
use crate::helper::functions::{apply_zoom, format_seconds};
use crate::helper::printer::{print_get_person, print_get_pod, print_get_station};
use crate::network::Network;
use crate::person::peoplebox::PeopleBox;
use crate::person::person::Person;
use crate::pod::podsbox::PodsBox;
use crate::station::platform::Platform;
use crate::station::station::Station;
use ggez::event::{EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, Color, DrawParam, Font, PxScale, Text};
use ggez::{timer, Context, GameResult};
use rand::Rng;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::process::exit;
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
    pub fn update(&mut self) {
        self.network.update();
        self.pods_box
            .update(&mut self.network, &self.config, self.time_passed);
        self.people_box.update(
            &mut self.pods_box,
            &mut self.network,
            &self.config,
            self.time_passed,
        );
    }

    fn handle_actions(&mut self, action: Actions) {
        let mut recalculate_graph = false;
        for action in action.actions {
            if action != Action::NoAction {
                println!("{:?}", action);
            }
            match action {
                // The special case where nothing is done, used for debugging purposes
                Action::NoAction => {}
                // Actions that just retrieve something
                Action::GetStation { id } => print_get_station(&self.network, id),
                Action::GetPerson { id } => print_get_person(&self.people_box, id),
                Action::GetPod { id } => print_get_pod(&self.pods_box, id),
                // Actions with effect on the state
                Action::BlockConnection { ids } => {
                    self.network.apply_block_connection(&ids);
                    self.pods_box.apply_block_connection(&ids);
                    recalculate_graph = true;
                }
                Action::UnblockConnection { ids } => {
                    self.network.apply_unblock_connection(&ids);
                    self.pods_box.apply_unblock_connection(&ids);
                    recalculate_graph = true;
                }
                Action::MakePlatformOperational {
                    station_id,
                    line_name,
                    direction,
                } => {
                    self.network
                        .apply_make_platform_op(station_id, line_name, direction);
                    recalculate_graph = true;
                }
                Action::MakePlatformPassable {
                    station_id,
                    line_name,
                    direction,
                } => {
                    self.network
                        .apply_make_platform_pass(station_id, line_name, direction);
                    recalculate_graph = true;
                }
                Action::MakePlatformQueueable {
                    station_id,
                    line_name,
                    direction,
                } => {
                    self.network
                        .apply_make_platform_qu(station_id, line_name, direction);
                    recalculate_graph = true;
                }
                Action::SpawnPod {
                    station_id,
                    line_name,
                    direction,
                    force,
                } => {
                    self.network.apply_spawn_pod(
                        station_id,
                        line_name,
                        direction,
                        force,
                        &mut self.pods_box,
                        &self.config,
                        self.time_passed,
                    );
                }
                Action::ShowPerson { id } => self.people_box.apply_show_person(id),
                Action::HidePerson { id } => self.people_box.apply_hide_person(id),
                Action::ShowPod { id } => self.pods_box.apply_show_pod(id),
                Action::HidePod { id } => self.pods_box.apply_hide_pod(id),
                Action::ShowStation { id } => self.network.apply_show_station(id),
                Action::HideStation { id } => self.network.apply_hide_station(id),
                Action::RoutePerson {
                    id,
                    stay_there: _,
                    station_id: _,
                    random_station: _,
                } => self.people_box.apply_route_person(id, action),
                Action::GatherMetrics => {
                    self.people_box.start_gather_metrics();
                    self.pods_box.start_gather_metrics();
                }
                Action::KillSimulation { code } => {
                    exit(code);
                }
                Action::DumpMetricsPerson { person_id } => {
                    self.people_box.dump_metrics(person_id, &self.config);
                }
                Action::DumpMetricsPeople { all, avg } => {
                    if avg {
                        self.people_box.dump_avg_metrics(&self.config);
                    }
                    if all {
                        self.people_box.dump_all_metrics(&self.config);
                    }
                }
                Action::DumpMetricsPods { all, avg } => {
                    if avg {
                        self.pods_box.dump_avg_metrics(&self.config);
                    }
                    if all {
                        self.pods_box.dump_all_metrics(&self.config)
                    }
                }
                Action::DumpMetricsPod { pod_id } => {
                    self.pods_box.dump_metrics(pod_id, &self.config);
                }
                Action::DumpConfig => {
                    self.dump_config();
                }
                Action::Sleep { duration: _ }
                | Action::Loop { n: _ }
                | Action::Endloop
                | Action::StartConcurency
                | Action::DoConcurrently
                | Action::EndConcurency => {
                    // Do Nothing, these are handled in the action proxy
                    // Also this should never be reached since the proxy thread
                    // is not supposed to send these actions forward.
                }
            }
        }
        if recalculate_graph {
            self.network.graph = calc_graph(&self.network.lines);
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let mut time_passed = Text::new(String::from(format!(
            "Time passed: {}",
            format_seconds(self.time_passed)
        )));
        time_passed.set_font(Font::default(), PxScale::from(40.));
        let draw_param = DrawParam::new().offset([-10., -10.]).color(Color::WHITE);
        let _res = graphics::draw(ctx, &time_passed, draw_param);
        match _res {
            Ok(_) => {}
            Err(err) => panic!("{:?}", err),
        }

        let last_mouse_left = self.config.visual.last_mouse_left;

        if self.config.logic.on_pause {
            let maybe_station = self
                .network
                .try_retrieve_station(last_mouse_left, &self.config);
            match maybe_station {
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
                    let mut pods = Text::new(String::from(format!(
                        "Pods: {:?}",
                        station.get_pods_in_station_as_vec()
                    )));
                    pods.set_font(Font::default(), PxScale::from(18.));

                    let draw_param_name = DrawParam::new()
                        .offset([-last_mouse_left.0 - 10., -last_mouse_left.1 - 10.])
                        .color(Color::WHITE);
                    let _res = graphics::draw(ctx, &name, draw_param_name);
                    let draw_param_name = DrawParam::new()
                        .offset([-last_mouse_left.0 - 10., -last_mouse_left.1 - 30.])
                        .color(Color::WHITE);
                    let _res = graphics::draw(ctx, &id, draw_param_name);
                    let draw_param_count = DrawParam::new()
                        .offset([-last_mouse_left.0 - 10., -last_mouse_left.1 - 50.])
                        .color(Color::WHITE);
                    let _res = graphics::draw(ctx, &count, draw_param_count);
                    let draw_param_pods = DrawParam::new()
                        .offset([-last_mouse_left.0 - 10., -last_mouse_left.1 - 70.])
                        .color(Color::WHITE);
                    let _res = graphics::draw(ctx, &pods, draw_param_pods);
                }
                None => {
                    let maybe_pod = self.pods_box.try_retrieve_pod(last_mouse_left);
                    match maybe_pod {
                        Some(pod) => {
                            let mut id = Text::new(String::from(format!("ID: {}", pod.id)));
                            id.set_font(Font::default(), PxScale::from(18.));
                            let mut line = Text::new(String::from(format!(
                                "Line: {:?}",
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
                            let _res = graphics::draw(ctx, &id, draw_param_id);
                            let draw_param_line = DrawParam::new()
                                .offset([-last_mouse_left.0 - 10., -last_mouse_left.1 - 30.])
                                .color(Color::WHITE);
                            let _res = graphics::draw(ctx, &line, draw_param_line);
                            let draw_param_count = DrawParam::new()
                                .offset([-last_mouse_left.0 - 10., -last_mouse_left.1 - 50.])
                                .color(Color::WHITE);
                            let _res = graphics::draw(ctx, &count, draw_param_count);
                            let draw_param_count = DrawParam::new()
                                .offset([-last_mouse_left.0 - 10., -last_mouse_left.1 - 70.])
                                .color(Color::WHITE);
                            let _res = graphics::draw(ctx, &capacity, draw_param_count);
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
            let (name, entrypoint_for, city, (lat, lon)) = abstract_station.1;

            // println!("{:?}", config.network.edge_map.get(&station_id).unwrap());
            let abstract_platforms = config.network.station_platforms.get(station_id).unwrap();
            let mut platforms = vec![];

            for abstract_platform in abstract_platforms {
                platforms.push(Platform::new(
                    &config,
                    *station_id,
                    entrypoint_for,
                    Direction::Pos,
                    &abstract_platform.0,
                    &abstract_platform.1,
                ));
                platforms.push(Platform::new(
                    &config,
                    *station_id,
                    entrypoint_for,
                    Direction::Neg,
                    &abstract_platform.0,
                    &abstract_platform.1,
                ));
            }

            stations.push(Station {
                id: *station_id,
                visualize: false,
                name: name.clone(),
                city: city.clone(),
                edges_to: config.network.edge_map.get(&station_id).unwrap().clone(),
                // pods_in_station: HashSet::from([]), // The pods will register themselves later
                people_in_station: HashSet::from([]),
                coordinates: (*lat as f32, *lon as f32),
                platforms: platforms,
            })
        }

        // println!("{:?}", stations);

        let network = Network::new(stations, &config);

        // println!("{:?}", network.lines);

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

    pub fn dump_config(&self) {
        if let Some(timestamp_run) = self.config.timestamp_run {
            let timestamp = timestamp_run
                .naive_utc()
                .format("%Y.%m.%d %H:%M")
                .to_string()
                .replace(" ", "_");
            println!("timestamp_run: {:?}", timestamp);

            let source_path_str = format!("{}/{}", CONFIG_ROOT, CONFIG_NAME);
            let dest_path_str = format!(
                "{}/{}/{}/{}.yaml",
                "metrics", self.config.environment, timestamp, "config"
            );
            let path = Path::new(&dest_path_str);
            let parent = path.parent().unwrap();
            let _res = fs::create_dir_all(parent);
            let _res = fs::copy(source_path_str, dest_path_str);
        }
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
                .try_get_station_by_id(
                    person.path_state.try_get_current_station_id().unwrap() as i32
                )
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
        while timer::check_update_time(ctx, self.config.logic.speed_multiplier) {
            // println!("fps: {}", timer::fps(ctx));
            let actions = recv_actions(&self.rx);
            self.handle_actions(actions);

            if !self.config.logic.on_pause {
                self.time_passed += 1;
                self.update();
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let bg_color = Color::new(0.15, 0.15, 0.15, 1.0);
        graphics::clear(ctx, bg_color);
        self.network.draw(ctx, &self.config);
        self.pods_box.draw(ctx, &self.config);
        self.people_box.draw(ctx);
        self.draw(ctx);
        graphics::present(ctx)
    }
}
