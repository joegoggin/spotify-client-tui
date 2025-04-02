use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Paragraph, Wrap},
    Frame,
};

use crate::{
    components::Component,
    core::{
        app::App,
        spotify::{album::Album, song::Song},
    },
    AppResult, Message,
};

#[derive(Debug, Clone)]
pub struct AlbumSongList {
    pub album: Album,
    pub current_song: Song,
    pub area: Rect,
    pub max_songs_shown: u16,
    pub active_song_index: usize,
    pub song_start_index: usize,
    pub song_end_index: usize,
}

impl Default for AlbumSongList {
    fn default() -> Self {
        Self {
            album: Album::default(),
            current_song: Song::default(),
            area: Rect::default(),
            max_songs_shown: 0,
            active_song_index: 0,
            song_start_index: 0,
            song_end_index: 0,
        }
    }
}

impl AlbumSongList {
    pub fn refresh(&mut self, album: Album, song: Option<Song>, area: &Rect) {
        if let Some(song) = song {
            if song.id != self.current_song.id {
                self.current_song = song;
            }
        }

        if album.id != self.album.id {
            let max_songs_shown = area.height - 2;

            self.area = area.to_owned();
            self.max_songs_shown = max_songs_shown;
            self.song_end_index = max_songs_shown.into();
            self.song_start_index = 0;
            self.active_song_index = 0;
            self.album = album;
        }
    }

    fn get_song_style(&self, index: usize) -> Style {
        let mut style = Style::default().fg(Color::Green);

        if self.active_song_index == index {
            style = Style::default().fg(Color::White).bg(Color::Green);
        }

        style
    }
}

impl Component for AlbumSongList {
    fn view(&mut self, _: &App, frame: &mut Frame) {
        let mut song_constraits = Vec::<Constraint>::new();
        let mut song_paragraphs = Vec::<Paragraph>::new();

        for _ in 0..self.max_songs_shown {
            song_constraits.push(Constraint::Max(1))
        }

        for i in self.song_start_index..self.song_end_index {
            if i < self.album.total_songs as usize {
                let song = &self.album.songs[i];
                let mut song_string = format!("{}. {}", song.track_number, song.name);

                if song.id == self.current_song.id {
                    song_string = format!("* {} *", song_string);
                }

                let paragraph = Paragraph::new(song_string)
                    .left_aligned()
                    .style(self.get_song_style(i))
                    .wrap(Wrap { trim: false });

                song_paragraphs.push(paragraph);
            }
        }

        let song_chunks = Layout::default()
            .margin(1)
            .direction(Direction::Vertical)
            .constraints(song_constraits)
            .split(self.area);

        for i in 0..song_chunks.len() {
            if i < song_paragraphs.len() {
                frame.render_widget(song_paragraphs[i].clone(), song_chunks[i]);
            }
        }
    }

    fn tick(&mut self, _: &mut App) -> AppResult<Option<Message>> {
        todo!()
    }

    fn handle_key_press(&mut self, _: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        match key.code {
            KeyCode::Char('j') => {
                if self.active_song_index >= self.song_end_index - 1 {
                    self.song_start_index = self.song_start_index + 1;
                    self.song_end_index = self.song_end_index + 1;
                }

                if self.active_song_index < self.album.songs.len() - 1 {
                    self.active_song_index = self.active_song_index + 1;
                } else {
                    self.active_song_index = 0;
                    self.song_start_index = 0;
                    self.song_end_index = self.max_songs_shown.into();
                }

                Ok(None)
            }
            KeyCode::Char('k') => {
                if self.active_song_index <= self.song_start_index && self.active_song_index != 0 {
                    self.song_start_index = self.song_start_index - 1;
                    self.song_end_index = self.song_end_index - 1;
                }

                if self.active_song_index == 0 {
                    self.active_song_index = self.album.songs.len() - 1;
                    self.song_end_index = self.album.songs.len();

                    if self.album.total_songs < self.max_songs_shown as u64 {
                        self.song_start_index = 0;
                    } else {
                        self.song_start_index =
                            (self.album.total_songs - self.max_songs_shown as u64) as usize;
                    }
                } else {
                    self.active_song_index = self.active_song_index - 1;
                }

                Ok(None)
            }
            KeyCode::Enter => {
                let track_number = self.album.songs[self.active_song_index]
                    .clone()
                    .track_number;
                let album_id = self.album.id.clone();

                Ok(Some(Message::PlaySongOnAlbum {
                    track_number,
                    album_id,
                }))
            }
            _ => Ok(None),
        }
    }
}
