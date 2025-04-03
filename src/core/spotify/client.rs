use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
};

use base64::{engine::general_purpose, Engine};
use color_eyre::eyre::eyre;
use log::error;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    core::{app::AppResult, config::Config},
    utils::directory::get_home_dir,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    access_token: String,
    refresh_token: String,
}

#[derive(Debug, Clone)]
pub struct SpotifyClient {
    pub config: Config,
    pub credentials: Option<Credentials>,
    pub code: Option<String>,
    pub auth_url: String,
    pub http_client: Client,
}

impl SpotifyClient {
    pub fn new(config: Config) -> AppResult<Self> {
        let mut url = Url::parse("https://accounts.spotify.com/authorize")?;

        url.query_pairs_mut().append_pair("response_type", "code");

        let mut error_message = "Failed to create Spotify Client: \n".to_string();

        match config.client_id.clone() {
            Some(client_id) => {
                url.query_pairs_mut().append_pair("client_id", &client_id);
            }
            None => {
                error_message = error_message + "No Client ID provided.";

                error!("{}", error_message);
                return Err(eyre!(error_message));
            }
        }

        match config.redirect_uri.clone() {
            Some(redirect_uri) => {
                url.query_pairs_mut()
                    .append_pair("redirect_uri", &redirect_uri);
            }
            None => {
                error_message = error_message + "No Redirect URI provided.";

                error!("{}", error_message);
                return Err(eyre!(error_message));
            }
        }

        match config.scope.clone() {
            Some(scope) => {
                url.query_pairs_mut().append_pair("scope", &scope);
            }
            None => {
                error_message = error_message + "No Scope Provied.";

                error!("{}", error_message);
                return Err(eyre!(error_message));
            }
        }

        let mut credentials: Option<Credentials> = None;
        let file_path = Self::get_file_path()?;

        if Path::new(&file_path).exists() {
            let data = fs::read_to_string(&file_path)?;
            let credentials_data: Credentials = serde_json::from_str(&data)?;

            credentials = Some(credentials_data);
        }

        Ok(Self {
            config,
            credentials,
            code: None,
            auth_url: url.to_string(),
            http_client: Client::new(),
        })
    }

    pub async fn set_code_and_access_token(&mut self, code: String) -> AppResult<()> {
        self.code = Some(code.clone());

        if let Some(client_id) = self.config.client_id.clone() {
            if let Some(client_secret) = self.config.client_secret.clone() {
                if let Some(redirect_uri) = self.config.redirect_uri.clone() {
                    let auth_header = format!(
                        "Basic {}",
                        general_purpose::STANDARD.encode(format!(
                            "{}:{}",
                            client_id.clone(),
                            client_secret.clone()
                        ))
                    );

                    let mut body = HashMap::<&str, &str>::new();

                    body.insert("code", &code);
                    body.insert("grant_type", "authorization_code");
                    body.insert("redirect_uri", &redirect_uri);

                    let response = self
                        .http_client
                        .post("https://accounts.spotify.com/api/token")
                        .header("Content-Type", "application/x-www-form-urlencoded")
                        .header("Authorization", auth_header)
                        .form(&body)
                        .send()
                        .await?
                        .json::<Value>()
                        .await?;

                    let mut access_token: Option<String> = None;
                    let mut refresh_token: Option<String> = None;

                    if let Some(access_token_value) = response.get("access_token") {
                        match access_token_value.to_owned() {
                            Value::String(access_token_value) => {
                                access_token = Some(access_token_value);
                            }
                            _ => {}
                        }
                    }

                    if let Some(refresh_token_value) = response.get("refresh_token") {
                        match refresh_token_value.to_owned() {
                            Value::String(refresh_token_value) => {
                                refresh_token = Some(refresh_token_value);
                            }
                            _ => {}
                        }
                    }

                    if let Some(access_token) = access_token {
                        if let Some(refresh_token) = refresh_token {
                            let credentials = Credentials {
                                refresh_token,
                                access_token,
                            };

                            let data = serde_json::to_string_pretty(&credentials)?;
                            let file_path = Self::get_file_path()?;

                            if let Some(parent) = Path::new(&file_path).parent() {
                                fs::create_dir_all(parent)?;
                            }

                            let mut file = File::create(file_path)?;
                            file.write_all(data.as_bytes())?;

                            self.credentials = Some(credentials);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn refresh_auth_token(&mut self) -> AppResult<()> {
        if let Some(credentials) = self.credentials.clone() {
            if let Some(client_id) = self.config.client_id.clone() {
                if let Some(client_secret) = self.config.client_secret.clone() {
                    let auth_header = format!(
                        "Basic {}",
                        general_purpose::STANDARD.encode(format!(
                            "{}:{}",
                            client_id.clone(),
                            client_secret.clone()
                        ))
                    );

                    let mut body = HashMap::<&str, &str>::new();

                    body.insert("grant_type", "refresh_token");
                    body.insert("refresh_token", &credentials.refresh_token);
                    body.insert("client_id", &client_id);

                    let response = self
                        .http_client
                        .post("https://accounts.spotify.com/api/token")
                        .header("Content-Type", "application/x-www-form-urlencoded")
                        .header("Authorization", auth_header)
                        .form(&body)
                        .send()
                        .await?
                        .json::<Value>()
                        .await?;

                    let mut access_token = credentials.access_token;

                    if let Some(access_token_value) = response.get("access_token") {
                        match access_token_value.to_owned() {
                            Value::String(access_token_value) => access_token = access_token_value,
                            _ => {}
                        }
                    }

                    let new_credentials = Credentials {
                        refresh_token: credentials.refresh_token,
                        access_token,
                    };

                    let data = serde_json::to_string_pretty(&new_credentials)?;
                    let file_path = Self::get_file_path()?;

                    if let Some(parent) = Path::new(&file_path).parent() {
                        fs::create_dir_all(parent)?;
                    }

                    let mut file = File::create(file_path)?;
                    file.write_all(data.as_bytes())?;

                    self.credentials = Some(new_credentials);
                }
            }
        }

        Ok(())
    }

    pub fn get_auth_header(&self) -> AppResult<String> {
        match self.credentials.clone() {
            Some(credentials) => Ok(format!("Bearer {}", credentials.access_token)),
            None => {
                let error_message = "No credentials set";

                error!("{}", error_message);
                Err(eyre!(error_message))
            }
        }
    }

    fn get_file_path() -> AppResult<String> {
        let home_dir = get_home_dir()?;

        Ok(format!(
            "{}/.config/spotify-client-tui/credentials.json",
            home_dir
        ))
    }
}
