use ratatui::{
    crossterm::event::KeyEvent,
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    Frame,
};

use crate::{
    components::Component,
    core::{
        app::App,
        spotify::{album::Album, song::Song},
    },
    widgets::paragraph::create_left_aligned_paragraph,
    AppResult, Message,
};

#[derive(Debug, Clone)]
pub struct SongInfoWindow {
    pub song: Song,
    pub album: Album,
    pub area: Rect,
}

impl Default for SongInfoWindow {
    fn default() -> Self {
        Self {
            song: Song::default(),
            album: Album::default(),
            area: Rect::default(),
        }
    }
}

impl SongInfoWindow {
    pub fn refresh(&mut self, album: Album, song: Song, area: &Rect) {
        if album.id != self.album.id {
            self.album = album;
        }

        if song.id != self.song.id {
            self.song = song;
        }

        self.area = area.to_owned();
    }
}

impl Component for SongInfoWindow {
    fn view(&mut self, _: &App, frame: &mut Frame) {
        let song_string = format!("Song: {}", self.song.name);
        let artists_string = format!("Artists: {}", self.song.get_artists_string());
        let album_string = format!("Album: {}", self.album.name);
        let song_length_string = format!("Song Length: {}", self.song.get_song_lenth_string());
        let year_string = format!("Year: {}", self.album.year);
        let disk_string = format!("Disk: {}", self.song.disk_number);
        let track_string = format!("Track {}", self.song.track_number);

        let song_paragraph = create_left_aligned_paragraph(&song_string, Some(Color::Green));
        let artists_paragraph = create_left_aligned_paragraph(&artists_string, Some(Color::Green));
        let album_paragraph = create_left_aligned_paragraph(&album_string, Some(Color::Green));
        let year_paragraph = create_left_aligned_paragraph(&year_string, Some(Color::Green));
        let song_length_paragraph =
            create_left_aligned_paragraph(&song_length_string, Some(Color::Green));
        let disk_paragraph = create_left_aligned_paragraph(&disk_string, Some(Color::Green));
        let track_paragraph = create_left_aligned_paragraph(&track_string, Some(Color::Green));

        let mut info_constraints = Vec::<Constraint>::new();

        for _ in 0..3 {
            info_constraints.push(Constraint::Max(4))
        }

        for _ in 0..5 {
            info_constraints.push(Constraint::Max(1))
        }

        let info_chunks = Layout::default()
            .margin(1)
            .direction(Direction::Vertical)
            .constraints(info_constraints)
            .split(self.area);

        frame.render_widget(song_paragraph, info_chunks[0]);
        frame.render_widget(artists_paragraph, info_chunks[1]);
        frame.render_widget(album_paragraph, info_chunks[2]);
        frame.render_widget(year_paragraph, info_chunks[3]);
        frame.render_widget(song_length_paragraph, info_chunks[4]);
        frame.render_widget(disk_paragraph, info_chunks[5]);
        frame.render_widget(track_paragraph, info_chunks[6]);
    }

    fn tick(&mut self, _: &mut App) -> AppResult<Option<Message>> {
        Ok(None)
    }

    fn handle_key_press(&mut self, _: &mut App, _: KeyEvent) -> crate::AppResult<Option<Message>> {
        Ok(None)
    }
}
