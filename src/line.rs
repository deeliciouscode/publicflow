use crate::config::Config;
use crate::connection::{Connection, YieldTuple};
use crate::helper::get_screen_coordinates;
use crate::network::Network;
use ggez::graphics::Rect;
use ggez::{graphics, Context, GameResult};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Line {
    pub name: String,
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

// block connection 650-641-631-611
// block connections 5-44 5-0

#[derive(Clone, Debug)]
pub struct LineState {
    pub line: Line,
    pub line_ix: i32,
    pub next_ix: i32,
    pub direction: i32,
}

impl LineState {
    pub fn get_station_id(&self) -> i32 {
        self.line.stations[self.line_ix as usize]
    }

    pub fn get_next_station_id(&self) -> i32 {
        self.line.stations[self.next_ix as usize]
    }

    pub fn set_next_station_ix(&mut self) {
        if self.line_ix + self.direction > (self.line.stations.len() - 1) as i32 {
            if !self.line.circular {
                self.direction *= -1;
                self.next_ix = self.line_ix + self.direction;
            } else {
                self.next_ix = 0;
            }
        } else if self.line_ix + self.direction < 0 {
            if !self.line.circular {
                self.direction *= -1;
                self.next_ix = self.line_ix + self.direction;
            } else {
                self.next_ix = (self.line.stations.len() - 1) as i32;
            }
        } else {
            self.next_ix = self.line_ix + self.direction;
        }
    }

    pub fn update_line_ix(&mut self) {
        self.line_ix = self.next_ix;
    }

    pub fn get_connection(&self, fst: i32, snd: i32) -> Option<&Connection> {
        for connection in &self.line.connections {
            if connection.station_ids == HashSet::from([fst, snd]) {
                return Some(connection);
            }
        }
        return None;
    }

    pub fn get_current_connection(&self) -> Option<&Connection> {
        let fst = self.line.stations[self.line_ix as usize];
        let snd = self.line.stations[self.next_ix as usize];
        for connection in &self.line.connections {
            if connection.station_ids == HashSet::from([fst, snd]) {
                return Some(connection);
            }
        }
        return None;
    }
}
