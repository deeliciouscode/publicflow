use crate::metrics::traits::Metrics;

#[derive(Clone, Debug, Default)]
pub struct StationMetrics {
    pub number_of_pods: f32,
    pub time_in_station: f32,
    pub time_in_pods: f32,
    pub meters_traveled: f32,
}

// values should be a float to calculate averages more accurately

impl StationMetrics {
    pub fn new() -> StationMetrics {
        StationMetrics {
            number_of_pods: 0.,
            time_in_station: 0.,
            time_in_pods: 0.,
            meters_traveled: 0.,
        }
    }

    pub fn increase_number_of_pods(&mut self) {
        self.number_of_pods += 1.;
    }

    pub fn increase_time_in_station(&mut self) {
        self.time_in_station += 1.;
    }

    pub fn increase_time_in_pods(&mut self) {
        self.time_in_pods += 1.;
    }

    pub fn increase_meters_traveled(&mut self, meters: f32) {
        self.meters_traveled += meters;
    }
}

impl Metrics for StationMetrics {
    fn add_metrics(&mut self, other: &StationMetrics) {
        self.number_of_pods += other.number_of_pods;
        self.time_in_station += other.time_in_station;
        self.time_in_pods += other.time_in_pods;
        self.meters_traveled += other.meters_traveled;
    }

    fn normalize_by(&mut self, number_of_people: u32) {
        self.number_of_pods /= number_of_people as f32;
        self.time_in_station /= number_of_people as f32;
        self.time_in_pods /= number_of_people as f32;
        self.meters_traveled /= number_of_people as f32;
    }

    fn format_to_string(&self) -> String {
        format!(
            "{},{},{},{}",
            self.number_of_pods, self.time_in_station, self.time_in_pods, self.meters_traveled
        )
    }
}
