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

    pub fn format_to_string(&self) -> String {
        let mut txt =
            String::from("ts,number_of_pods,time_in_station,time_in_pods,meters_traveled\n");
        for ts in &self.time_series {
            txt.push_str(&format!("{}\n", ts.format_to_string()))
        }
        txt
    }

    pub fn add_layer(&mut self, other: &Self) {
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

    pub fn normalize_by(&mut self, number_of_people: u32) {
        for ts in &mut self.time_series {
            ts.normalize_by(number_of_people)
        }
    }
}
