use async_recursion::async_recursion;
use serde_json::Value;

use crate::{core::app::AppResult, utils::value::GetOrDefault};

use super::client::SpotifyClient;

#[derive(Debug, Clone)]
pub struct Song {
    pub id: String,
    pub name: String,
    pub artists: Vec<String>,
    pub song_length: u64,
    pub disk_number: u64,
    pub track_number: u64,
}

impl Default for Song {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            artists: vec![],
            song_length: 0,
            disk_number: 0,
            track_number: 0,
        }
    }
}

impl Song {
    pub fn new(id: String) -> Self {
        Self {
            id,
            name: String::new(),
            artists: vec![],
            song_length: 0,
            disk_number: 0,
            track_number: 0,
        }
    }

    #[async_recursion]
    pub async fn refresh(&mut self, spotify_client: &mut SpotifyClient) -> AppResult<()> {
        let auth_header = spotify_client.get_auth_header()?;
        let url = format!("https://api.spotify.com/v1/tracks/{}", self.id);

        let response = spotify_client
            .http_client
            .get(url)
            .header("Authorization", auth_header)
            .send()
            .await?;

        let status = response.status();

        if status == 401 {
            spotify_client.refresh_auth_token().await?;

            return self.refresh(spotify_client).await;
        }

        let json = response.json::<Value>().await?;

        let name = json.get_string_or_default("name");
        let mut artists = Vec::<String>::new();
        let song_length = json.get_number_or_default("duration_ms");
        let disk_number = json.get_number_or_default("disc_number");
        let track_number = json.get_number_or_default("track_number");

        let artists_array = json.get_array_or_default("artists");

        for artist_value in artists_array {
            let artist = artist_value.get_string_or_default("name");

            artists.push(artist);
        }

        self.name = name;
        self.artists = artists;
        self.disk_number = disk_number;
        self.song_length = song_length;
        self.track_number = track_number;

        Ok(())
    }

    pub fn get_artists_string(&self) -> String {
        let mut artists_string = String::new();

        for (index, value) in self.artists.iter().enumerate() {
            if index == self.artists.len() - 1 {
                artists_string = artists_string + &format!("{}", value);
            } else {
                artists_string = artists_string + &format!("{}, ", value);
            }
        }

        artists_string
    }

    pub fn get_song_lenth_string(&self) -> String {
        self.milliseconds_to_string(self.song_length)
    }

    pub fn is_empty(&self) -> bool {
        self.id == "".to_string()
            || self.artists.is_empty()
            || self.song_length == 0
            || self.track_number == 0
    }

    fn milliseconds_to_string(&self, ms: u64) -> String {
        let total_seconds = ms / 1_000;
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;

        format!("{}:{:02}", minutes, seconds)
    }
}
