use crate::config::structs::Config;
use crate::control::action::Action;
use std::str::FromStr;
use std::time::Duration;

pub fn parse_sleep(input_list: &Vec<&str>, config: &Config) -> Vec<Action> {
    let mut actions: Vec<Action> = vec![];
    if input_list.len() < 3 {
        println!("sleep how long in which mode??");
        return actions;
    }

    let timing = input_list[2];
    let seconds = FromStr::from_str(input_list[1]);

    match seconds {
        Ok(seconds) => {
            if timing == "real" {
                actions = vec![Action::Sleep {
                    duration: Duration::from_secs(seconds),
                }];
            } else if timing == "sim" {
                actions = vec![Action::Sleep {
                    duration: Duration::from_millis(
                        (seconds * 1000) / config.logic.speed_multiplier as u64,
                    ),
                }];
            }
        }
        Err(e) => {
            println!("sleeps argument has to be a (positive) number: {e}");
        }
    }

    return actions;
}
