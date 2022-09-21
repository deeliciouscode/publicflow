use crate::metrics::traits::Metrics;

#[derive(Clone, Debug, Default)]
pub struct Timestamp<T: Metrics> {
    pub ts: u32,
    pub metrics: T,
}

impl<T: Metrics> Timestamp<T> {
    pub fn new(ts: u32, metrics: T) -> Timestamp<T> {
        Timestamp {
            ts: ts,
            metrics: metrics,
        }
    }

    pub fn dummy(ts: u32) -> Timestamp<T> {
        Timestamp {
            ts: ts,
            metrics: T::default(),
        }
    }
}

impl<T: Metrics> Metrics for Timestamp<T> {
    fn add_metrics(&mut self, other: &Self) {
        self.metrics.add_metrics(&other.metrics);
    }

    fn normalize_by(&mut self, n: u32) {
        self.metrics.normalize_by(n)
    }

    fn format_to_string(&self) -> String {
        format!("{},{}", self.ts, self.metrics.format_to_string())
    }
}
