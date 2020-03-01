use tmux_interface::TmuxInterface;

#[derive(Debug)]
pub struct Session {
    pub name: String
}

pub fn get_sessions(/* tmux: &mut TmuxInterface */) -> Vec<Session> {
    let mut tmux = TmuxInterface::new();
    let sessions = match tmux.list_sessions(Some("#{session_name}")) {
        Ok(s) => s,
        Err(_) => panic!("Couldn't list sessions"), // TODO: Handle the error better
    };

    sessions.lines().map(|x| {
        Session {
            name: x.to_string()
        }
    }).collect()
}
