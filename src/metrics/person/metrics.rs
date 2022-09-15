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

    pub fn increase_time_in_station(&mut self) {
        self.time_in_station += 1;
    }

    pub fn increase_time_in_pods(&mut self) {
        self.time_in_pods += 1;
    }

    pub fn increase_number_of_pods(&mut self) {
        self.number_of_pods += 1;
    }

    pub fn increase_meters_traveled(&mut self, meters: u32) {
        self.meters_traveled += meters;
    }
}
