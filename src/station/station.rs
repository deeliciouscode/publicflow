use crate::config::structs::Config;
use crate::helper::enums::LineName;
use crate::helper::helper::get_screen_coordinates;
use crate::station::platform::Platform;
use crate::station::platformstate::PlatformState;
use ggez::graphics::{Font, Text};
use ggez::{graphics, Context, GameResult};
use std::collections::HashSet;
use std::collections::VecDeque;

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
    pub fn update(&mut self) {
        for platform in &mut self.platforms {
            platform.update();
        }
    }

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
                "\n Lines: {:?} | Neighbors: {:?} | Direction: {:?} | State: {:?} | Pods: {:?}",
                platform.edges_to,
                platform.lines_using_this,
                platform.direction,
                platform.state,
                platform.pods_at_platform,
            ))
        }
        platforms_string
    }

    pub fn make_operational(&mut self, line: &LineName) {
        for platform in &mut self.platforms {
            if platform.lines_using_this.contains(line) {
                match &platform.state {
                    PlatformState::Queuable { queue } => {
                        platform.state = PlatformState::Operational {
                            queue: queue.clone(),
                        }
                    }
                    PlatformState::Passable => {
                        platform.state = PlatformState::Operational {
                            queue: VecDeque::from([]),
                        }
                    }
                    PlatformState::Operational { queue: _ } => {
                        panic!("Is Operational already.")
                    }
                }
            }
        }
    }

    pub fn make_passable(&mut self, line: &LineName) {
        for platform in &mut self.platforms {
            if platform.lines_using_this.contains(line) {
                platform.state = PlatformState::Passable
            }
        }
    }

    pub fn make_queuable(&mut self, line: &LineName) {
        for platform in &mut self.platforms {
            if platform.lines_using_this.contains(line) {
                match &platform.state {
                    PlatformState::Operational { queue } => {
                        platform.state = PlatformState::Queuable {
                            queue: queue.clone(),
                        }
                    }
                    PlatformState::Passable => {
                        platform.state = PlatformState::Queuable {
                            queue: VecDeque::from([]),
                        }
                    }
                    PlatformState::Queuable { queue: _ } => {
                        panic!("Is Queuable already.")
                    }
                }
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context, config: &Config) -> GameResult<()> {
        let mut _res;
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

        _res = graphics::draw(ctx, &circle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));

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

            _res = graphics::draw(ctx, &circle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
        }

        match _res {
            Err(err) => panic!("Error 0: {}", err),
            Ok(_m) => {
                // println!("No error at 0: {:?}", m),
            }
        }

        let count = Text::new(String::from(self.people_in_station.len().to_string()));
        _res = graphics::draw(
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

        _res = graphics::draw(ctx, &circle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));

        match _res {
            Err(err) => panic!("Error 3: {}", err),
            Ok(_m) => {
                // println!("No error at 3: {:?}", m);
                return _res;
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
