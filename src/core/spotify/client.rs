use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
};

use async_recursion::async_recursion;
use base64::{engine::general_purpose, Engine};
use color_eyre::eyre::eyre;
use log::error;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{core::config::Config, utils::directory::get_home_dir, AppResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    access_token: String,
    refresh_token: String,
}

#[derive(Debug, Clone)]
pub struct NowPlaying {
    pub song: String,
    pub artists: Vec<String>,
    pub album: String,
    pub song_length: u64,
    pub progress: u64,
    pub shuffle: bool,
}

impl NowPlaying {
    pub fn get_song_length_string(&self) -> String {
        Self::milliseconds_to_string(self.song_length)
    }

    pub fn get_progress_string(&self) -> String {
        Self::milliseconds_to_string(self.progress)
    }

    pub fn get_shuffle_string(&self) -> String {
        match self.shuffle {
            true => "Shuffle: On".to_string(),
            false => "Shuffle: Off".to_string(),
        }
    }

    fn milliseconds_to_string(ms: u64) -> String {
        let total_seconds = ms / 1_000;
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;

        format!("{}:{:02}", minutes, seconds)
    }
}

#[derive(Debug, Clone)]
pub struct SpotifyClient {
    pub config: Config,
    pub credentials: Option<Credentials>,
    pub code: Option<String>,
    pub auth_url: String,
    pub now_playing: Option<NowPlaying>,
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
            now_playing: None,
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
                }
            }
        }

        Ok(())
    }

    #[async_recursion]
    pub async fn refresh_now_playing(&mut self) -> AppResult<Self> {
        let auth_header = Self::get_auth_header(self)?;

        let response = self
            .http_client
            .get("https://api.spotify.com/v1/me/player")
            .header("Authorization", auth_header)
            .send()
            .await?;

        let status = response.status();

        if status == 204 {
            self.now_playing = None;
        }

        if status == 401 {
            self.refresh_auth_token().await?;

            return self.refresh_now_playing().await;
        }

        let json = response.json::<Value>().await?;

        let mut song_string = String::new();
        let mut album_string = String::new();
        let mut artists_vec = Vec::<String>::new();
        let mut song_length_num: u64 = 0;
        let mut progress_num: u64 = 0;
        let mut shuffle_bool = false;

        if let Some(item) = json.get("item") {
            if let Some(song) = item.get("name") {
                match song {
                    Value::String(song) => song_string = song.to_owned(),
                    _ => {}
                }
            }

            if let Some(album) = item.get("album") {
                if let Some(album_name) = album.get("name") {
                    match album_name {
                        Value::String(album_name) => album_string = album_name.to_owned(),
                        _ => {}
                    }
                }
            }

            if let Some(artists) = item.get("artists") {
                match artists {
                    Value::Array(artists) => {
                        for artist in artists {
                            if let Some(artist_name) = artist.get("name") {
                                match artist_name {
                                    Value::String(artist_name) => {
                                        artists_vec.push(artist_name.to_owned())
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }

            if let Some(song_length) = item.get("duration_ms") {
                match song_length {
                    Value::Number(song_length) => {
                        if let Some(song_length) = song_length.to_owned().as_u64() {
                            song_length_num = song_length;
                        }
                    }
                    _ => {}
                }
            }
        }

        if let Some(progress) = json.get("progress_ms") {
            match progress {
                Value::Number(progress) => {
                    if let Some(progress) = progress.to_owned().as_u64() {
                        progress_num = progress;
                    }
                }
                _ => {}
            }
        }

        if let Some(shuffle) = json.get("shuffle_state") {
            match shuffle {
                Value::Bool(shuffle) => {
                    shuffle_bool = shuffle.to_owned();
                }
                _ => {}
            }
        }

        self.now_playing = Some(NowPlaying {
            song: song_string,
            artists: artists_vec,
            album: album_string,
            song_length: song_length_num,
            progress: progress_num,
            shuffle: shuffle_bool,
        });

        Ok(self.to_owned())
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
