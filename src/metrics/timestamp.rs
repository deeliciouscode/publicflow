use crate::metrics::components::person::PersonMetrics;
use crate::metrics::components::pod::PodMetrics;
use crate::metrics::traits::Metrics;

#[derive(Clone, Debug)]
pub struct Timespamp<T: Metrics> {
    pub ts: u32,
    pub metrics: T,
}

impl<T: Metrics> Timespamp<T> {
    pub fn new(ts: u32, metrics: T) -> Timespamp<T> {
        Timespamp {
            ts: ts,
            metrics: metrics,
        }
    }
}

impl<T: Metrics> Metrics for Timespamp<T> {
    fn add_metrics(&mut self, other: &Self) {
        self.metrics.add_metrics(&other.metrics);
    }

    fn normalize_by(&mut self, number_of_people: u32) {
        self.metrics.normalize_by(number_of_people)
    }

    fn format_to_string(&self) -> String {
        format!("{},{}", self.ts, self.metrics.format_to_string())
    }
}
