use async_recursion::async_recursion;
use color_eyre::eyre::eyre;
use serde_json::Value;

use crate::{core::app::AppResult, utils::value::GetOrDefault};

use super::{album::Album, client::SpotifyClient, song::Song};

#[derive(Debug, Clone)]
pub struct NowPlaying {
    pub song: Song,
    pub album: Album,
    pub progress: u64,
    pub shuffle: bool,
}

impl Default for NowPlaying {
    fn default() -> Self {
        Self {
            song: Song::default(),
            album: Album::default(),
            progress: 0,
            shuffle: false,
        }
    }
}

impl NowPlaying {
    pub fn get_song_length_string(&self) -> String {
        Self::milliseconds_to_string(self.song.song_length)
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

    #[async_recursion]
    pub async fn refresh(&mut self, spotify_client: &mut SpotifyClient) -> AppResult<()> {
        let auth_header = spotify_client.get_auth_header()?;

        let response = spotify_client
            .http_client
            .get("https://api.spotify.com/v1/me/player")
            .header("Authorization", auth_header)
            .send()
            .await?;

        let status = response.status();

        if status == 204 {
            return Err(eyre!("No device available"));
        }

        if status == 401 {
            spotify_client.refresh_auth_token().await?;

            return self.refresh(spotify_client).await;
        }

        let json = response.json::<Value>().await?;

        let progress = json.get_number_or_default("progress_ms");
        let shuffle = json.get_bool_or_default("shuffle_state");

        if let Some(item) = json.get("item") {
            let song_id = item.get_string_or_default("id");

            if song_id != self.song.id {
                self.song.id = song_id;
                self.song.refresh(spotify_client).await?;
            }

            if let Some(album_value) = item.get("album") {
                let album_id = album_value.get_string_or_default("id");

                if album_id != self.album.id {
                    self.album.id = album_id;
                    self.album.refresh(spotify_client).await?;
                }
            }
        }

        self.progress = progress;
        self.shuffle = shuffle;

        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.song.is_empty()
            || self.album.is_empty()
            || self.song.song_length == 0
            || self.progress == 0
    }

    fn milliseconds_to_string(ms: u64) -> String {
        let total_seconds = ms / 1_000;
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;

        format!("{}:{:02}", minutes, seconds)
    }
}
