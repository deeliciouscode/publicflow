use crate::config::structs::Config;
use crate::connection::{Connection, YieldTuple};
use crate::helper::enums::LineName;
use crate::helper::helper::get_screen_coordinates;
use crate::network::Network;
use ggez::{graphics, Context, GameResult};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Line {
    pub name: LineName,
    pub stations: Vec<i32>,
    pub distances: Vec<i32>,
    pub circular: bool,
    pub connections: Vec<Connection>,
}

impl Line {
    // TODO: handle result better
    pub fn draw(&self, ctx: &mut Context, network: &Network, config: &Config) -> GameResult<()> {
        let mut res: GameResult<()> = std::result::Result::Ok(());

        for connection in &self.connections {
            let mut color = [0.5, 0.5, 0.5, 1.0].into();
            if connection.is_blocked {
                color = [1.0, 0.2, 0.2, 1.0].into();
            }
            let station_ids = &connection.yield_tuple();
            // println!("MARKER: {:?}", station_ids);
            let from = network.try_get_station_by_id_unmut(station_ids.0).unwrap();
            let to = network.try_get_station_by_id_unmut(station_ids.1).unwrap();

            let x1: f32;
            let x2: f32;
            let y1: f32;
            let y2: f32;

            if (to.coordinates.0 == from.coordinates.0 && to.coordinates.1 > from.coordinates.1)
                || to.coordinates.0 > from.coordinates.0
            {
                x1 = from.coordinates.0;
                y1 = from.coordinates.1;
                x2 = to.coordinates.0;
                y2 = to.coordinates.1;
            } else {
                x1 = to.coordinates.0;
                y1 = to.coordinates.1;
                x2 = from.coordinates.0;
                y2 = from.coordinates.1;
            }

            let (x1_real, y1_real) = get_screen_coordinates((x1, y1), config);
            let (x2_real, y2_real) = get_screen_coordinates((x2, y2), config);

            let line = graphics::Mesh::new_line(
                ctx,
                &[[x1_real, y1_real], [x2_real, y2_real]],
                config.visual.width_line,
                color,
            )?;

            res = graphics::draw(ctx, &line, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
        }
        res
    }

    pub fn block_connection(&mut self, ids: &HashSet<i32>) {
        for connection in &mut self.connections {
            if &connection.station_ids == ids {
                // println!("{:?} | {:?} - blocked", ids, connection.station_ids);
                connection.is_blocked = true;
                // println!("connection: {:?}", connection);
            }
        }
    }

    pub fn unblock_connection(&mut self, ids: &HashSet<i32>) {
        for connection in &mut self.connections {
            if &connection.station_ids == ids {
                // println!("{:?} | {:?} - blocked", ids, connection.station_ids);
                connection.is_blocked = false;
                // println!("connection: {:?}", connection);
            }
        }
    }
}
