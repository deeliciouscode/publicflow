use crate::control::action::Action;
use std::str::FromStr;
use std::time::Duration;

pub fn parse_sleep(input_list: &Vec<&str>) -> Vec<Action> {
    let mut effect_actions: Vec<Action> = vec![];
    let seconds = FromStr::from_str(input_list[1]);
    match seconds {
        Ok(seconds) => {
            effect_actions = vec![Action::Sleep {
                duration: Duration::from_secs(seconds),
            }];
        }
        Err(e) => {
            println!("sleeps argument has to be a (positive) number: {e}");
        }
    }

    return effect_actions;
}
