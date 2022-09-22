use crate::config::structs::Config;
use crate::control::action::Action;
use crate::network::Network;
use crate::person::person::Person;
use crate::pod::podsbox::PodsBox;
use ggez::Context;
use rand::rngs::mock::StepRng;
// use shuffle::irs::Irs; // Turned out to slow down execution too much
use crate::metrics::timeseries::TimeSeries;
use crate::metrics::traits::Series;
use shuffle::fy::FisherYates;
use shuffle::shuffler::Shuffler;
use std::fs::File;
use std::fs::*;
use std::io::prelude::*;
use std::path::Path;

// use crate::person::person::Person;

// TODO: implement destinations
#[derive(Clone, Debug)]
pub struct PeopleBox {
    pub people: Vec<Person>,
}

impl PeopleBox {
    pub fn try_get_person_by_id_unmut(&self, id: i32) -> Option<&Person> {
        for person in &self.people {
            if person.id == id {
                return Some(person);
            }
        }
        None
    }

    pub fn update(
        &mut self,
        pods_box: &mut PodsBox,
        network: &mut Network,
        config: &Config,
        time_passed: u32,
    ) {
        for person in &mut self.people {
            person.get_out_if_needed(pods_box, network, config);
        }

        // Only FisherYates is acceptable in terms of speed
        // Inverse Riffle Shuffeling would be more random but so much slower
        if config.logic.shuffle_people {
            let mut rng = StepRng::new(2, 10);
            let mut fy = FisherYates::default();
            let _res = fy.shuffle(&mut self.people, &mut rng);
        }

        for person in &mut self.people {
            person.update(pods_box, network, config, time_passed);
        }
    }

    // TODO: differentiate between follow and not
    pub fn apply_show_person(&mut self, id: i32, follow: bool) {
        for person in &mut self.people {
            if person.id == id {
                if follow {
                    person.visualize = true;
                } else {
                    person.visualize = true;
                }
            }
        }
    }

    pub fn apply_hide_person(&mut self, id: i32) {
        for person in &mut self.people {
            if person.id == id {
                person.visualize = false;
            }
        }
    }

    pub fn apply_route_person(&mut self, id: i32, action: Action) {
        for person in &mut self.people {
            if person.id == id {
                person.action_to_process = Some(action.clone())
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        for person in &self.people {
            if person.visualize {
                let _res = person.draw(ctx);
            }
        }
    }

    pub fn dump_metrics(&self, person_id: i32, config: &Config) {
        let maybe_person = self.try_get_person_by_id_unmut(person_id);
        match maybe_person {
            Some(person) => {
                if let Some(timestamp_run) = config.timestamp_run {
                    let timestamp = timestamp_run
                        .naive_utc()
                        .format("%Y.%m.%d %H:%M:%S")
                        .to_string()
                        .replace(" ", "_");
                    println!("timestamp_run: {:?}", timestamp);

                    let path_str =
                        format!("{}/{}/{}/{}.txt", "metrics", timestamp, "people", person_id);
                    let path = Path::new(&path_str);
                    let parent = path.parent().unwrap();
                    let _res = create_dir_all(parent);
                    let res = File::create(path);
                    match res {
                        Ok(mut file) => {
                            let txt = person.time_series.format_to_file(String::from(
                                "ts,number_of_pods,time_in_station,time_in_pods,meters_traveled\n",
                            ));
                            let res = file.write_all(txt.as_bytes());
                            match res {
                                Ok(_) => {
                                    println!("written file");
                                }
                                Err(e) => {
                                    println!("error writing file: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("error opening file: {}", e);
                        }
                    }
                }
            }
            None => {}
        }
    }

    pub fn dump_avg_metrics(&self, config: &Config) {
        let mut timeseries_accumulator = TimeSeries::new();
        for person in &self.people {
            timeseries_accumulator.add_layer(&person.time_series);
        }

        timeseries_accumulator.normalize_by(config.logic.number_of_people as u32);

        if let Some(timestamp_run) = config.timestamp_run {
            let timestamp = timestamp_run
                .naive_utc()
                .format("%Y.%m.%d %H:%M:%S")
                .to_string()
                .replace(" ", "_");
            println!("timestamp_run: {:?}", timestamp);

            let path_str = format!(
                "{}/{}/{}/{}/{}.txt",
                "metrics", config.environment, timestamp, "people", "avg"
            );
            let path = Path::new(&path_str);
            let parent = path.parent().unwrap();
            let _res = create_dir_all(parent);
            let res = File::create(path);
            match res {
                Ok(mut file) => {
                    let txt = timeseries_accumulator.format_to_file(String::from(
                        "ts,number_of_pods,time_in_station,time_in_pods,meters_traveled\n",
                    ));
                    let res = file.write_all(txt.as_bytes());
                    match res {
                        Ok(_) => {
                            println!("written file");
                        }
                        Err(e) => {
                            println!("error writing file: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("error opening file: {}", e);
                }
            }
        }
    }
}
