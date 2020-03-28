use crate::utils::expand_path;
use serde::Deserialize;
use std::error::Error;
use std::fs::read_to_string;


#[derive(Deserialize, Debug)]
pub struct Config {
    pub session_files: Vec<String>,
    #[serde(default = "default_session_format")]
    pub session_format: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            session_files: vec![],
            session_format: default_session_format()
        }
    }
}

#[derive(Debug)]
pub struct ConfigFileError { }

impl std::fmt::Display for ConfigFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "ConfigFileError")
    }
}

impl Error for ConfigFileError {
    fn description(&self) -> &str {
        "Error finding config file"
    }
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

pub fn get_config() -> Config {
    match get_config_internal() {
        Ok(c) => c,
        Err(e) => {
            warn!("Failed to load config due to error: {}", e);
            Config::default()
        }
    }
}

fn get_config_internal() -> Result<Config, Box<dyn Error>> {
    let xdg = xdg::BaseDirectories::with_prefix("tmux-session")?;
    let config_file = match xdg.find_config_file("config.toml") {
        Some(f) => f.as_path().to_str().unwrap_or("").to_string(),
        None => {
            warn!("No configuration file was found");
            return Err(Box::new(ConfigFileError { }));
        }
    };

    let contents = read_to_string(expand_path(&config_file))?;

    let mut config: Config = toml::from_str(&contents)?;

    config.session_files = config
        .session_files
        .iter()
        .map(|x| expand_path(&x))
        .collect();

    debug!("Config: {:?}", config);
    Ok(config)
}

pub fn default_session_format() -> String {
    String::from("#{session_name}")
}
