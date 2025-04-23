use serde_json::Value;

use crate::{utils::value::GetOrDefault, AppResult};

use super::{client::SpotifyClient, NameAndId};

#[derive(Clone, Debug)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub top_songs: Vec<NameAndId>,
    pub albums: Vec<NameAndId>,
    pub singles: Vec<NameAndId>,
}

impl Default for Artist {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            top_songs: vec![],
            albums: vec![],
            singles: vec![],
        }
    }
}

impl Artist {
    pub fn new(id: String) -> Self {
        Self {
            id,
            name: String::new(),
            top_songs: vec![],
            albums: vec![],
            singles: vec![],
        }
    }

    pub async fn refresh(&mut self, spotify_cleint: &mut SpotifyClient) -> AppResult<()> {
        let url = format!("artists/{}", self.id);
        let resposne = spotify_cleint.get(&url).await?;
        let json = resposne.json::<Value>().await?;

        let name = json.get_string_or_default("name");

        let url = format!("artists/{}/top-tracks", self.id);
        let response = spotify_cleint.get(&url).await?;
        let json = response.json::<Value>().await?;
        let song_values = json.get_array_or_default("tracks");
        let mut top_songs: Vec<NameAndId> = vec![];

        for song in song_values {
            let song_name = song.get_string_or_default("name");
            let song_id = song.get_string_or_default("id");

            top_songs.push((song_name, song_id));
        }

        let url = format!("artists/{}/albums?limit=50&include_groups=album", self.id);
        let response = spotify_cleint.get(&url).await?;
        let json = response.json::<Value>().await?;
        let album_values = json.get_array_or_default("items");
        let mut albums: Vec<NameAndId> = vec![];

        for album in album_values {
            let album_name = album.get_string_or_default("name");
            let album_id = album.get_string_or_default("id");

            albums.push((album_name, album_id))
        }

        let url = format!("artists/{}/albums?limit=50&include_groups=single", self.id);
        let response = spotify_cleint.get(&url).await?;
        let json = response.json::<Value>().await?;
        let single_values = json.get_array_or_default("items");
        let mut singles: Vec<NameAndId> = vec![];

        for single in single_values {
            let single_name = single.get_string_or_default("name");
            let single_id = single.get_string_or_default("id");

            singles.push((single_name, single_id));
        }

        self.name = name;
        self.top_songs = top_songs;
        self.albums = albums;
        self.singles = singles;

        Ok(())
    }
}
