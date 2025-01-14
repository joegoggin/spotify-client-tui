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
    pub redirect_uri: Option<String>,
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
            redirect_uri: None,
        })
    }

    pub fn update(&mut self, client_id: String, redirect_uri: String) -> AppResult<()> {
        self.client_id = Some(client_id);
        self.redirect_uri = Some(redirect_uri);

        let data = serde_json::to_string_pretty(self)?;
        let mut file = File::create(Self::get_file_path())?;
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
