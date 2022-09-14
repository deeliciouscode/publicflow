use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct PersonMetrics {
    // maps a timestamp to metrics tied to that point in time
    // e.g. time in station so far, time travveling so far etc
    //                              Ts    NoTr TSta TPod MGes
    pub timestamp_metrics_map: HashMap<i32, (i32, i32, i32, i32)>,
}

impl PersonMetrics {
    pub fn _new() -> PersonMetrics {
        PersonMetrics {
            timestamp_metrics_map: HashMap::new(),
        }
    }
}
