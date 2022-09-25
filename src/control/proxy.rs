use crate::control::action::{Action, Actions};
use std::sync::mpsc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

pub fn run_proxy(rx: mpsc::Receiver<Actions>, tx: mpsc::Sender<Actions>) {
    let mut action_buffer = Actions::new();
    let mut loop_buffer = Actions::new();
    let mut conc_buffer = Actions::new();
    let mut in_loop = false;
    let mut in_conc = false;
    let mut loop_n = 1;
    loop {
        sleep(Duration::from_millis(10)); // This doesn't make a difference in terms of use but saves the thread some work
        let actions = recv_queries(&rx);
        for action in actions.actions {
            match action {
                Action::Loop { n } => {
                    send_actions(tx.clone(), action_buffer.clone());
                    action_buffer = Actions::new();
                    loop_buffer = Actions::new();
                    in_loop = true;
                    loop_n = n;
                }
                Action::Endloop => {
                    // let mut new_action_buffer = Actions::new();
                    if in_conc {
                        for _ in 0..loop_n {
                            conc_buffer.actions.extend(loop_buffer.actions.clone());
                        }
                    } else {
                        for _ in 0..loop_n {
                            action_buffer.actions.extend(loop_buffer.actions.clone());
                        }
                        // send_actions(tx.clone(), action_buffer.clone());
                    }
                    // println!("conc_buffer endloop: {:?}", &conc_buffer);
                    // println!("loop_buffer endloop: {:?}", &loop_buffer);
                    // println!("loop_n: {}", loop_n);
                    // action_buffer = Actions::new();
                    loop_buffer = Actions::new();
                    in_loop = false;
                    loop_n = 1;
                }
                Action::StartConcurency => {
                    // send_actions(tx.clone(), action_buffer.clone());
                    action_buffer = Actions::new();
                    conc_buffer = Actions::new();
                    in_conc = true;
                }
                Action::DoConcurrently => {
                    if in_conc {
                        println!("conc_buffer: {:?}", &conc_buffer);
                        send_actions_conc(tx.clone(), conc_buffer);
                        conc_buffer = Actions::new();
                    }
                }
                Action::EndConcurency => {
                    if in_conc {
                        send_actions_conc(tx.clone(), conc_buffer);
                        conc_buffer = Actions::new();
                        in_conc = false;
                    }
                }
                action => {
                    if in_loop {
                        loop_buffer.actions.push(action);
                    } else if in_conc {
                        conc_buffer.actions.push(action);
                    } else {
                        action_buffer.actions.push(action);
                    }
                }
            }
        }
        send_actions(tx.clone(), action_buffer.clone());
        action_buffer = Actions::new();
    }
}

fn send_actions_conc(tx: mpsc::Sender<Actions>, actions: Actions) {
    thread::spawn(|| send_actions(tx, actions));
}

fn send_actions(tx: mpsc::Sender<Actions>, actions: Actions) {
    let mut action_buffer = Actions::new();
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
