use async_recursion::async_recursion;
use color_eyre::eyre::eyre;
use log::{debug, error};
use serde_json::{json, Value};

use crate::AppResult;

use super::client::SpotifyClient;

pub struct SpotifyPlayer;

impl SpotifyPlayer {
    pub fn new() -> Self {
        Self {}
    }

    #[async_recursion]
    pub async fn toggle_pause_play(&self, spotify_client: &mut SpotifyClient) -> AppResult<()> {
        let auth_header = spotify_client.get_auth_header()?;

        if self.is_playing(spotify_client).await? {
            let response = spotify_client
                .http_client
                .put("https://api.spotify.com/v1/me/player/pause")
                .header("Authorization", auth_header)
                .header("Content-Length", 0)
                .send()
                .await?;

            let status = response.status();

            if status == 401 {
                spotify_client.refresh_auth_token().await?;

                return self.toggle_pause_play(spotify_client).await;
            }
        } else {
            let response = spotify_client
                .http_client
                .put("https://api.spotify.com/v1/me/player/play")
                .header("Authorization", auth_header)
                .header("Content-Length", 0)
                .send()
                .await?;

            let status = response.status();

            if status == 401 {
                spotify_client.refresh_auth_token().await?;

                return self.toggle_pause_play(spotify_client).await;
            }
        }

        Ok(())
    }

    #[async_recursion]
    pub async fn is_playing(&self, spotify_client: &mut SpotifyClient) -> AppResult<bool> {
        let auth_header = spotify_client.get_auth_header()?;

        let response = spotify_client
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
            spotify_client.refresh_auth_token().await?;

            return self.is_playing(spotify_client).await;
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
    pub async fn next_song(&self, spotify_client: &mut SpotifyClient) -> AppResult<()> {
        let auth_header = spotify_client.get_auth_header()?;

        let response = spotify_client
            .http_client
            .post("https://api.spotify.com/v1/me/player/next")
            .header("Authorization", auth_header)
            .header("Content-Length", 0)
            .send()
            .await?;

        let status = response.status();

        if status == 401 {
            spotify_client.refresh_auth_token().await?;

            return self.next_song(spotify_client).await;
        }

        Ok(())
    }

    #[async_recursion]
    pub async fn previous_song(&self, spotify_client: &mut SpotifyClient) -> AppResult<()> {
        let auth_header = spotify_client.get_auth_header()?;

        let response = spotify_client
            .http_client
            .post("https://api.spotify.com/v1/me/player/previous")
            .header("Authorization", auth_header)
            .header("Content-Length", 0)
            .send()
            .await?;

        let status = response.status();

        if status == 401 {
            spotify_client.refresh_auth_token().await?;

            return self.previous_song(spotify_client).await;
        }

        Ok(())
    }

    #[async_recursion]
    pub async fn toggle_shuffle(&self, spotify_client: &mut SpotifyClient) -> AppResult<()> {
        let auth_header = spotify_client.get_auth_header()?;

        let response = spotify_client
            .http_client
            .get("https://api.spotify.com/v1/me/player")
            .header("Authorization", auth_header.clone())
            .send()
            .await?;

        let status = response.status();

        if status == 401 {
            spotify_client.refresh_auth_token().await?;

            return self.toggle_shuffle(spotify_client).await;
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

                    let response = spotify_client
                        .http_client
                        .put(url)
                        .header("Content-Type", "application/x-www-form-urlencoded")
                        .header("Authorization", auth_header)
                        .header("Content-Length", 0)
                        .send()
                        .await?;

                    let status = response.status();

                    if status == 401 {
                        spotify_client.refresh_auth_token().await?;

                        return self.toggle_shuffle(spotify_client).await;
                    }
                }
            }
        }

        Ok(())
    }

    #[async_recursion]
    pub async fn list_devices(&self, spotify_client: &mut SpotifyClient) -> AppResult<()> {
        let auth_header = spotify_client.get_auth_header()?;

        let response = spotify_client
            .http_client
            .get("https://api.spotify.com/v1/me/player/devices")
            .header("Authorization", auth_header)
            .send()
            .await?;

        let status = response.status();

        if status == 401 {
            spotify_client.refresh_auth_token().await?;

            return self.list_devices(spotify_client).await;
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
    pub async fn set_device(
        &mut self,
        spotify_client: &mut SpotifyClient,
        device_id: String,
    ) -> AppResult<()> {
        let auth_header = spotify_client.get_auth_header()?;

        let body = json!({
            "device_ids": [&device_id],
            "play": true,
        });

        let response = spotify_client
            .http_client
            .put("https://api.spotify.com/v1/me/player")
            .header("Authorization", auth_header)
            .json(&body)
            .send()
            .await?;

        let status = response.status();

        if status == 401 {
            spotify_client.refresh_auth_token().await?;

            return self.set_device(spotify_client, device_id).await;
        }

        Ok(())
    }
}
