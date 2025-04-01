use async_recursion::async_recursion;
use log::debug;
use serde_json::Value;

use crate::{utils::value::GetOrDefault, AppResult};

use super::{client::SpotifyClient, song::Song};

#[derive(Debug, Clone)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub artists: Vec<String>,
    pub year: String,
    pub songs: Vec<Song>,
    pub total_songs: u64,
}

impl Default for Album {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            artists: vec![],
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
            artists: vec![],
            year: String::new(),
            songs: vec![],
            total_songs: 0,
        }
    }

    #[async_recursion]
    pub async fn refresh(&mut self, spotify_client: &mut SpotifyClient) -> AppResult<()> {
        let auth_header = spotify_client.get_auth_header()?;
        let url = format!("https://api.spotify.com/v1/albums/{}", self.id);

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
        let year = json.get_string_or_default("release_date")[0..4].to_string();
        let mut songs = Vec::<Song>::new();
        let total_songs = json.get_number_or_default("total_tracks");

        let artists_array = json.get_array_or_default("artists");

        for artist_value in artists_array {
            let artist = artist_value.get_string_or_default("name");

            artists.push(artist);
        }

        if let Some(tracks) = json.get("tracks") {
            let songs_array = tracks.get_array_or_default("items");

            for song_value in songs_array {
                let song_id = song_value.get_string_or_default("id");
                let mut song = Song::new(song_id);

                song.refresh(spotify_client).await?;

                songs.push(song)
            }
        }

        self.name = name;
        self.artists = artists;
        self.year = year;
        self.songs = songs;
        self.total_songs = total_songs;

        debug!("{:#?}", self);
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.id == "".to_string()
            || self.artists.is_empty()
            || self.year == "".to_string()
            || self.songs.is_empty()
            || self.total_songs == 0
    }
}
