use async_recursion::async_recursion;
use serde_json::Value;

use crate::{core::app::AppResult, utils::value::GetOrDefault};

use super::client::SpotifyClient;

#[derive(Debug, Clone)]
pub struct Song {
    pub id: String,
    pub name: String,
    pub artist_names: Vec<String>,
    pub album_name: String,
    pub album_year: String,
    pub song_length: u64,
    pub disk_number: u64,
    pub track_number: u64,
}

impl Default for Song {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            artist_names: vec![],
            album_name: String::new(),
            album_year: String::new(),
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
            artist_names: vec![],
            album_name: String::new(),
            album_year: String::new(),
            song_length: 0,
            disk_number: 0,
            track_number: 0,
        }
    }

    #[async_recursion]
    pub async fn refresh(&mut self, spotify_client: &mut SpotifyClient) -> AppResult<()> {
        let url = format!("tracks/{}", self.id);
        let response = spotify_client.get(&url).await?;
        let json = response.json::<Value>().await?;

        let name = json.get_string_or_default("name");
        let mut artist_names = Vec::<String>::new();
        let mut album_name = String::new();
        let mut album_year = String::new();
        let song_length = json.get_number_or_default("duration_ms");
        let disk_number = json.get_number_or_default("disc_number");
        let track_number = json.get_number_or_default("track_number");

        let artists_array = json.get_array_or_default("artists");

        for artist_value in artists_array {
            let artist = artist_value.get_string_or_default("name");

            artist_names.push(artist);
        }

        if let Some(album) = json.get("album") {
            album_name = album.get_string_or_default("name");
            album_year = album.get_string_or_default("release_date");

            if album_year.len() > 5 {
                album_year = album_year[0..4].to_string();
            }
        }

        self.name = name;
        self.artist_names = artist_names;
        self.album_name = album_name;
        self.album_year = album_year;
        self.disk_number = disk_number;
        self.song_length = song_length;
        self.track_number = track_number;

        Ok(())
    }

    pub fn get_artists_string(&self) -> String {
        let mut artists_string = String::new();

        for (index, value) in self.artist_names.iter().enumerate() {
            if index == self.artist_names.len() - 1 {
                artists_string = artists_string + &format!("{}", value);
            } else {
                artists_string = artists_string + &format!("{}, ", value);
            }
        }

        artists_string
    }

    pub fn get_song_length_string(&self) -> String {
        self.milliseconds_to_string(self.song_length)
    }

    pub fn is_empty(&self) -> bool {
        self.id == "".to_string()
            || self.artist_names.is_empty()
            || self.album_name == "".to_string()
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
