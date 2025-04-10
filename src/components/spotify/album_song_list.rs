use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Paragraph, Wrap},
    Frame,
};

use crate::{
    components::{loading::Loading, Component},
    core::{
        app::{App, AppResult},
        message::Message,
        spotify::album::Album,
    },
};

#[derive(Debug, Clone)]
pub struct AlbumSongList {
    pub album: Album,
    pub current_song_id: String,
    area: Rect,
    max_songs_shown: u16,
    active_song_index: usize,
    song_start_index: usize,
    song_end_index: usize,
    album_changed: bool,
}

impl Default for AlbumSongList {
    fn default() -> Self {
        Self {
            album: Album::default(),
            current_song_id: String::new(),
            area: Rect::default(),
            max_songs_shown: 0,
            active_song_index: 0,
            song_start_index: 0,
            song_end_index: 0,
            album_changed: false,
        }
    }
}

impl AlbumSongList {
    pub fn set_area(&mut self, area: &Rect) {
        let max_songs_shown = area.height - 2;

        self.area = area.to_owned();
        self.max_songs_shown = max_songs_shown;

        if self.album_changed {
            self.song_end_index = max_songs_shown.into();
            self.album_changed = false;
        }
    }

    pub fn set_album_id(&mut self, album_id: String) {
        self.album.id = album_id;
        self.song_start_index = 0;
        self.active_song_index = 0;
        self.album_changed = true;
    }

    pub fn get_active_song_id(&self) -> String {
        if !self.album.is_empty() {
            return self.album.songs[self.active_song_index].1.clone();
        }

        "".to_string()
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
    fn view(&mut self, app: &App, frame: &mut Frame) {
        if self.album.is_empty() {
            let mut loading = Loading::default();

            loading.set_area(&self.area);
            loading.view(app, frame);
            return;
        }

        let mut song_constraits = Vec::<Constraint>::new();
        let mut song_paragraphs = Vec::<Paragraph>::new();

        for _ in 0..self.max_songs_shown {
            song_constraits.push(Constraint::Max(1));
        }

        for i in self.song_start_index..self.song_end_index {
            if i < self.album.total_songs as usize {
                let song = &self.album.songs[i];
                let mut song_string = song.0.clone();

                if song.1 == self.current_song_id {
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
        Ok(None)
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
                let album_id = self.album.id.clone();

                Ok(Some(Message::PlaySongOnAlbum {
                    track_number: (self.active_song_index + 1) as u64,
                    album_id,
                }))
            }
            _ => Ok(None),
        }
    }
}
