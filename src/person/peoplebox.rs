use crate::config::structs::Config;
use crate::control::action::Action;
use crate::network::Network;
use crate::person::person::Person;
use crate::pod::podsbox::PodsBox;
use ggez::Context;
use rand::rngs::mock::StepRng;
// use shuffle::irs::Irs; // Turned out to slow down execution too much
use shuffle::fy::FisherYates;
use shuffle::shuffler::Shuffler;
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

    pub fn update(&mut self, pods_box: &mut PodsBox, network: &mut Network, config: &Config) {
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
            person.update(pods_box, network, config);
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
                person.action_on_arrival = Some(action.clone())
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
}
