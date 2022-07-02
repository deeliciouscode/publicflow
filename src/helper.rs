use crate::config::{OFFSET, SCREEN_SIZE};

pub fn get_real_coordinates(coordinates: (f32, f32)) -> (f32, f32) {
    let lat_min: f32 = 11.4606;
    let lat_max: f32 = 11.7036;
    let lat_delta: f32 = lat_max - lat_min;

    let lon_min: f32 = 48.0416;
    let lon_max: f32 = 48.2649;
    let lon_delta: f32 = lon_max - lon_min;

    let x_percentage = (coordinates.0 - lat_min) / lat_delta;
    let y_percentage = (coordinates.1 - lon_min) / lon_delta;

    let x = OFFSET
        + (x_percentage * SCREEN_SIZE.0) * ((SCREEN_SIZE.0 - 2.0 * OFFSET) / SCREEN_SIZE.0) as f32;

    let y = OFFSET
        + ((1. - y_percentage) * SCREEN_SIZE.1)
            * ((SCREEN_SIZE.1 - 2.0 * OFFSET) / SCREEN_SIZE.1) as f32;
    (x, y)
}
