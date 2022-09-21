use crate::metrics::components::person::PersonMetrics;
use crate::metrics::components::pod::PodMetrics;
use crate::metrics::timestamp::Timespamp;
use crate::metrics::traits::Metrics;
use crate::metrics::traits::Series;

#[derive(Clone, Debug, Default)]
pub struct TimeSeries<T: Metrics> {
    pub time_series: Vec<Timespamp<T>>,
}

impl<T: Metrics> TimeSeries<T> {
    pub fn new() -> TimeSeries<T> {
        TimeSeries {
            time_series: Vec::new(),
        }
    }

    pub fn add_timestamp(&mut self, ts: u32, metrics: T) {
        self.time_series.push(Timespamp::new(ts, metrics))
    }
}

impl<T: Metrics> Series for TimeSeries<T> {
    fn format_to_file(&self, header: String) -> String {
        let mut txt = header;
        for ts in &self.time_series {
            txt.push_str(&format!("{}\n", ts.format_to_string()))
        }
        txt
    }

    fn add_layer(&mut self, other: &Self) {
        if self.time_series.is_empty() {
            self.time_series = other.time_series.clone();
        } else {
            for i in 0..self.time_series.len() {
                let self_elem = &mut self.time_series[i];
                let other_elem = &other.time_series[i];
                if self_elem.ts != other_elem.ts {
                    panic!("Elements in same spot in vector have different ts - this should never be the case.")
                }
                self_elem.add_metrics(other_elem);
            }
        }
    }

    fn normalize_by(&mut self, number_of_people: u32) {
        for ts in &mut self.time_series {
            ts.normalize_by(number_of_people)
        }
    }
}
