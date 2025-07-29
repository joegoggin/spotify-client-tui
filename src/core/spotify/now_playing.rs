use color_eyre::eyre::eyre;
use serde_json::Value;

use crate::{core::app::AppResult, utils::value::GetOrDefault};

use super::client::SpotifyClient;

#[derive(Debug, Clone)]
pub struct NowPlaying {
    pub song_id: String,
    pub album_id: String,
    pub artist_ids: Vec<String>,
    pub progress: u64,
    pub shuffle: bool,
}

impl Default for NowPlaying {
    fn default() -> Self {
        Self {
            song_id: String::new(),
            album_id: String::new(),
            artist_ids: Vec::<String>::new(),
            progress: 0,
            shuffle: false,
        }
    }
}

impl NowPlaying {
    pub fn get_progress_string(&self) -> String {
        Self::milliseconds_to_string(self.progress)
    }

    pub fn get_shuffle_string(&self) -> String {
        match self.shuffle {
            true => "Shuffle: On".to_string(),
            false => "Shuffle: Off".to_string(),
        }
    }

    pub async fn refresh(&mut self, spotify_client: &mut SpotifyClient) -> AppResult<()> {
        let response = spotify_client.get("me/player").await?;
        let status = response.status();

        if status == 204 {
            return Err(eyre!("No device available"));
        }

        let json = response.json::<Value>().await?;

        let mut song_id = String::new();
        let mut album_id = String::new();
        let mut artist_ids = Vec::<String>::new();
        let progress = json.get_number_or_default("progress_ms");
        let shuffle = json.get_bool_or_default("shuffle_state");

        if let Some(item) = json.get("item") {
            song_id = item.get_string_or_default("id");

            if let Some(album_value) = item.get("album") {
                album_id = album_value.get_string_or_default("id");
            }

            let artists = item.get_array_or_default("artists");

            for artist in artists {
                let id = artist.get_string_or_default("id");

                artist_ids.push(id);
            }
        }

        self.song_id = song_id;
        self.album_id = album_id;
        self.artist_ids = artist_ids;
        self.progress = progress;
        self.shuffle = shuffle;

        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.song_id == "".to_string()
            || self.album_id == "".to_string()
            || self.artist_ids.is_empty()
            || self.progress == 0
    }

    fn milliseconds_to_string(ms: u64) -> String {
        let total_seconds = ms / 1_000;
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;

        format!("{}:{:02}", minutes, seconds)
    }
}
