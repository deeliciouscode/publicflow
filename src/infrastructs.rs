use std::collections::HashSet;

pub struct Station {
    pub id: i32,
    pub since_last_pod: i32,
    pub edges_to: Vec<i32>,
}

pub struct Connection {
    pub station_ids: HashSet<i32>,
    pub travel_time: i32,
}

pub struct Network {
    pub stations: Vec<Station>,
    pub connections: Vec<Connection>,
    pub pods: Vec<Pod>
}

impl Network {
    pub fn get_station_by_id(&self, id: i32) -> &Station {
        for station in  &self.stations {
            if station.id == id {
                return station
            } 
        }
        panic!("no station with this id found in network.")
    }

    pub fn get_connection(&self, fst: i32, snd: i32) -> &Connection {
        for connection in &self.connections {
            if connection.station_ids == HashSet::from([fst,snd]) {
                return connection
            }
        } 
        panic!("no station with this id found in network.")
    }

}

// TODO: implement destinations
pub struct Person {
    pub is_moving: bool,
    pub pod_id: i32,
    pub station_id: i32,
}

pub struct State {
    pub network: Network,
    pub people: Vec<Person>,
}

pub struct Pod {
    id: i32,
    capacity: i32,
    line: Vec<i32>,
    direction: i32,
    in_station: bool,
    in_station_since: i32,
    in_station_for: i32,
    station_id: i32
}

pub fn get_state() -> State {
    let mut one = Station {
        id: 0,
        since_last_pod: 0,
        edges_to: vec![1],
    };
    let mut two = Station {
        id: 1,
        since_last_pod: 0,
        edges_to: vec![0, 2],
    };
    let mut three = Station {
        id: 2,
        since_last_pod: 0,
        edges_to: vec![1],
    };

    let conn01 = Connection {
        station_ids: HashSet::from([0,1]),
        travel_time: 120,
    };

    let conn12 = Connection {
        station_ids: HashSet::from([1,2]),
        travel_time: 120,
    };

    let mut network = Network {
        stations: vec![one, two, three],
        connections: vec![conn01, conn12],
        pods: vec![]
    };

    let mut person = Person {
        is_moving: false,
        pod_id: -1,
        station_id: 0,
    };

    let state = State {
        network: network,
        people: vec![person],
    };

    state
}