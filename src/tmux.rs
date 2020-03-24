use super::session::Session;
use super::session_manager::*;
use std::iter::Iterator;
use tmux_interface::TmuxInterface;
use tmux_interface::{
    clients_and_sessions::{NewSession, SwitchClient},
    NewWindow, SplitWindow,
};

pub fn get_sessions() -> Vec<Session> {
    let mut tmux = TmuxInterface::new();
    let sessions = match tmux.list_sessions(Some("#{session_name}")) {
        Ok(s) => s,
        Err(_) => panic!("Couldn't list sessions"), // TODO: Handle the error better
    };

    sessions
        .lines()
        .map(|x| Session::new(x.to_string(), false))
        .collect()
}

pub fn new_session(name: &String, attach: bool) {
    let mut tmux = TmuxInterface::new();

    if let Some(session) = read_session(name) {
        debug!("Creating session from template: {:?}", session);
        let window = session.windows.first().unwrap();
        let wd = get_wd_path(&window.panes.first().unwrap().directory);
        create_session(&mut tmux, name, attach, Some(wd.as_str()));
        setup_panes(&mut tmux, &name, window.panes.iter().skip(1));
        for window in session.windows.iter().skip(1) {
            create_window(&mut tmux, name, &window);
        }
    } else {
        create_session(&mut tmux, name, attach, None)
    }
}

fn create_session(tmux: &mut TmuxInterface, name: &String, attach: bool, cwd: Option<&str>) {
    let mut options = NewSession::new();
    options.detached = Some(true);
    options.session_name = Some(name.as_str());
    options.cwd = cwd;

    match tmux.new_session(Some(&options)) {
        Ok(_) => info!("Created new session: {}", name),
        Err(e) => eprintln!("Failed to create new session due to error: {}", e),
    }
    if attach {
        attach_session(name);
    }
}

fn create_window(tmux: &mut TmuxInterface, session: &String, window: &Window) {
    let session = format!("{}:", session);

    let first_pane = window.panes.first().unwrap();
    let wd = get_wd_path(&first_pane.directory);
    let mut new_window = NewWindow::new();
    new_window.cwd = Some(wd.as_str());
    new_window.target_window = Some(&session);

    debug!("New window opts: {:?}", new_window);
    match tmux.new_window(Some(&new_window)) {
        Ok(_) => info!("Created new window: {}", first_pane),
        Err(e) => error!("Failed to create a new window due to error: {}", e),
    };

    setup_panes(tmux, &session, window.panes.iter().skip(1));
}

fn setup_panes<'a, I>(tmux: &mut TmuxInterface, session: &String, panes: I)
where
    I: Iterator<Item = &'a Pane>,
{
    for pane in panes {
        let wd = get_wd_path(&pane.directory);
        let mut split_window = SplitWindow::new();
        split_window.cwd = Some(wd.as_str());
        split_window.target_pane = Some(&session);

        match &pane.split {
            Some(s) => match s.as_str() {
                "vertical" => split_window.horizontal = Some(false),
                "horizontal" => split_window.horizontal = Some(true),
                _ => warn!("Invalid value {} for split", s),
            },
            None => split_window.horizontal = Some(false),
        };
        debug!("split window opts: {:?}", split_window);
        match tmux.split_window(Some(&split_window)) {
            Ok(_) => info!("Created pane split for configuration: {}", pane),
            Err(e) => error!("Failed to split window due to error: {}", e),
        };
    }
}

fn get_wd_path(path: &String) -> String {
    match shellexpand::full(path.as_str()) {
        Ok(s) => {
            debug!("Expanded path: {}, to {}", path, s);
            s.into_owned()
        }
        Err(e) => {
            warn!(
                "Failed to perform shell expansion on path: {}, error {}",
                path, e
            );
            path.clone()
        }
    }
}

pub fn attach_session(name: &String) {
    let mut tmux = TmuxInterface::new();
    let mut options = SwitchClient::new();
    options.target_session = Some(name.as_str());

    match tmux.switch_client(Some(&options)) {
        Ok(_) => info!("Switched to session: {}", name),
        Err(e) => error!("Failed to attach session due to error: {}", e),
    }
}

pub fn delete_session(name: &String) {
    let mut tmux = TmuxInterface::new();
    match tmux.kill_session(None, None, Some(name.as_str())) {
        Ok(_) => info!("Deleted session: {}", name),
        Err(e) => error!("Failed to delete session due to error: {}", e),
    };
}
