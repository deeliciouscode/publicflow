use crate::config::structs::Config;
use crate::control::action::SetAction;
use crate::network::Network;
use crate::person::person::Person;
use crate::pod::podsbox::PodsBox;
use ggez::Context;
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
        set_actions: &Vec<SetAction>,
        config: &Config,
    ) {
        for action in set_actions {
            match action {
                // TODO: differentiate between follow and not
                SetAction::ShowPerson { id, follow } => {
                    for person in &mut self.people {
                        if person.id == *id {
                            if *follow {
                                person.visualize = true;
                            } else {
                                person.visualize = true;
                            }
                        }
                    }
                }
                SetAction::HidePerson { id } => {
                    for person in &mut self.people {
                        if person.id == *id {
                            person.visualize = false;
                        }
                    }
                }
                SetAction::RoutePerson {
                    id,
                    station_id: _,
                    random_station: _,
                } => {
                    for person in &mut self.people {
                        if person.id == *id {
                            person.action_on_arrival = Some(action.clone())
                        }
                    }
                }
                _ => {}
            }
        }
        for person in &mut self.people {
            person.get_out_if_needed(pods_box, network, config);
        }
        for person in &mut self.people {
            person.update(pods_box, network, config);
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
