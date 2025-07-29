use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    Frame,
};

use crate::{
    components::{list::List, Component},
    core::{
        message::Message,
        spotify::{album::Album, artist::Artist, now_playing::NowPlaying, song::Song},
    },
    widgets::block::create_block,
    App, AppResult,
};

use super::song_info_window::SongInfoWindow;

#[derive(Clone, PartialEq)]
enum ListType {
    Single,
    Song,
}

#[derive(Clone)]
pub struct ArtistSingles {
    area: Rect,
    now_playing: NowPlaying,
    artist: Artist,
    album: Album,
    single_list: List,
    song_list: List,
    info_window: SongInfoWindow,
    active_list_type: ListType,
}

impl Default for ArtistSingles {
    fn default() -> Self {
        let mut song_list = List::default();

        song_list.is_active = false;

        Self {
            area: Rect::default(),
            now_playing: NowPlaying::default(),
            artist: Artist::default(),
            album: Album::default(),
            single_list: List::default(),
            song_list,
            info_window: SongInfoWindow::default(),
            active_list_type: ListType::Single,
        }
    }
}

impl Component for ArtistSingles {
    fn view(&mut self, app: &App, frame: &mut Frame) {
        let list_block = create_block(Color::Green);
        let info_block = create_block(Color::Green);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(self.area);

        frame.render_widget(list_block, chunks[0]);
        frame.render_widget(info_block, chunks[1]);

        match self.active_list_type {
            ListType::Single => {
                self.single_list.set_area(chunks[0]);
                self.song_list.set_area(chunks[1]);

                self.single_list.view(app, frame);
                self.song_list.view(app, frame);
            }
            ListType::Song => {
                self.song_list.set_area(chunks[0]);
                self.info_window.set_area(&chunks[1]);

                self.song_list.view(app, frame);
                self.info_window.view(app, frame);
            }
        }
    }

    fn tick(&mut self, _: &mut App) -> AppResult<Option<Message>> {
        if !self.now_playing.is_empty() {
            if self.now_playing.artist_ids[0] != self.artist.id {
                self.artist.id = self.now_playing.artist_ids[0].clone();

                return Ok(Some(Message::RefreshArtist));
            }
        }

        if self.single_list.get_active_item().1 != self.album.id {
            self.album.id = self.single_list.get_active_item().1;

            return Ok(Some(Message::RefreshAlbum));
        }

        if self.artist.singles != self.single_list.items {
            self.single_list.set_items(self.artist.singles.clone());
        }

        if self.album.songs != self.song_list.items {
            self.song_list.set_items(self.album.songs.clone());
        }

        if Some(self.now_playing.album_id.clone()) != self.single_list.current_item_id {
            self.single_list.current_item_id = Some(self.now_playing.album_id.clone());
        }

        if Some(self.now_playing.song_id.clone()) != self.song_list.current_item_id {
            self.song_list.current_item_id = Some(self.now_playing.song_id.clone());
        }

        if self.song_list.get_active_item().1 != self.info_window.song.id {
            self.info_window.song.id = self.song_list.get_active_item().1;

            return Ok(Some(Message::RefreshSong));
        }

        Ok(Some(Message::RefreshNowPlaying))
    }

    fn handle_key_press(&mut self, app: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        let message = match self.active_list_type {
            ListType::Single => self.single_list.handle_key_press(app, key)?,
            ListType::Song => self.song_list.handle_key_press(app, key)?,
        };

        if let Some(message) = message {
            return Ok(Some(message));
        }

        match key.code {
            KeyCode::Char('l') => {
                if self.active_list_type == ListType::Single {
                    self.single_list.is_active = false;
                    self.song_list.is_active = true;
                    self.active_list_type = ListType::Song;
                }
            }
            KeyCode::Char('h') => {
                if self.active_list_type == ListType::Song {
                    self.song_list.is_active = false;
                    self.single_list.is_active = true;
                    self.active_list_type = ListType::Single;
                    self.song_list.active_index = 0;
                }
            }
            KeyCode::Enter => match self.active_list_type {
                ListType::Single => {
                    self.single_list.is_active = false;
                    self.song_list.is_active = true;
                    self.active_list_type = ListType::Song;
                }
                ListType::Song => {
                    return Ok(Some(Message::PlaySongs {
                        offset: self.song_list.active_index,
                        songs: self.song_list.items.clone(),
                    }));
                }
            },

            _ => {}
        }

        Ok(None)
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

    fn get_album(&mut self) -> Option<&mut Album> {
        Some(&mut self.album)
    }

    fn get_song(&mut self) -> Option<&mut Song> {
        Some(&mut self.info_window.song)
    }
}
