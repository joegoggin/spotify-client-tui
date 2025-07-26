use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    style::Color,
    Frame,
};

use crate::{
    components::{
        screen_block::ScreenBlock,
        spotify::{
            artist_albums::ArtistAlbums, artist_info::ArtistInfo, artist_singles::ArtistSingles,
            top_songs::TopSongs,
        },
        tabs::{tab::Tab, tabbed_view::TabbedView},
        Component,
    },
    core::{
        app::{App, AppResult},
        message::Message,
        spotify::{album::Album, artist::Artist, now_playing::NowPlaying, song::Song},
    },
    screens::{Screen, ScreenType},
};

#[derive(Clone)]
pub struct ViewArtistScreen {
    tabbed_view: TabbedView,
}

impl Default for ViewArtistScreen {
    fn default() -> Self {
        let top_songs = TopSongs::default();
        let albums = ArtistAlbums::default();
        let singles = ArtistSingles::default();
        let artist_info = ArtistInfo::default();

        let mut tabs: Vec<Tab> = vec![];
        tabs.push(Tab::new(
            "Artist Info",
            KeyCode::Char('1'),
            Box::new(artist_info),
        ));
        tabs.push(Tab::new(
            "Top Songs",
            KeyCode::Char('2'),
            Box::new(top_songs),
        ));
        tabs.push(Tab::new("Albums", KeyCode::Char('3'), Box::new(albums)));
        tabs.push(Tab::new(
            "Singles and EPs",
            KeyCode::Char('4'),
            Box::new(singles),
        ));

        Self {
            tabbed_view: TabbedView::new(tabs),
        }
    }
}

impl ViewArtistScreen {
    fn get_title(&self) -> String {
        let mut title = "View Artist".to_string();

        if let Some(componet) = self.tabbed_view.clone().get_active_component() {
            if let Some(artist) = componet.get_artist() {
                if !artist.is_empty() {
                    title = format!("{}", artist.name)
                }
            }
        }

        title
    }
}

impl Screen for ViewArtistScreen {
    fn get_screen_type(&self) -> ScreenType {
        ScreenType::ViewArtistScreen
    }
}

impl Component for ViewArtistScreen {
    fn view(&mut self, app: &App, frame: &mut Frame) {
        ScreenBlock::new_with_color(self.get_title(), Color::Green).view(app, frame);

        self.tabbed_view.view(app, frame);
    }

    fn tick(&mut self, app: &mut App) -> AppResult<Option<Message>> {
        self.tabbed_view.tick(app)
    }

    fn handle_key_press(&mut self, app: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        self.tabbed_view.handle_key_press(app, key)
    }

    fn get_now_playing(&mut self) -> Option<&mut NowPlaying> {
        self.tabbed_view.get_now_playing()
    }

    fn get_artist(&mut self) -> Option<&mut Artist> {
        self.tabbed_view.get_artist()
    }

    fn get_song(&mut self) -> Option<&mut Song> {
        self.tabbed_view.get_song()
    }

    fn get_album(&mut self) -> Option<&mut Album> {
        self.tabbed_view.get_album()
    }
}
