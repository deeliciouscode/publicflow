use crate::control::action::{Action, Actions};
use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;

pub fn run_emmiter(rx: mpsc::Receiver<Actions>, tx: mpsc::Sender<Actions>) {
    let mut action_buffer = Actions::new();
    loop {
        sleep(Duration::from_millis(10)); // This doesn't make a difference in terms of use but saves the thread some work
        let actions = recv_queries(&rx);
        for action in actions.actions {
            match action {
                Action::Sleep { duration } => {
                    if !action_buffer.actions.is_empty() {
                        let _res = tx.send(action_buffer.clone());
                        action_buffer = Actions::new();
                    }
                    println!("commit actions, then sleep for {:?}", duration);
                    sleep(duration);
                }
                Action::KillSimulation { code: _ } => {
                    action_buffer.actions.push(action);
                    let _res = tx.send(action_buffer.clone());
                    break;
                }
                action => {
                    action_buffer.actions.push(action);
                }
            }
        }
        if !action_buffer.actions.is_empty() {
            let _res = tx.send(action_buffer.clone());
            action_buffer = Actions::new();
        }
    }
}

pub fn recv_queries(rx: &mpsc::Receiver<Actions>) -> Actions {
    let maybe_received = rx.try_recv();
    match maybe_received {
        Ok(received) => {
            return received;
        }
        Err(_) => {}
    }
    return Actions::new();
}

pub fn recv_actions(rx: &mpsc::Receiver<Actions>) -> Actions {
    let maybe_received = rx.try_recv();
    match maybe_received {
        Ok(received) => {
            return received;
        }
        Err(_) => {}
    }
    return Actions::new();
}
