use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
struct Root {
    session: Vec<Session>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    name: String,
    pub windows: Vec<Window>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Window {
    pub name: Option<String>,
    pub panes: Vec<Pane>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pane {
    pub directory: String,
    pub split: Option<String>,
}

impl Display for Pane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Pane configuration:\n\tDirectory: {}\n\tSplit: {}", self.directory, match &self.split {
            Some(s) => match s.as_str() {
                "vertical" => "vertical",
                "horizontal" => "horizontal",
                _ => "UNKNOWN"
            }
            None => "vertical"
        }
        )
    }
}

pub fn read_session(session_name: &String) -> Option<Session> {
    debug!("Reading sessions");

    let contents = std::fs::read_to_string("sessions/config.json").unwrap();
    let session: Root = serde_json::from_str(&contents).unwrap();

    match session.session.iter().find(|x| &x.name == session_name) {
        Some(s) => Some(s.clone()),
        None => None,
    }
}
