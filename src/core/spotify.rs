use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
};

use async_recursion::async_recursion;
use base64::{engine::general_purpose, Engine};
use color_eyre::eyre::eyre;
use log::{debug, error};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{utils::directory::get_home_dir, AppResult};

use super::config::Config;

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
}

#[derive(Debug, Clone)]
pub struct SpotifyClient {
    pub config: Config,
    pub credentials: Option<Credentials>,
    pub code: Option<String>,
    pub auth_url: String,
    pub now_playing: Option<NowPlaying>,
    http_client: Client,
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

    pub async fn refresh(&mut self) -> AppResult<()> {
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
    pub async fn is_playing(&mut self) -> AppResult<bool> {
        let auth_header = self.get_auth_header()?;

        let response = self
            .http_client
            .get("https://api.spotify.com/v1/me/player")
            .header("Authorization", auth_header.clone())
            .send()
            .await?;

        let status = response.status();

        if status == 204 {
            let error_message = "No Spotify device active.";

            error!("{}", error_message);
            return Err(eyre!(error_message));
        }

        if status == 401 {
            self.refresh().await?;

            return self.is_playing().await;
        }

        if status == 200 {
            let response_json = response.json::<Value>().await?;

            if let Some(is_playing) = response_json.get("is_playing") {
                if let Value::Bool(is_playing) = is_playing {
                    return Ok(is_playing.to_owned());
                }
            }
        }

        Ok(false)
    }

    #[async_recursion]
    pub async fn toggle_pause_play(&mut self) -> AppResult<()> {
        let auth_header = self.get_auth_header()?;

        if self.is_playing().await? {
            let response = self
                .http_client
                .put("https://api.spotify.com/v1/me/player/pause")
                .header("Authorization", auth_header)
                .header("Content-Length", 0)
                .send()
                .await?;

            let status = response.status();

            if status == 401 {
                self.refresh().await?;

                return self.toggle_pause_play().await;
            }
        } else {
            let response = self
                .http_client
                .put("https://api.spotify.com/v1/me/player/play")
                .header("Authorization", auth_header)
                .header("Content-Length", 0)
                .send()
                .await?;

            let status = response.status();

            if status == 401 {
                self.refresh().await?;

                return self.toggle_pause_play().await;
            }
        }

        Ok(())
    }

    #[async_recursion]
    pub async fn next_song(&mut self) -> AppResult<()> {
        let auth_header = self.get_auth_header()?;

        let response = self
            .http_client
            .post("https://api.spotify.com/v1/me/player/next")
            .header("Authorization", auth_header)
            .header("Content-Length", 0)
            .send()
            .await?;

        let status = response.status();

        if status == 401 {
            self.refresh().await?;

            return self.next_song().await;
        }

        Ok(())
    }

    #[async_recursion]
    pub async fn previous_song(&mut self) -> AppResult<()> {
        let auth_header = self.get_auth_header()?;

        let response = self
            .http_client
            .post("https://api.spotify.com/v1/me/player/previous")
            .header("Authorization", auth_header)
            .header("Content-Length", 0)
            .send()
            .await?;

        let status = response.status();

        if status == 401 {
            self.refresh().await?;

            return self.previous_song().await;
        }

        Ok(())
    }

    #[async_recursion]
    pub async fn toggle_shuffle(&mut self) -> AppResult<()> {
        let auth_header = self.get_auth_header()?;

        let response = self
            .http_client
            .get("https://api.spotify.com/v1/me/player")
            .header("Authorization", auth_header.clone())
            .send()
            .await?;

        let status = response.status();

        if status == 401 {
            self.refresh().await?;

            return self.toggle_shuffle().await;
        }

        if status == 200 {
            let response_json = response.json::<Value>().await?;

            if let Some(current_shuffle_state) = response_json.get("shuffle_state") {
                if let Value::Bool(current_shuffle_state) = current_shuffle_state {
                    let shuffle_state = !current_shuffle_state;

                    let url = format!(
                        "https://api.spotify.com/v1/me/player/shuffle?state={}",
                        shuffle_state.to_string()
                    );

                    let response = self
                        .http_client
                        .put(url)
                        .header("Content-Type", "application/x-www-form-urlencoded")
                        .header("Authorization", auth_header)
                        .header("Content-Length", 0)
                        .send()
                        .await?;

                    let status = response.status();

                    if status == 401 {
                        self.refresh().await?;

                        return self.toggle_shuffle().await;
                    }
                }
            }
        }

        Ok(())
    }

    #[async_recursion]
    pub async fn list_devices(&mut self) -> AppResult<()> {
        let auth_header = self.get_auth_header()?;

        let response = self
            .http_client
            .get("https://api.spotify.com/v1/me/player/devices")
            .header("Authorization", auth_header)
            .send()
            .await?;

        let status = response.status();

        if status == 401 {
            self.refresh().await?;

            return self.list_devices().await;
        }

        if status == 200 {
            let response_json = response.json::<Value>().await?;

            if let Some(devices) = response_json.get("devices") {
                if let Value::Array(devices) = devices {
                    for device in devices {
                        if let Some(id) = device.get("id") {
                            if let Value::String(id) = id {
                                println!("id: {}", id);
                            }
                        }

                        if let Some(name) = device.get("name") {
                            if let Value::String(name) = name {
                                println!("name: {}", name);
                            }
                        }

                        println!();
                    }
                }
            }
        }

        Ok(())
    }

    #[async_recursion]
    pub async fn set_device(&mut self, device_id: String) -> AppResult<()> {
        let auth_header = Self::get_auth_header(self)?;

        let body = json!({
            "device_ids": [&device_id],
            "play": true,
        });

        let response = self
            .http_client
            .put("https://api.spotify.com/v1/me/player")
            .header("Authorization", auth_header)
            .json(&body)
            .send()
            .await?;

        let status = response.status();

        if status == 401 {
            self.refresh().await?;

            return self.set_device(device_id).await;
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
            self.refresh().await?;

            return self.refresh_now_playing().await;
        }

        let json = response.json::<Value>().await?;

        let mut song_string = String::new();
        let mut album_string = String::new();
        let mut artists_vec = Vec::<String>::new();

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
        }

        self.now_playing = Some(NowPlaying {
            song: song_string,
            artists: artists_vec,
            album: album_string,
        });

        Ok(self.to_owned())
    }

    fn get_auth_header(&self) -> AppResult<String> {
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
