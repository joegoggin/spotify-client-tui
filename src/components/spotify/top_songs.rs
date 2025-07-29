use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    Frame,
};

use crate::{
    components::{list::List, spotify::song_info_window::SongInfoWindow, Component},
    core::{
        message::Message,
        spotify::{artist::Artist, now_playing::NowPlaying, song::Song},
    },
    widgets::block::create_block,
    App, AppResult,
};

#[derive(Clone, Debug)]
pub struct TopSongs {
    artist: Artist,
    now_playing: NowPlaying,
    song_list: List,
    info_window: SongInfoWindow,
    area: Rect,
}

impl Default for TopSongs {
    fn default() -> Self {
        Self {
            artist: Artist::default(),
            now_playing: NowPlaying::default(),
            song_list: List::default(),
            info_window: SongInfoWindow::default(),
            area: Rect::default(),
        }
    }
}

impl Component for TopSongs {
    fn view(&mut self, app: &App, frame: &mut Frame) {
        let song_list_block = create_block(Color::Green);
        let info_block = create_block(Color::Green);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(self.area);

        frame.render_widget(song_list_block, chunks[0]);
        frame.render_widget(info_block, chunks[1]);

        self.song_list.set_area(chunks[0]);
        self.info_window.set_area(&chunks[1]);

        self.song_list.view(app, frame);
        self.info_window.view(app, frame);
    }

    fn tick(&mut self, _: &mut App) -> AppResult<Option<Message>> {
        if !self.now_playing.artist_ids.is_empty() {
            if self.now_playing.artist_ids[0] != self.artist.id {
                self.artist.id = self.now_playing.artist_ids[0].clone();

                return Ok(Some(Message::RefreshArtist));
            }

            if Some(self.now_playing.song_id.clone()) != self.song_list.current_item_id {
                self.song_list.current_item_id = Some(self.now_playing.song_id.clone());
            }
        }

        if self.song_list.get_active_item().1 != self.info_window.song.id {
            self.info_window.song.id = self.song_list.get_active_item().1;

            return Ok(Some(Message::RefreshSong));
        }

        if self.artist.top_songs != self.song_list.items {
            self.song_list.set_items(self.artist.top_songs.clone());
        }

        Ok(Some(Message::RefreshNowPlaying))
    }

    fn handle_key_press(&mut self, app: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        if let Some(message) = self.song_list.handle_key_press(app, key)? {
            return Ok(Some(message));
        }

        match key.code {
            KeyCode::Enter => Ok(Some(Message::PlaySongs {
                offset: self.song_list.active_index,
                songs: self.song_list.items.clone(),
            })),
            _ => Ok(None),
        }
    }

    fn get_area(&mut self) -> Rect {
        self.area
    }

    fn set_area(&mut self, area: Rect) {
        self.area = area;
    }

    fn get_now_playing(&mut self) -> Option<&mut NowPlaying> {
        Some(&mut self.now_playing)
    }

    fn get_artist(&mut self) -> Option<&mut Artist> {
        Some(&mut self.artist)
    }

    fn get_song(&mut self) -> Option<&mut Song> {
        Some(&mut self.info_window.song)
    }
}
