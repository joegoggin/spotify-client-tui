use async_recursion::async_recursion;
use color_eyre::eyre::eyre;
use serde_json::Value;

use crate::{utils::value::GetOrDefault, AppResult};

use super::client::SpotifyClient;

#[derive(Debug, Clone)]
pub struct NowPlaying {
    pub song: String,
    pub artists: Vec<String>,
    pub album: String,
    pub song_length: u64,
    pub progress: u64,
    pub shuffle: bool,
}

impl Default for NowPlaying {
    fn default() -> Self {
        Self {
            song: String::new(),
            artists: vec![].into(),
            album: String::new(),
            song_length: 0,
            progress: 0,
            shuffle: false,
        }
    }
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

        let mut song = String::new();
        let mut album = String::new();
        let mut artists = Vec::<String>::new();
        let mut song_length: u64 = 0;
        let progress = json.get_number_or_default("progress_ms");
        let shuffle = json.get_bool_or_default("shuffle_state");

        if let Some(item) = json.get("item") {
            song = item.get_string_or_default("name");

            if let Some(album_value) = item.get("album") {
                album = album_value.get_string_or_default("name");
            }

            let artists_array = item.get_array_or_default("artists");

            for artist in artists_array {
                let artist_name = artist.get_string_or_default("name");

                artists.push(artist_name.to_string());
            }

            song_length = item.get_number_or_default("duration_ms");
        }

        self.song = song;
        self.artists = artists;
        self.album = album;
        self.song_length = song_length;
        self.progress = progress;
        self.shuffle = shuffle;

        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.song == "".to_string()
            || self.artists.is_empty()
            || self.album == "".to_string()
            || self.song_length == 0
            || self.progress == 0
    }

    fn milliseconds_to_string(ms: u64) -> String {
        let total_seconds = ms / 1_000;
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;

        format!("{}:{:02}", minutes, seconds)
    }
}
