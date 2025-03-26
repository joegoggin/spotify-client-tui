use async_recursion::async_recursion;
use color_eyre::eyre::eyre;
use serde_json::Value;

use crate::AppResult;

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
            return Err(eyre!("Playback not available"));
        }

        if status == 401 {
            spotify_client.refresh_auth_token().await?;

            return self.refresh(spotify_client).await;
        }

        let json = response.json::<Value>().await?;

        let mut song_string = String::new();
        let mut album_string = String::new();
        let mut artists_vec = Vec::<String>::new();
        let mut song_length_num: u64 = 0;
        let mut progress_num: u64 = 0;
        let mut shuffle_bool = false;

        if let Some(item) = json.get("item") {
            if let Some(song) = item.get("name") {
                match song {
                    Value::String(song) => song_string = song.to_owned(),
                    _ => {}
                }
            }

            if let Some(album) = item.get("album") {
                if let Some(album_name) = album.get("name") {
                    match album_name {
                        Value::String(album_name) => album_string = album_name.to_owned(),
                        _ => {}
                    }
                }
            }

            if let Some(artists) = item.get("artists") {
                match artists {
                    Value::Array(artists) => {
                        for artist in artists {
                            if let Some(artist_name) = artist.get("name") {
                                match artist_name {
                                    Value::String(artist_name) => {
                                        artists_vec.push(artist_name.to_owned())
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }

            if let Some(song_length) = item.get("duration_ms") {
                match song_length {
                    Value::Number(song_length) => {
                        if let Some(song_length) = song_length.to_owned().as_u64() {
                            song_length_num = song_length;
                        }
                    }
                    _ => {}
                }
            }
        }

        if let Some(progress) = json.get("progress_ms") {
            match progress {
                Value::Number(progress) => {
                    if let Some(progress) = progress.to_owned().as_u64() {
                        progress_num = progress;
                    }
                }
                _ => {}
            }
        }

        if let Some(shuffle) = json.get("shuffle_state") {
            match shuffle {
                Value::Bool(shuffle) => {
                    shuffle_bool = shuffle.to_owned();
                }
                _ => {}
            }
        }

        self.song = song_string;
        self.artists = artists_vec;
        self.album = album_string;
        self.song_length = song_length_num;
        self.progress = progress_num;
        self.shuffle = shuffle_bool;

        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.song == "".to_string()
            && self.artists.is_empty()
            && self.album == "".to_string()
            && self.song_length == 0
            && self.progress == 0
    }

    fn milliseconds_to_string(ms: u64) -> String {
        let total_seconds = ms / 1_000;
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;

        format!("{}:{:02}", minutes, seconds)
    }
}
