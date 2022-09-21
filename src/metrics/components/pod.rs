use crate::metrics::traits::Metrics;

#[derive(Clone, Debug, Default)]
pub struct PodMetrics {
    pub utilization: f32,
    pub time_in_station: f32,
    pub time_in_queue: f32,
    pub time_driving: f32,
    pub meters_traveled: f32,
}

// values should be a float to calculate averages more accurately

impl PodMetrics {
    pub fn new() -> PodMetrics {
        PodMetrics {
            utilization: 0.,
            time_in_station: 0.,
            time_in_queue: 0.,
            time_driving: 0.,
            meters_traveled: 0.,
        }
    }

    pub fn set_utilization(&mut self, utilization: f32) {
        self.utilization = utilization;
    }

    pub fn increase_time_in_station(&mut self) {
        self.time_in_station += 1.;
    }

    pub fn increase_time_in_queue(&mut self) {
        self.time_in_queue += 1.;
    }

    pub fn increase_time_driving(&mut self) {
        self.time_driving += 1.;
    }

    pub fn increase_meters_traveled(&mut self, meters: f32) {
        self.meters_traveled += meters;
    }
}

impl Metrics for PodMetrics {
    fn add_metrics(&mut self, other: &PodMetrics) {
        self.utilization += other.utilization;
        self.time_in_station += other.time_in_station;
        self.time_in_queue += other.time_in_queue;
        self.time_driving += other.time_driving;
        self.meters_traveled += other.meters_traveled;
    }

    fn normalize_by(&mut self, number_of_pods: u32) {
        self.utilization /= number_of_pods as f32;
        self.time_in_station /= number_of_pods as f32;
        self.time_in_queue /= number_of_pods as f32;
        self.time_in_station /= number_of_pods as f32;
        self.meters_traveled /= number_of_pods as f32;
    }

    fn format_to_string(&self) -> String {
        format!(
            "{},{},{},{},{}",
            self.utilization,
            self.time_in_station,
            self.time_in_queue,
            self.time_driving,
            self.meters_traveled
        )
    }
}
