use crate::config::{Config, DESIRED_FPS};
use crate::connection::Connection;
use crate::line::{Line, LineState};
use crate::network::Network;
use crate::person::{PeopleBox, Person};
use crate::pod::{Pod, PodsBox};
use crate::station::Station;
use ggez::event::EventHandler;
use ggez::graphics::{self, Color};
use ggez::{timer, Context, GameResult};
use rand::Rng;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct State {
    pub network: Network,
    pub pods_box: PodsBox,
    pub people_box: PeopleBox,
    pub time_passed: u32,
}

impl State {
    pub fn print_state(&self) {
        self.network.print_state();
        self.pods_box.print_state();
        self.people_box.print_state();
    }

    pub fn update(&mut self) {
        for station in &mut self.network.stations {
            station.since_last_pod += 1;
        }

        for pod in &mut self.pods_box.pods {
            pod.update_state(&mut self.network)
        }

        for person in &mut self.people_box.people {
            person.update_state(&mut self.pods_box, &mut self.network);
        }
    }

    pub fn new(config: &Config) -> Self {
        let mut stations: Vec<Station> = vec![];
        for station_id in 0..config.network.n_stations {
            // unwrap can panic, maybe do pattern matching instead??
            let (coords_x, coords_y) = config.network.coordinates_map.get(&station_id).unwrap();
            stations.push(Station {
                id: station_id,
                since_last_pod: 0,
                edges_to: config.network.edge_map.get(&station_id).unwrap().clone(),
                pods_in_station: HashSet::from([]), // The pods will register themselves later
                coordinates: (*coords_x as f32, *coords_y as f32),
            })
        }

        let network = Network {
            stations: stations,
            lines: config.network.lines.clone(),
        };

        // let mut stations_occupied: Vec<i32> = vec![];
        let calc_line_state = |pod_id: &i32| -> LineState {
            let mut rng = rand::thread_rng();
            let mut n_stations_skipped = 0;
            // default, needed so Line can never be nothing
            let mut line: Line = Line {
                stations: vec![],
                circular: true,
                connections: vec![],
            };
            let mut line_ix: i32 = -1;
            // let mut station_id: i32 = -1;
            let mut direction: i32 = 1;

            for lineref in &config.network.lines {
                // println!("{}, {}", pod_id, n_stations_skipped);
                if *pod_id > n_stations_skipped + (lineref.stations.len() as i32 - 1) {
                    n_stations_skipped += lineref.stations.len() as i32;
                    continue;
                }

                line_ix = pod_id - n_stations_skipped;
                line = lineref.clone();
                // station_id = lineref.stations[line_ix as usize];
                direction = if rng.gen_bool(0.5) { 1 } else { -1 };
                break;
            }

            if line.stations.is_empty() {
                panic!("Something went wrong, stations should not be empty. Probably the number of pods does not match the expected number.")
            }

            let line_state = LineState {
                line: line,
                line_ix: line_ix,
                next_ix: -1,
                direction: direction,
            };

            // println!("-------------> {:?}", line_state);

            return line_state;
        };

        let mut people: Vec<Person> = vec![];
        for person_id in 0..config.people.n_people {
            people.push(Person::new(person_id, 10, &network));
        }

        let people_box = PeopleBox { people: people };

        let mut pods: Vec<Pod> = vec![];
        for pod_id in 0..config.network.pods.n_pods {
            pods.push(Pod::new(pod_id, 10, 5, calc_line_state(&pod_id)));
        }

        let pods_box = PodsBox { pods: pods };

        let state = State {
            network: network,
            people_box: people_box,
            pods_box: pods_box,
            time_passed: 0,
        };

        // println!("{:?}", state);
        // println!("conns: {:?}", state.network.lines);
        return state;
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        while timer::check_update_time(ctx, DESIRED_FPS) {
            // println!("fps: {}", timer::fps(ctx));
            self.time_passed += 1;
            self.update();

            if self.time_passed % 25 == 0 {
                // println!("-------------------------------");
                // println!("time passed: {}", self.time_passed);
                // self.print_state();
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);

        self.network.draw(ctx);
        self.pods_box.draw(ctx, &self.network);
        self.people_box.draw(ctx, &self.network);

        graphics::present(ctx)
    }
}

// simplest possible dummy state - for debugging purposes
pub fn _get_dummy_state() -> State {
    let one = Station {
        id: 0,
        since_last_pod: 0,
        edges_to: HashSet::from([1]),
        pods_in_station: HashSet::from([0]),
        coordinates: (1., 1.),
    };
    let two = Station {
        id: 1,
        since_last_pod: 0,
        edges_to: HashSet::from([0, 2]),
        pods_in_station: HashSet::new(),
        coordinates: (2.0, 1.),
    };
    let three = Station {
        id: 2,
        since_last_pod: 0,
        edges_to: HashSet::from([1]),
        pods_in_station: HashSet::new(),
        coordinates: (3., 1.),
    };

    let conn01 = Connection {
        station_ids: HashSet::from([0, 1]),
        travel_time: 20,
    };

    let conn12 = Connection {
        station_ids: HashSet::from([1, 2]),
        travel_time: 20,
    };

    let lines = vec![Line {
        stations: vec![0, 1, 2],
        circular: true,
        connections: vec![conn01, conn12],
    }];

    let network = Network {
        stations: vec![one, two, three],
        lines: lines.clone(),
    };

    let line_state = LineState {
        line: lines[0].clone(),
        line_ix: 0,
        next_ix: 1,
        direction: 1,
    };

    let pod = Pod::new(0, 10, 10, line_state);

    let person = Person::new(0, 10, &network);

    let people_box = PeopleBox {
        people: vec![person],
    };

    let pods_box = PodsBox { pods: vec![pod] };

    let state = State {
        network: network,
        people_box: people_box,
        pods_box: pods_box,
        time_passed: 0,
    };

    state.clone()
}
