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

    pub fn add_metrics(&mut self, other: &Self) {
        self.metrics.add(&other.metrics);
    }

    pub fn normalize_by(&mut self, number_of_people: u32) {
        self.metrics.normalize_by(number_of_people)
    }

    pub fn format_to_string(&self) -> String {
        format!("{},{}", self.ts, self.metrics.format_to_string())
    }
}
