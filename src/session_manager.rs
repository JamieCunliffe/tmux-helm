use serde::{Deserialize, Serialize};
use std::fmt::Display;
use crate::config::Config;


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
    #[serde(default = "default_dir")]
    pub directory: String,
    #[serde(default = "default_split")]
    pub split: Split,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Split {
    #[serde(rename = "vertical")]
    Vertical,
    #[serde(rename = "horizontal")]
    Horizontal,
}

impl Display for Pane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            r#"Pane configuraxtion:
	Directory: {}
	Split: {}"#,
            self.directory, self.split
        )
    }
}

impl Display for Split {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Split::Vertical => "vertical",
                Split::Horizontal => "horizontal",
            }
        )
    }
}

pub fn read_session(session_name: &String, config: &Config) -> Option<Session> {
    debug!("Reading sessions");

    for session_file in &config.session_files {
        let contents = match std::fs::read_to_string(&session_file) {
            Ok(a) => a,
            Err(e) => {
                error!("Error reading session file ({}) : {}", session_file, e);
                return None;
            }
        };
        let session: Root = serde_json::from_str(&contents).unwrap();

        match session.session.iter().find(|x| &x.name == session_name) {
            Some(s) => return Some(s.clone()),
            None => (),
        };
    }

    None
}

pub fn default_dir() -> String {
    String::from("~/")
}

pub fn default_split() -> Split {
    Split::Vertical
}
