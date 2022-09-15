use crate::metrics::person::metrics::PersonMetrics;
use crate::metrics::person::timestamp::Timespamp;

#[derive(Clone, Debug, Default)]
pub struct PersonTimeSeries {
    pub time_series: Vec<Timespamp>,
}

impl PersonTimeSeries {
    pub fn new() -> PersonTimeSeries {
        PersonTimeSeries {
            time_series: Vec::new(),
        }
    }

    pub fn add_timestamp(&mut self, ts: u32, metrics: PersonMetrics) {
        self.time_series.push(Timespamp::new(ts, metrics))
    }
}
