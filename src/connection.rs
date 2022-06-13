use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Connection {
    pub station_ids: HashSet<i32>,
    pub travel_time: i32,
}

impl YieldTuple<i32> for Connection {
    fn yield_tuple(&self) -> (i32, i32) {
        let mut tuple = (0, 0);
        for (i, x) in self.station_ids.iter().enumerate() {
            if i > 1 {
                println!("Connection contains more than 2 elements");
                break;
            }
            if i == 0 {
                tuple.0 = *x;
            }
            if i == 1 {
                tuple.1 = *x;
            }
        }
        return tuple;
    }
}

impl YieldTuple<u32> for Connection {
    fn yield_tuple(&self) -> (u32, u32) {
        let mut tuple = (0, 0);
        for (i, x) in self.station_ids.iter().enumerate() {
            if i > 1 {
                println!("Connection contains more than 2 elements");
                break;
            }
            if i == 0 {
                tuple.0 = *x as u32;
            }
            if i == 1 {
                tuple.1 = *x as u32;
            }
        }
        return tuple;
    }
}

pub trait YieldTuple<T> {
    fn yield_tuple(&self) -> (T, T);
}
