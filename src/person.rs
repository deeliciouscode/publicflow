// TODO: implement destinations
#[derive(Clone, Debug)]
pub struct PeopleBox {
    pub people: Vec<Person>,
}

#[derive(Clone, Debug)]
pub struct Person {
    pub id: i32,
    pub in_station_since: i32,
    pub pod_id: i32,
    pub station_id: i32,
    pub last_station_id: i32,
    pub transition_time: i32,
}

impl Person {
    pub fn take_pod(&mut self, pod_id: i32) {
        self.pod_id = pod_id;
        self.last_station_id = self.station_id;
        self.station_id = -1;
    }

    pub fn get_off_pod(&mut self, station_id: i32) {
        self.station_id = station_id;
        self.in_station_since = 0;
        self.pod_id = -1;
    }
}

// #[derive(Clone, Debug)]
// pub struct PersonSM {
//     pub id: i32,
//     pub transition_time: i32,
//     state: PersonState,
// }

// #[derive(Debug, Clone, Copy)]
// enum PersonState {
//     ReadyToTakePod { station_id: i32 },
//     RidingPod { pod_id: i32 },
//     TransitioningToNewPod { station_id: i32, previous_pod_id: i32, time_in_station: i32 },
// }


// Person State Machine:

// ReadyToTakePod ---> RidingTrain ---> TransitioningToNewPod
//      ^                    ^                    |       |
//      |                    |--------------------|       |
//      |-------------------------------------------------|

