use log::debug;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    style::Color,
    Frame,
};

use crate::{
    components::{screen_block::ScreenBlock, Component},
    core::{
        app::{App, AppResult},
        config::Config,
        message::Message,
        spotify::{artist::Artist, client::SpotifyClient, now_playing::NowPlaying},
    },
    screens::{
        auth::{create_config::CreateConfigFormScreen, show_link::ShowAuthLinkScreen},
        Screen, ScreenType,
    },
};

#[derive(Clone)]
pub struct ViewArtistScreen {
    now_playing: NowPlaying,
    artist: Artist,
}

impl Default for ViewArtistScreen {
    fn default() -> Self {
        Self {
            now_playing: NowPlaying::default(),
            artist: Artist::default(),
        }
    }
}

impl Screen for ViewArtistScreen {
    fn get_screen_type(&self) -> ScreenType {
        ScreenType::ViewArtistScreen
    }

    fn get_now_playing(&mut self) -> Option<&mut NowPlaying> {
        Some(&mut self.now_playing)
    }

    fn get_artist(&mut self) -> Option<&mut Artist> {
        Some(&mut self.artist)
    }
}

impl Component for ViewArtistScreen {
    fn view(&mut self, app: &App, frame: &mut Frame) {
        ScreenBlock::new_with_color("View Artist", Color::Green).view(app, frame);
    }

    fn tick(&mut self, _: &mut App) -> AppResult<Option<Message>> {
        if !self.now_playing.is_empty() {
            if self.now_playing.artist_ids[0] != self.artist.id {
                self.artist.id = self.now_playing.artist_ids[0].clone();
            }
        }

        Ok(Some(Message::RefreshNowPlaying))
    }

    fn handle_key_press(&mut self, _: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        match key.code {
            KeyCode::Enter => Ok(Some(Message::RefreshArtist)),
            _ => Ok(None),
        }
    }
}
