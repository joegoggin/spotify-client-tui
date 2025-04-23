use async_recursion::async_recursion;
use color_eyre::eyre::eyre;
use log::error;
use serde_json::{json, Value};

use crate::core::app::AppResult;

use super::client::SpotifyClient;

pub struct SpotifyPlayer;

impl SpotifyPlayer {
    pub fn new() -> Self {
        Self {}
    }

    #[async_recursion]
    pub async fn play_song_on_album(
        &self,
        spotify_client: &mut SpotifyClient,
        track_number: u64,
        album_id: String,
    ) -> AppResult<()> {
        let album_uri = format!("spotify:album:{}", album_id);
        let position = track_number - 1;

        let body = json!({
            "context_uri": album_uri,
            "offset": {
                "position": position,
            }
        });

        spotify_client.put("me/player/play", Some(&body)).await?;

        Ok(())
    }

    #[async_recursion]
    pub async fn toggle_pause_play(&self, spotify_client: &mut SpotifyClient) -> AppResult<()> {
        if self.is_playing(spotify_client).await? {
            spotify_client.put("me/player/pause", None).await?;
        } else {
            spotify_client.put("me/player/play", None).await?;
        }

        Ok(())
    }

    #[async_recursion]
    pub async fn is_playing(&self, spotify_client: &mut SpotifyClient) -> AppResult<bool> {
        let response = spotify_client.get("me/player").await?;
        let status = response.status();

        if status == 204 {
            let error_message = "No Spotify device active.";

            error!("{}", error_message);
            return Err(eyre!(error_message));
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
        spotify_client.post("me/player/next", None).await?;

        Ok(())
    }

    #[async_recursion]
    pub async fn previous_song(&self, spotify_client: &mut SpotifyClient) -> AppResult<()> {
        spotify_client.post("me/player/previous", None).await?;

        Ok(())
    }

    #[async_recursion]
    pub async fn toggle_shuffle(&self, spotify_client: &mut SpotifyClient) -> AppResult<()> {
        let response = spotify_client.get("me/player").await?;
        let status = response.status();

        if status == 200 {
            let response_json = response.json::<Value>().await?;

            if let Some(current_shuffle_state) = response_json.get("shuffle_state") {
                if let Value::Bool(current_shuffle_state) = current_shuffle_state {
                    let shuffle_state = !current_shuffle_state;
                    let url = format!("me/player/shuffle?state={}", shuffle_state.to_string());

                    spotify_client.put(&url, None).await?;
                }
            }
        }

        Ok(())
    }
}
