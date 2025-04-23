use async_recursion::async_recursion;
use serde_json::Value;

use crate::{core::app::AppResult, utils::value::GetOrDefault};

use super::{client::SpotifyClient, NameAndId};

#[derive(Debug, Clone)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub artist_names: Vec<String>,
    pub year: String,
    pub songs: Vec<NameAndId>,
    pub total_songs: u64,
}

impl Default for Album {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            artist_names: vec![],
            year: String::new(),
            songs: vec![],
            total_songs: 0,
        }
    }
}

impl Album {
    pub fn new(id: String) -> Self {
        Self {
            id,
            name: String::new(),
            artist_names: vec![],
            year: String::new(),
            songs: vec![],
            total_songs: 0,
        }
    }

    #[async_recursion]
    pub async fn refresh(&mut self, spotify_client: &mut SpotifyClient) -> AppResult<()> {
        let url = format!("albums/{}", self.id);
        let response = spotify_client.get(&url).await?;
        let json = response.json::<Value>().await?;

        let name = json.get_string_or_default("name");
        let mut artists = Vec::<String>::new();
        let year = json.get_string_or_default("release_date")[0..4].to_string();
        let mut songs = Vec::<NameAndId>::new();
        let total_songs = json.get_number_or_default("total_tracks");

        let artists_array = json.get_array_or_default("artists");

        for artist_value in artists_array {
            let artist = artist_value.get_string_or_default("name");

            artists.push(artist);
        }

        if let Some(tracks) = json.get("tracks") {
            let songs_array = tracks.get_array_or_default("items");

            for song_value in songs_array {
                let song_name = song_value.get_string_or_default("name");
                let song_id = song_value.get_string_or_default("id");

                songs.push((song_name, song_id))
            }
        }

        self.name = name;
        self.artist_names = artists;
        self.year = year;
        self.songs = songs;
        self.total_songs = total_songs;

        Ok(())
    }

    pub fn get_first_artist(&self) -> String {
        let mut first_artist = String::new();

        if let Some(artist) = self.artist_names.get(0) {
            first_artist = artist.clone();
        }

        first_artist
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

    pub fn is_empty(&self) -> bool {
        self.id == "".to_string()
            || self.artist_names.is_empty()
            || self.year == "".to_string()
            || self.songs.is_empty()
            || self.total_songs == 0
    }
}
