use crate::config::{MAX_XY, OFFSET, SCREEN_SIZE, SIDELEN_STATION, WIDTH_LINE};
use crate::connection::{Connection, YieldTuple};
use crate::network::Network;
use ggez::graphics::Rect;
use ggez::{graphics, Context, GameResult};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Line {
    pub stations: Vec<i32>,
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

            let x_left: f32;
            let x_right: f32;
            let y_up: f32;
            let y_down: f32;

            if to.coordinates.0 > from.coordinates.0 {
                x_left = from.coordinates.0;
                x_right = to.coordinates.0;
            } else {
                x_left = to.coordinates.0;
                x_right = from.coordinates.0;
            }

            if to.coordinates.1 > from.coordinates.1 {
                y_up = from.coordinates.1;
                y_down = to.coordinates.1;
            } else {
                y_up = to.coordinates.1;
                y_down = from.coordinates.1;
            }

            let x = OFFSET
                + (x_left / MAX_XY.0 * SCREEN_SIZE.0)
                    * ((SCREEN_SIZE.0 - 2.0 * OFFSET) / SCREEN_SIZE.0) as f32;

            let y = OFFSET
                + (y_up / MAX_XY.1 * SCREEN_SIZE.1)
                    * ((SCREEN_SIZE.1 - 2.0 * OFFSET) / SCREEN_SIZE.1) as f32;

            let w = OFFSET
                + (x_right / MAX_XY.0 * SCREEN_SIZE.0)
                    * ((SCREEN_SIZE.0 - 2.0 * OFFSET) / SCREEN_SIZE.0) as f32
                - x
                + WIDTH_LINE;

            let h = OFFSET
                + (y_down / MAX_XY.1 * SCREEN_SIZE.1)
                    * ((SCREEN_SIZE.1 - 2.0 * OFFSET) / SCREEN_SIZE.1) as f32
                - y
                + WIDTH_LINE;

            let line_left_rect = Rect {
                x: x,
                y: y,
                w: w,
                h: h,
            };

            let left_rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                line_left_rect,
                color,
            )?;

            let x_right: f32;
            let y_right: f32;

            if h > w {
                x_right = x + SIDELEN_STATION - WIDTH_LINE;
                y_right = y;
            } else {
                x_right = x;
                y_right = y + SIDELEN_STATION - WIDTH_LINE;
            }

            let line_right_rect = Rect {
                x: x_right,
                y: y_right,
                w: w,
                h: h,
            };

            let right_rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                line_right_rect,
                color,
            )?;

            res = graphics::draw(ctx, &left_rect, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
            res = graphics::draw(ctx, &right_rect, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
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
