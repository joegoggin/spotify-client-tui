use serde_json::Value;

use crate::{utils::value::GetOrDefault, AppResult};

use super::{client::SpotifyClient, NameAndId};

#[derive(Debug, Clone)]
pub struct SearchResults {
    query: String,
    pub artists: Vec<NameAndId>,
    pub albums: Vec<NameAndId>,
    pub songs: Vec<NameAndId>,
    pub playlists: Vec<NameAndId>,
}

impl Default for SearchResults {
    fn default() -> Self {
        Self {
            query: String::new(),
            artists: vec![],
            albums: vec![],
            songs: vec![],
            playlists: vec![],
        }
    }
}

impl SearchResults {
    pub fn set_query(&mut self, query: String) {
        let formated_query = query
            .split(" ")
            .map(|word| word)
            .collect::<Vec<_>>()
            .join("%20");

        self.query = formated_query;
    }

    pub fn get_top_results(&self) -> Vec<NameAndId> {
        let mut top_results: Vec<NameAndId> = vec![];

        for i in 0..=40 {
            match i % 4 {
                0 => {
                    if i < self.artists.len() {
                        let name_and_id = self.artists[i].clone();
                        let name = format!("{} - Artist", name_and_id.0);

                        top_results.push((name, name_and_id.1));
                    }
                }
                1 => {
                    if i < self.albums.len() {
                        let name_and_id = self.albums[i].clone();
                        let name = format!("{} - Album", name_and_id.0);

                        top_results.push((name, name_and_id.1));
                    }
                }
                2 => {
                    if i < self.songs.len() {
                        let name_and_id = self.songs[i].clone();
                        let name = format!("{} - Song", name_and_id.0);

                        top_results.push((name, name_and_id.1));
                    }
                }
                3 => {
                    if i < self.playlists.len() {
                        let name_and_id = self.playlists[i].clone();
                        let name = format!("{} - Playlist", name_and_id.0);

                        top_results.push((name, name_and_id.1));
                    }
                }
                _ => {}
            }
        }

        top_results
    }

    pub async fn refresh(&mut self, spotify_client: &mut SpotifyClient) -> AppResult<()> {
        let url = format!(
            "search?type=artist,album,track,playlist&limit=50&q={}",
            self.query
        );
        let response = spotify_client.get(&url).await?;
        let json = response.json::<Value>().await?;

        let artist_values = json.get("artists");
        let album_values = json.get("albums");
        let song_values = json.get("tracks");
        let playlist_values = json.get("playlists");

        let artists: Vec<NameAndId> = Self::get_names_and_ids(artist_values);
        let albums: Vec<NameAndId> = Self::get_names_and_ids(album_values);
        let songs: Vec<NameAndId> = Self::get_names_and_ids(song_values);
        let playlists: Vec<NameAndId> = Self::get_names_and_ids(playlist_values);

        self.artists = artists;
        self.albums = albums;
        self.songs = songs;
        self.playlists = playlists;

        Ok(())
    }

    fn get_names_and_ids(value: Option<&Value>) -> Vec<NameAndId> {
        let mut names_and_ids: Vec<NameAndId> = vec![];

        if let Some(value) = value {
            let values_array = value.get_array_or_default("items");

            for item_value in values_array {
                let name = item_value.get_string_or_default("name");
                let id = item_value.get_string_or_default("id");

                if name != "" && id != "" {
                    names_and_ids.push((name, id));
                }
            }
        }

        names_and_ids
    }
}
