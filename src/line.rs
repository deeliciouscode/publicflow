use crate::config::{MAX_XY, OFFSET, SCREEN_SIZE, SIDELEN_STATION, WIDTH_LINE};
use crate::connection::{Connection, YieldTuple};
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
    pub fn draw(&self, ctx: &mut Context, network: &Network) -> GameResult<()> {
        let color = [0.8, 0.8, 0.8, 1.0].into();
        let mut res: GameResult<()> = std::result::Result::Ok(());

        for connection in &self.connections {
            let station_ids = &connection.yield_tuple();
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

            let x1_real = OFFSET
                + (x1 / MAX_XY.0 * SCREEN_SIZE.0)
                    * ((SCREEN_SIZE.0 - 2.0 * OFFSET) / SCREEN_SIZE.0) as f32;

            let y1_real = OFFSET
                + (y1 / MAX_XY.1 * SCREEN_SIZE.1)
                    * ((SCREEN_SIZE.1 - 2.0 * OFFSET) / SCREEN_SIZE.1) as f32;

            let x2_real = OFFSET
                + (x2 / MAX_XY.0 * SCREEN_SIZE.0)
                    * ((SCREEN_SIZE.0 - 2.0 * OFFSET) / SCREEN_SIZE.0) as f32;

            let y2_real = OFFSET
                + (y2 / MAX_XY.1 * SCREEN_SIZE.1)
                    * ((SCREEN_SIZE.1 - 2.0 * OFFSET) / SCREEN_SIZE.1) as f32;

            let mut x1_left_offset: f32 = 0.;
            let mut y1_left_offset: f32 = 0.;
            let mut x2_left_offset: f32 = 0.;
            let mut y2_left_offset: f32 = 0.;
            let mut x1_right_offset: f32 = 0.;
            let mut y1_right_offset: f32 = 0.;
            let mut x2_right_offset: f32 = 0.;
            let mut y2_right_offset: f32 = 0.;

            let mx = (y1_real - y2_real) / (x2_real - x1_real);
            // println!("ids: {:?} | mx: {}", station_ids, mx);
            if mx == std::f32::INFINITY || mx == std::f32::NEG_INFINITY {
                x1_left_offset = WIDTH_LINE / 2.;
                y1_left_offset = SIDELEN_STATION - WIDTH_LINE / 2.;
                x2_left_offset = WIDTH_LINE / 2.;
                y2_left_offset = WIDTH_LINE / 2.;
                x1_right_offset = SIDELEN_STATION - WIDTH_LINE / 2.;
                y1_right_offset = SIDELEN_STATION - WIDTH_LINE / 2.;
                x2_right_offset = SIDELEN_STATION - WIDTH_LINE / 2.;
                y2_right_offset = WIDTH_LINE / 2.;
            } else if mx == 0. {
                x1_left_offset = SIDELEN_STATION - WIDTH_LINE / 2.;
                y1_left_offset = WIDTH_LINE / 2.;
                x2_left_offset = WIDTH_LINE / 2.;
                y2_left_offset = WIDTH_LINE / 2.;
                x1_right_offset = SIDELEN_STATION - WIDTH_LINE / 2.;
                y1_right_offset = SIDELEN_STATION - WIDTH_LINE / 2.;
                x2_right_offset = WIDTH_LINE / 2.;
                y2_right_offset = SIDELEN_STATION - WIDTH_LINE / 2.;
            } else if mx > 1. {
                x1_left_offset = WIDTH_LINE / 2.;
                y1_left_offset = WIDTH_LINE / 2.;
                x2_left_offset = WIDTH_LINE / 2.;
                y2_left_offset = SIDELEN_STATION - WIDTH_LINE / 2.;
                x1_right_offset = SIDELEN_STATION - WIDTH_LINE / 2.;
                y1_right_offset = WIDTH_LINE / 2.;
                x2_right_offset = SIDELEN_STATION - WIDTH_LINE / 2.;
                y2_right_offset = SIDELEN_STATION - WIDTH_LINE / 2.;
            } else if mx < -1. {
                x1_left_offset = WIDTH_LINE / 2.;
                y1_left_offset = SIDELEN_STATION - WIDTH_LINE / 2.;
                x2_left_offset = WIDTH_LINE / 2.;
                y2_left_offset = SIDELEN_STATION - WIDTH_LINE / 2.;
                x1_right_offset = SIDELEN_STATION - WIDTH_LINE / 2.;
                y1_right_offset = SIDELEN_STATION - WIDTH_LINE / 2.;
                x2_right_offset = WIDTH_LINE / 2.;
                y2_right_offset = WIDTH_LINE / 2.;
            } else {
                x1_left_offset = SIDELEN_STATION - WIDTH_LINE / 2.;
                y1_left_offset = WIDTH_LINE / 2.;
                x2_left_offset = WIDTH_LINE / 2.;
                y2_left_offset = WIDTH_LINE / 2.;
                x1_right_offset = SIDELEN_STATION - WIDTH_LINE / 2.;
                y1_right_offset = SIDELEN_STATION - WIDTH_LINE / 2.;
                x2_right_offset = WIDTH_LINE / 2.;
                y2_right_offset = SIDELEN_STATION - WIDTH_LINE / 2.;
            }

            let left_line = graphics::Mesh::new_line(
                ctx,
                &[
                    [x1_real + x1_left_offset, y1_real + y1_left_offset],
                    [x2_real + x2_left_offset, y2_real + y2_left_offset],
                ],
                WIDTH_LINE,
                color,
            )?;

            let right_line = graphics::Mesh::new_line(
                ctx,
                &[
                    [x1_real + x1_right_offset, y1_real + y1_right_offset],
                    [x2_real + x2_right_offset, y2_real + y2_right_offset],
                ],
                WIDTH_LINE,
                color,
            )?;

            res = graphics::draw(ctx, &left_line, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
            res = graphics::draw(ctx, &right_line, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
        }
        res
    }
}

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
