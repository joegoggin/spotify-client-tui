use async_recursion::async_recursion;
use color_eyre::eyre::eyre;
use log::error;
use serde_json::{json, Value};

use crate::AppResult;

use super::client::SpotifyClient;

pub struct SpotifyPlayer {
    pub spotify_client: SpotifyClient,
}

impl SpotifyPlayer {
    pub fn new(spotify_client: SpotifyClient) -> Self {
        Self { spotify_client }
    }

    #[async_recursion]
    pub async fn toggle_pause_play(&mut self) -> AppResult<()> {
        let auth_header = self.spotify_client.get_auth_header()?;

        if self.is_playing().await? {
            let response = self
                .spotify_client
                .http_client
                .put("https://api.spotify.com/v1/me/player/pause")
                .header("Authorization", auth_header)
                .header("Content-Length", 0)
                .send()
                .await?;

            let status = response.status();

            if status == 401 {
                self.spotify_client.refresh_auth_token().await?;

                return self.toggle_pause_play().await;
            }
        } else {
            let response = self
                .spotify_client
                .http_client
                .put("https://api.spotify.com/v1/me/player/play")
                .header("Authorization", auth_header)
                .header("Content-Length", 0)
                .send()
                .await?;

            let status = response.status();

            if status == 401 {
                self.spotify_client.refresh_auth_token().await?;

                return self.toggle_pause_play().await;
            }
        }

        Ok(())
    }

    #[async_recursion]
    pub async fn is_playing(&mut self) -> AppResult<bool> {
        let auth_header = self.spotify_client.get_auth_header()?;

        let response = self
            .spotify_client
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
            self.spotify_client.refresh_auth_token().await?;

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
    pub async fn next_song(&mut self) -> AppResult<()> {
        let auth_header = self.spotify_client.get_auth_header()?;

        let response = self
            .spotify_client
            .http_client
            .post("https://api.spotify.com/v1/me/player/next")
            .header("Authorization", auth_header)
            .header("Content-Length", 0)
            .send()
            .await?;

        let status = response.status();

        if status == 401 {
            self.spotify_client.refresh_auth_token().await?;

            return self.next_song().await;
        }

        Ok(())
    }

    #[async_recursion]
    pub async fn previous_song(&mut self) -> AppResult<()> {
        let auth_header = self.spotify_client.get_auth_header()?;

        let response = self
            .spotify_client
            .http_client
            .post("https://api.spotify.com/v1/me/player/previous")
            .header("Authorization", auth_header)
            .header("Content-Length", 0)
            .send()
            .await?;

        let status = response.status();

        if status == 401 {
            self.spotify_client.refresh_auth_token().await?;

            return self.previous_song().await;
        }

        Ok(())
    }

    #[async_recursion]
    pub async fn toggle_shuffle(&mut self) -> AppResult<()> {
        let auth_header = self.spotify_client.get_auth_header()?;

        let response = self
            .spotify_client
            .http_client
            .get("https://api.spotify.com/v1/me/player")
            .header("Authorization", auth_header.clone())
            .send()
            .await?;

        let status = response.status();

        if status == 401 {
            self.spotify_client.refresh_auth_token().await?;

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
                        .spotify_client
                        .http_client
                        .put(url)
                        .header("Content-Type", "application/x-www-form-urlencoded")
                        .header("Authorization", auth_header)
                        .header("Content-Length", 0)
                        .send()
                        .await?;

                    let status = response.status();

                    if status == 401 {
                        self.spotify_client.refresh_auth_token().await?;

                        return self.toggle_shuffle().await;
                    }
                }
            }
        }

        Ok(())
    }

    #[async_recursion]
    pub async fn list_devices(&mut self) -> AppResult<()> {
        let auth_header = self.spotify_client.get_auth_header()?;

        let response = self
            .spotify_client
            .http_client
            .get("https://api.spotify.com/v1/me/player/devices")
            .header("Authorization", auth_header)
            .send()
            .await?;

        let status = response.status();

        if status == 401 {
            self.spotify_client.refresh_auth_token().await?;

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
        let auth_header = self.spotify_client.get_auth_header()?;

        let body = json!({
            "device_ids": [&device_id],
            "play": true,
        });

        let response = self
            .spotify_client
            .http_client
            .put("https://api.spotify.com/v1/me/player")
            .header("Authorization", auth_header)
            .json(&body)
            .send()
            .await?;

        let status = response.status();

        if status == 401 {
            self.spotify_client.refresh_auth_token().await?;

            return self.set_device(device_id).await;
        }

        Ok(())
    }
}
