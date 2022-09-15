use crate::metrics::person::metrics::PersonMetrics;

#[derive(Clone, Debug)]
pub struct Timespamp {
    pub ts: u32,
    pub metrics: PersonMetrics,
}

impl Timespamp {
    pub fn new(ts: u32, metrics: PersonMetrics) -> Timespamp {
        Timespamp {
            ts: ts,
            metrics: metrics,
        }
    }
}
