use crate::config::{Config, MAX_XY, OFFSET, RADIUS_STATION, SCREEN_SIZE, SIDELEN_STATION};
use crate::helper::get_real_coordinates;
use ggez::graphics::{Font, Rect, Text};
use ggez::{graphics, Context, GameResult};
use std::collections::HashSet;

// TODO: Maybe define capacity of HashSet when using it (for performance)
#[derive(Clone, Debug)]
pub struct Station {
    pub id: i32,
    pub name: String,
    pub city: String,
    pub since_last_pod: i32,
    pub edges_to: HashSet<i32>,
    pub pods_in_station: HashSet<i32>,
    pub people_in_station: HashSet<i32>,
    pub coordinates: (f32, f32),
    pub config: Config,
}

impl Station {
    pub fn register_pod(&mut self, pod_id: i32) {
        self.pods_in_station.insert(pod_id);
    }

    pub fn deregister_pod(&mut self, pod_id: i32) {
        self.pods_in_station.remove(&pod_id);
    }

    pub fn register_person(&mut self, person_id: i32) {
        // println!("register person {} in station {}.", person_id, self.id);
        self.people_in_station.insert(person_id);
    }

    pub fn deregister_person(&mut self, person_id: i32) {
        self.people_in_station.remove(&person_id);
    }

    pub fn get_pod_ids_in_station_as_vec(&mut self) -> Option<Vec<i32>> {
        if self.pods_in_station.is_empty() {
            return None;
        }
        Some(self.pods_in_station.clone().into_iter().collect())
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let mut res;
        let color = [0.5, 0.5, 0.5, 1.0].into();

        let real_coordinates = get_real_coordinates(self.coordinates);

        // println!("real_coordinates: {:?}", real_coordinates);

        // let station_rect = Rect {
        //     x: real_coordinates.0,
        //     y: real_coordinates.1,
        //     w: SIDELEN_STATION,
        //     h: SIDELEN_STATION,
        // };

        // let rectangle =
        //     graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), station_rect, color)?;

        // res = graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));

        let circle = graphics::Mesh::new_circle(
            ctx,
            // graphics::DrawMode::stroke(2.),
            graphics::DrawMode::fill(),
            ggez::mint::Point2 {
                x: real_coordinates.0,
                y: real_coordinates.1,
            },
            RADIUS_STATION,
            1.,
            color,
        )?;

        res = graphics::draw(ctx, &circle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));

        match res {
            Err(err) => panic!("Error 0: {}", err),
            Ok(m) => {
                // println!("No error at 0: {:?}", m),
            }
        }

        // let text = Text::new(String::from(self.name.clone()));

        // res = graphics::draw(
        //     ctx,
        //     &text,
        //     (ggez::mint::Point2 {
        //         x: real_coordinates.0,
        //         y: real_coordinates.1,
        //     },),
        // );

        // match res {
        //     Err(err) => panic!("Error 1: {}", err),
        //     Ok(m) => {
        //         // println!("No error at 1: {:?}", m),
        //     }
        // }

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
            radius / (self.config.people.n_people as f32 / self.config.network.n_stations as f32);
        let green = 1. - red;

        let color_circle = [red, green, 0., 0.5].into();

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
}
