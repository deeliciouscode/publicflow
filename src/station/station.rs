use crate::config::structs::Config;
use crate::helper::enums::{Direction, LineName};
use crate::helper::functions::get_screen_coordinates;
use crate::line::line::Line;
use crate::pod::podsbox::PodsBox;
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

    pub fn spawn_pod(
        &mut self,
        line_name: &LineName,
        direction: &Direction,
        force: bool,
        pods_box: &mut PodsBox,
        lines: &Vec<Line>,
        config: &Config,
        time_passed: u32,
    ) {
        for platform in &mut self.platforms {
            if (platform.can_spawn_for.contains(line_name) || force)
                && platform.lines_using_this.contains(&line_name)
                && direction == &platform.direction
            {
                pods_box.add_pod(&line_name, &direction, &self.id, lines, config, time_passed);
            }
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
                "\n Lines: {:?} | Spawns for: {:?} | Neighbors: {:?} | Direction: {:?} | State: {:?} | Pods: {:?}",
                platform.lines_using_this,
                platform.can_spawn_for,
                platform.edges_to,
                platform.direction,
                platform.state,
                platform.pods_at_platform,
            ))
        }
        platforms_string
    }

    pub fn make_operational(&mut self, line: &LineName, direction: &Direction) {
        for platform in &mut self.platforms {
            if platform.lines_using_this.contains(line) && &platform.direction == direction {
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
                        println!("Is Operational already.")
                    }
                }
            }
        }
    }

    pub fn make_passable(&mut self, line: &LineName, direction: &Direction) {
        for platform in &mut self.platforms {
            if platform.lines_using_this.contains(line) && &platform.direction == direction {
                platform.state = PlatformState::Passable
            }
        }
    }

    pub fn make_queuable(&mut self, line: &LineName, direction: &Direction) {
        for platform in &mut self.platforms {
            if platform.lines_using_this.contains(line) && &platform.direction == direction {
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
                        println!("Is Queuable already.")
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
        let fair_share = config.logic.number_of_people as f32 / config.network.n_stations as f32;
        let large_share = fair_share * 3.;
        let max_radius = config.visual.radius_station * 10.;
        let min_radius = config.visual.radius_station;
        let calculated_radius =
            min_radius + (self.people_in_station.len() as f32 / large_share) * max_radius;
        let radius;
        if calculated_radius > max_radius {
            radius = max_radius;
        } else {
            radius = calculated_radius;
        }

        // if self.id == 0 {
        //     println!("{}, {}", self.people_in_station.len() as f32, config.logic.number_of_people as f32);
        //     println!("calculated radius: {} | radius: {}", calculated_radius, radius);
        // }

        let red = self.people_in_station.len() as f32 / fair_share;
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
