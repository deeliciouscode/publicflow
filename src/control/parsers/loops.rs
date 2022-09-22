use crate::control::action::Action;
use std::str::FromStr;

pub fn parse_loop(input_list: &Vec<&str>) -> Vec<Action> {
    let mut actions: Vec<Action> = vec![];
    if input_list.len() < 2 {
        println!("loop how often??");
        return actions;
    }

    let ntimes = FromStr::from_str(input_list[1]);

    match ntimes {
        Ok(ntimes) => actions = vec![Action::Loop { n: ntimes }],
        Err(e) => {
            println!("loops argument has to be a (positive) number: {e}");
        }
    }

    return actions;
}
