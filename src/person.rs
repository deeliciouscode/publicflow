// TODO: implement destinations
pub struct Person {
    pub in_station_since: i32,
    pub pod_id: i32,
    pub station_id: i32,
    pub transition_time: i32,
}

impl Person {
    pub fn take_pod(&mut self, pod_id: i32) {
        self.pod_id = pod_id;
        self.station_id = -1;
    }
}
