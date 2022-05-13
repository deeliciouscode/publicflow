use crate::config::{MAX_XY, OFFSET, SCREEN_SIZE, SIDELEN_STATION};
use ggez::graphics::Rect;
use ggez::{graphics, Context, GameResult};
use std::collections::HashSet;

// TODO: Maybe define capacity of HashSet when using it (for performance)
#[derive(Clone, Debug)]
pub struct Station {
    pub id: i32,
    pub since_last_pod: i32,
    pub edges_to: HashSet<i32>,
    pub pods_in_station: HashSet<i32>,
    pub coordinates: (f32, f32),
}

impl Station {
    pub fn register_pod(&mut self, pod_id: i32) {
        self.pods_in_station.insert(pod_id);
    }

    pub fn deregister_pod(&mut self, pod_id: i32) {
        self.pods_in_station.remove(&pod_id);
    }

    pub fn get_pod_ids_in_station_as_vec(&mut self) -> Option<Vec<i32>> {
        if self.pods_in_station.is_empty() {
            return None;
        }
        Some(self.pods_in_station.clone().into_iter().collect())
    }

    /// Note: this method of drawing does not scale. If you need to render
    /// a large number of shapes, use a SpriteBatch. This approach is fine for
    /// this example since there are a fairly limited number of calls.
    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        // First we set the color to draw with, in this case all food will be
        // colored blue.
        let color = [0.5, 0.5, 0.5, 1.0].into();
        // Then we draw a rectangle with the Fill draw mode, and we convert the
        // Food's position into a `ggez::Rect` using `.into()` which we can do
        // since we implemented `From<GridPosition>` for `Rect` earlier.

        let station_rect = Rect {
            x: OFFSET
                + (self.coordinates.0 / MAX_XY.0 * SCREEN_SIZE.0)
                    * ((SCREEN_SIZE.0 - 2.0 * OFFSET) / SCREEN_SIZE.0) as f32,
            y: OFFSET
                + (self.coordinates.1 / MAX_XY.1 * SCREEN_SIZE.1)
                    * ((SCREEN_SIZE.1 - 2.0 * OFFSET) / SCREEN_SIZE.1) as f32,
            w: SIDELEN_STATION,
            h: SIDELEN_STATION,
        };
        let rectangle =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), station_rect, color)?;
        graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))
    }
}
