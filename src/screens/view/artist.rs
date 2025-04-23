use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    style::Color,
    Frame,
};

use crate::{
    components::{
        screen_block::ScreenBlock,
        spotify::{
            artist_albums::ArtistAlbums, artist_singles::ArtistSingles, top_songs::TopSongs,
        },
        tabs::{tab::Tab, tabbed_view::TabbedView},
        Component,
    },
    core::{
        app::{App, AppResult},
        message::Message,
        spotify::{artist::Artist, now_playing::NowPlaying},
    },
    screens::{Screen, ScreenType},
};

#[derive(Clone)]
pub struct ViewArtistScreen {
    now_playing: NowPlaying,
    artist: Artist,
    tabbed_view: TabbedView,
}

impl Default for ViewArtistScreen {
    fn default() -> Self {
        let top_songs = TopSongs::default();
        let albums = ArtistAlbums::default();
        let singles = ArtistSingles::default();

        let mut tabs: Vec<Tab> = vec![];
        tabs.push(Tab::new(
            "Top Songs",
            KeyCode::Char('1'),
            Box::new(top_songs),
        ));
        tabs.push(Tab::new("Albums", KeyCode::Char('2'), Box::new(albums)));
        tabs.push(Tab::new("Singles", KeyCode::Char('3'), Box::new(singles)));

        Self {
            now_playing: NowPlaying::default(),
            artist: Artist::default(),
            tabbed_view: TabbedView::new(tabs),
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

        self.tabbed_view.view(app, frame);
    }

    fn tick(&mut self, app: &mut App) -> AppResult<Option<Message>> {
        if !self.now_playing.is_empty() {
            if self.now_playing.artist_ids[0] != self.artist.id {
                self.artist.id = self.now_playing.artist_ids[0].clone();
            }
        }

        if let Some(message) = self.tabbed_view.tick(app)? {
            return Ok(Some(message));
        }

        Ok(Some(Message::RefreshNowPlaying))
    }

    fn handle_key_press(&mut self, app: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        self.tabbed_view.handle_key_press(app, key)
    }
}
