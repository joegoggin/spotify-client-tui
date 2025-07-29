use color_eyre::eyre::Error;
use num_format::{Locale, ToFormattedString};
use reqwest::get;
use scraper::{Html, Selector};
use serde_json::Value;

use crate::{
    utils::{string::Capitalize, value::GetOrDefault},
    AppResult,
};

use super::{client::SpotifyClient, NameAndId};

#[derive(Clone, Debug)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub top_songs: Vec<NameAndId>,
    pub albums: Vec<NameAndId>,
    pub singles: Vec<NameAndId>,
    pub genres: Vec<String>,
    pub followers: String,
    pub monthly_listeners: String,
}

impl Default for Artist {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            top_songs: vec![],
            albums: vec![],
            singles: vec![],
            genres: vec![],
            followers: String::new(),
            monthly_listeners: String::new(),
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
            genres: vec![],
            followers: String::new(),
            monthly_listeners: String::new(),
        }
    }

    pub async fn refresh(&mut self, spotify_client: &mut SpotifyClient) -> AppResult<()> {
        let url = format!("artists/{}", self.id);
        let resposne = spotify_client.get(&url).await?;
        let json = resposne.json::<Value>().await?;

        let name = json.get_string_or_default("name");
        let genre_values = json.get_array_or_default("genres");
        let mut genres: Vec<String> = vec![];
        let mut followers = String::new();

        for value in genre_values {
            if let Value::String(genre) = value {
                genres.push(Self::capitalize_genre(genre));
            }
        }

        let monthly_listeners = self.fetch_monthly_listeners().await?;

        if let Some(value) = json.get("followers") {
            followers = value
                .get_number_or_default("total")
                .to_formatted_string(&Locale::en)
        }

        let url = format!("artists/{}/top-tracks", self.id);
        let response = spotify_client.get(&url).await?;
        let json = response.json::<Value>().await?;
        let song_values = json.get_array_or_default("tracks");
        let mut top_songs: Vec<NameAndId> = vec![];

        for song in song_values {
            let song_name = song.get_string_or_default("name");
            let song_id = song.get_string_or_default("id");

            top_songs.push((song_name, song_id));
        }

        let url = format!("artists/{}/albums?limit=50&include_groups=album", self.id);
        let response = spotify_client.get(&url).await?;
        let json = response.json::<Value>().await?;
        let album_values = json.get_array_or_default("items");
        let mut albums: Vec<NameAndId> = vec![];

        for album in album_values {
            let album_name = album.get_string_or_default("name");
            let album_id = album.get_string_or_default("id");

            albums.push((album_name, album_id))
        }

        let url = format!("artists/{}/albums?limit=50&include_groups=single", self.id);
        let response = spotify_client.get(&url).await?;
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
        self.genres = genres;
        self.followers = followers;
        self.monthly_listeners = monthly_listeners;

        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.name.is_empty()
    }

    async fn fetch_monthly_listeners(&self) -> AppResult<String> {
        let mut monthly_listeners = String::new();
        let url = format!("https://open.spotify.com/artist/{}", self.id);
        let response = get(&url).await?;
        let text = response.text().await?;

        let document = Html::parse_document(&text);
        let selector = Selector::parse(r#"#main [data-testid="monthly-listeners-label"]"#)
            .map_err(|_| Error::msg("invalid selector"))?;
        let elements: Vec<_> = document.select(&selector).collect();

        if !elements.is_empty() {
            monthly_listeners =
                elements[0].inner_html().split(" ").collect::<Vec<_>>()[0].to_string();
        }

        Ok(monthly_listeners)
    }

    fn capitalize_genre(genre: String) -> String {
        if genre.contains(" ") {
            genre
                .split(" ")
                .map(|word| word.capitalize())
                .collect::<Vec<_>>()
                .join(" ")
        } else {
            genre.capitalize()
        }
    }
}
