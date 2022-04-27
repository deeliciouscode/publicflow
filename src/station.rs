use std::collections::HashSet;

// TODO: Maybe define capacity of HashSet when using it (for performance)
#[derive(Clone, Debug)]
pub struct Station {
    pub id: i32,
    pub since_last_pod: i32,
    pub edges_to: HashSet<i32>,
    pub pods_in_station: HashSet<i32>,
}

impl Station {
    pub fn register_pod(&mut self, pod_id: i32) {
        self.pods_in_station.insert(pod_id);
    }

    pub fn deregister_pod(&mut self, pod_id: i32) {
        self.pods_in_station.remove(&pod_id);
    }

    pub fn get_pods_in_station_as_vec(&mut self) -> Option<Vec<i32>> {
        if self.pods_in_station.is_empty() {
            return None;
        }
        Some(self.pods_in_station.clone().into_iter().collect())
    }
}
