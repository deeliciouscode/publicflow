use crate::config::Config;
use crate::helper::get_screen_coordinates;
use ggez::graphics::{Font, Rect, Text};
use ggez::{graphics, Context, GameResult};
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Station {
    pub id: i32,
    pub visualize: bool,
    pub name: String,
    pub city: String,
    pub edges_to: HashSet<i32>,
    // pub pods_in_station: HashSet<i32>,
    pub people_in_station: HashSet<i32>,
    pub coordinates: (f32, f32),
    pub platforms: Vec<Platform>,
}

impl Station {
    pub fn update(&self) {}

    pub fn get_pods_in_station_as_vec(&self) -> Vec<i32> {
        let mut pods_in_station: Vec<i32> = vec![];
        for platform in &self.platforms {
            pods_in_station.extend(platform.pods_at_platform.clone())
        }
        pods_in_station.sort();
        pods_in_station
    }

    pub fn stringify_platforms(&self) -> String {
        let mut platforms_string = "".to_string();
        for platform in &self.platforms {
            platforms_string.push_str(&format!(
                "\n ID: {} | Neighbors: {:?} | Lines: {:?} | State: {:?} | Pods: {:?}",
                platform.id,
                platform.edges_to,
                platform.lines_using_this,
                platform.state,
                platform.pods_at_platform
            ))
        }
        platforms_string
    }

    pub fn make_operational(&mut self, line: &String) {
        for platform in &mut self.platforms {
            if platform.lines_using_this.contains(line) {
                platform.state = PlatformState::Operational
            }
        }
    }

    pub fn make_passable(&mut self, line: &String) {
        for platform in &mut self.platforms {
            if platform.lines_using_this.contains(line) {
                platform.state = PlatformState::Passable
            }
        }
    }

    pub fn make_queuable(&mut self, line: &String) {
        for platform in &mut self.platforms {
            if platform.lines_using_this.contains(line) {
                platform.state = PlatformState::Queuable { queue: vec![] }
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context, config: &Config) -> GameResult<()> {
        let mut res;
        let color = [0.5, 0.5, 0.5, 1.0].into();

        let real_coordinates = get_screen_coordinates(self.coordinates, config);

        let circle = graphics::Mesh::new_circle(
            ctx,
            // graphics::DrawMode::stroke(2.),
            graphics::DrawMode::fill(),
            ggez::mint::Point2 {
                x: real_coordinates.0,
                y: real_coordinates.1,
            },
            config.visual.radius_station,
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
                    x: real_coordinates.0,
                    y: real_coordinates.1,
                },
                config.visual.radius_station + 4.,
                1.,
                color,
            )?;

            res = graphics::draw(ctx, &circle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
        }

        match res {
            Err(err) => panic!("Error 0: {}", err),
            Ok(_m) => {
                // println!("No error at 0: {:?}", m),
            }
        }

        let count = Text::new(String::from(self.people_in_station.len().to_string()));
        res = graphics::draw(
            ctx,
            &count,
            (ggez::mint::Point2 {
                x: real_coordinates.0 - Font::DEFAULT_FONT_SCALE / 2.,
                y: real_coordinates.1 - Font::DEFAULT_FONT_SCALE / 2.,
            },),
        );

        // match res {
        //     Err(err) => panic!("Error 2: {}", err),
        //     Ok(m) => {
        //         // println!("No error at 2: {:?}", m),
        //     }
        // }

        // let radius = self.people_in_station.len() as f32 / 10.;
        let radius = self.people_in_station.len() as f32;

        let red =
            radius / (config.logic.number_of_people as f32 / config.network.n_stations as f32);
        let green = 1. - red;

        let color_circle = [red, green, 0., 0.2].into();

        // println!("--------------------");
        // println!("people in station {}: {}", self.id, radius);
        // println!("real coordinates: {}, {}", real_coordinates.0, real_coordinates.1);
        // println!("radius: {}", radius);

        let circle = graphics::Mesh::new_circle(
            ctx,
            // graphics::DrawMode::stroke(2.),
            graphics::DrawMode::fill(),
            ggez::mint::Point2 {
                x: real_coordinates.0,
                y: real_coordinates.1,
            },
            radius,
            1.,
            color_circle,
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

    pub fn register_person(&mut self, person_id: i32) {
        // println!("register person {} in station {}.", person_id, self.id);
        self.people_in_station.insert(person_id);
    }

    pub fn deregister_person(&mut self, person_id: i32) {
        self.people_in_station.remove(&person_id);
    }

    pub fn try_get_pod_ids_in_station_as_vec(&mut self) -> Option<Vec<i32>> {
        if self.get_pods_in_station_as_vec().is_empty() {
            return None;
        }
        Some(self.get_pods_in_station_as_vec())
    }
}

#[derive(Clone, Debug)]
pub struct Platform {
    pub id: i32,
    pub since_last_pod: i32,
    pub edges_to: HashSet<i32>,
    pub lines_using_this: std::vec::Vec<std::string::String>,
    pub pods_at_platform: HashSet<i32>,
    pub state: PlatformState,
}

impl Platform {
    pub fn new(
        id: i32,
        edges_to: &HashSet<i32>,
        lines_using_this: &Vec<std::string::String>,
    ) -> Self {
        Platform {
            id: id,
            since_last_pod: 0,
            edges_to: edges_to.clone(),
            lines_using_this: lines_using_this.clone(),
            pods_at_platform: HashSet::new(),
            state: PlatformState::Operational,
        }
    }

    pub fn is_operational(&self) -> bool {
        match self.state {
            PlatformState::Operational => true,
            _ => false,
        }
    }

    pub fn is_queuable(&self) -> bool {
        match self.state {
            PlatformState::Queuable { queue: _ } => true,
            _ => false,
        }
    }

    pub fn register_pod(&mut self, pod_id: i32) {
        self.pods_at_platform.insert(pod_id);
    }

    pub fn deregister_pod(&mut self, pod_id: i32) {
        self.pods_at_platform.remove(&pod_id);
    }

    pub fn queue_pod(&mut self, pod_id: i32) {
        match &self.state {
            PlatformState::Queuable { queue } => {
                let mut new_queue = queue.clone();
                new_queue.push(pod_id);
                self.state = PlatformState::Queuable { queue: new_queue }
            }
            _ => {
                println!(
                    "Pod can not be queued because platform is in state {:?}",
                    self.state
                )
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum PlatformState {
    Operational,
    Passable,
    Queuable { queue: Vec<i32> },
    InvalidState { reason: String },
}
