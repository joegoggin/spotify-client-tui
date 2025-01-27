use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use dirs::home_dir;
use log::error;
use serde::{Deserialize, Serialize};

use crate::AppResult;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub redirect_uri: Option<String>,
    pub scope: Option<String>,
}

impl Config {
    pub fn new() -> AppResult<Self> {
        let file_path = Self::get_file_path();

        if Path::new(&file_path).exists() {
            let data = fs::read_to_string(&file_path)?;
            let config: Config = serde_json::from_str(&data)?;

            return Ok(config);
        }

        Ok(Self {
            client_id: None,
            client_secret: None,
            redirect_uri: None,
            scope: None,
        })
    }

    pub fn update(&mut self, new_config: Config) -> AppResult<()> {
        self.client_id = new_config.client_id;
        self.client_secret = new_config.client_secret;
        self.redirect_uri = new_config.redirect_uri;
        self.scope = new_config.scope;

        let data = serde_json::to_string_pretty(self)?;
        let file_path = Self::get_file_path();

        if let Some(parent) = Path::new(&file_path).parent() {
            fs::create_dir_all(parent)?;
        };

        let mut file = File::create(file_path.clone())?;
        file.write_all(data.as_bytes())?;

        Ok(())
    }

    fn get_file_path() -> String {
        match home_dir() {
            Some(home_dir) => format!(
                "{}/.config/spotify-client-tui/config.json",
                home_dir.display()
            ),
            None => {
                let error_message = "Unable to find home directory.";

                error!("{}", error_message);
                panic!("{}", error_message);
            }
        }
    }
}
