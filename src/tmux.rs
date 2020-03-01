use tmux_interface::TmuxInterface;
use tmux_interface::clients_and_sessions::{SwitchClient, NewSession};

use super::session::Session;

pub fn get_sessions() -> Vec<Session> {
    let mut tmux = TmuxInterface::new();
    let sessions = match tmux.list_sessions(Some("#{session_name}")) {
        Ok(s) => s,
        Err(_) => panic!("Couldn't list sessions"), // TODO: Handle the error better
    };

    sessions.lines().map(|x| {
        Session::new(x.to_string(), false)
    }).collect()
}

pub fn new_session(name: &String, attach: bool) {
    let mut tmux = TmuxInterface::new();
    let mut options = NewSession::new();
    options.detached = Some(true);
    options.session_name = Some(name.as_str());
    match tmux.new_session(Some(&options)) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e)
    }
    if attach {
        attach_session(name);
    }
}

pub fn attach_session(name: &String) {
    let mut tmux = TmuxInterface::new();
    let mut options = SwitchClient::new();
    options.target_session = Some(name.as_str());

    match tmux.switch_client(Some(&options)) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e)
    }
}

pub fn delete_session(name: &String) {
    let mut tmux = TmuxInterface::new();
    match tmux.kill_session(None, None, Some(name.as_str())) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e)
    };
}
