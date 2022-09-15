use crate::metrics::person::timestamp::Timespamp;

#[derive(Clone, Debug, Default)]
pub struct PersonMetrics {
    pub number_of_pods: u32,
    pub time_in_station: u32,
    pub time_in_pods: u32,
    pub meters_traveled: u32,
}

impl PersonMetrics {
    pub fn new() -> PersonMetrics {
        PersonMetrics {
            number_of_pods: 0,
            time_in_station: 0,
            time_in_pods: 0,
            meters_traveled: 0,
        }
    }
}
