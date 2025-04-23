use log::debug;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::Gauge,
    Frame,
};

use crate::{
    components::{loading::Loading, screen_block::ScreenBlock, Component},
    core::{
        app::{App, AppResult},
        message::Message,
        spotify::{now_playing::NowPlaying, song::Song},
    },
    widgets::paragraph::{
        create_centered_paragraph, create_left_aligned_paragraph, create_right_aligned_paragraph,
    },
};

use super::{queue::QueueScreen, search::SearchScreen, Screen, ScreenType};

#[derive(Debug, Clone)]
pub struct NowPlayingScreen {
    now_playing: NowPlaying,
    song: Song,
}

impl Default for NowPlayingScreen {
    fn default() -> Self {
        Self {
            now_playing: NowPlaying::default(),
            song: Song::default(),
        }
    }
}

impl Screen for NowPlayingScreen {
    fn get_screen_type(&self) -> ScreenType {
        ScreenType::NowPlayingScreen
    }

    fn get_now_playing(&mut self) -> Option<&mut NowPlaying> {
        Some(&mut self.now_playing)
    }

    fn get_song(&mut self) -> Option<&mut Song> {
        Some(&mut self.song)
    }
}

impl Component for NowPlayingScreen {
    fn view(&mut self, app: &App, frame: &mut Frame) {
        ScreenBlock::new_with_color("Now Playing", Color::Green).view(app, frame);

        if self.now_playing.is_empty() || self.song.is_empty() {
            Loading::default().view(app, frame);
            return;
        }

        let song_string = format!("Song: {}", self.song.name);
        let mut artist_string = "Artists: ".to_string();
        let album_string = format!("Album: {}", self.song.album_name);
        let progress_string = self.now_playing.get_progress_string();
        let song_length_string = self.song.get_song_length_string();
        let shuffle_string = self.now_playing.get_shuffle_string();

        for (index, value) in self.song.artist_names.iter().enumerate() {
            if index == self.song.artist_names.len() - 1 {
                artist_string = artist_string + &format!("{}", value);
            } else {
                artist_string = artist_string + &format!("{}, ", value);
            }
        }

        let song_paragraph = create_centered_paragraph(&song_string, Some(Color::Green));
        let artist_paragraph = create_centered_paragraph(&artist_string, Some(Color::Green));
        let album_paragraph = create_centered_paragraph(&album_string, Some(Color::Green));
        let progress_paragraph =
            create_left_aligned_paragraph(&progress_string, Some(Color::Green));
        let song_length_paragraph =
            create_right_aligned_paragraph(&song_length_string, Some(Color::Green));
        let shuffle_paragraph = create_centered_paragraph(&shuffle_string, Some(Color::Green));

        let progress_float: f64 = self.now_playing.progress as f64;
        let song_length_float: f64 = self.song.song_length as f64;
        let percent: u16 = ((progress_float / song_length_float) * 100.0) as u16;

        let progress_bar_gauge = Gauge::default()
            .percent(percent)
            .label("")
            .gauge_style(Style::default().fg(Color::Green));

        let chuncks = Layout::default()
            .margin(3)
            .constraints(vec![
                Constraint::Min(1),
                Constraint::Min(1),
                Constraint::Min(1),
                Constraint::Min(1),
                Constraint::Min(1),
                Constraint::Max(1),
                Constraint::Max(1),
                Constraint::Max(1),
            ])
            .split(frame.area());

        let progress_bar_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(chuncks[5]);

        frame.render_widget(song_paragraph, chuncks[1]);
        frame.render_widget(artist_paragraph, chuncks[2]);
        frame.render_widget(album_paragraph, chuncks[3]);
        frame.render_widget(progress_paragraph, progress_bar_chunks[0]);
        frame.render_widget(progress_bar_gauge, progress_bar_chunks[1]);
        frame.render_widget(song_length_paragraph, progress_bar_chunks[2]);
        frame.render_widget(shuffle_paragraph, chuncks[7]);
    }

    fn tick(&mut self, _: &mut App) -> AppResult<Option<Message>> {
        if self.now_playing.song_id != self.song.id {
            self.song.id = self.now_playing.song_id.clone();

            return Ok(Some(Message::RefreshSong));
        }

        Ok(Some(Message::RefreshNowPlaying))
    }

    fn handle_key_press(&mut self, _: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        match key.code {
            KeyCode::Char('p') => Ok(Some(Message::PausePlay)),
            KeyCode::Char('s') => Ok(Some(Message::Shuffle)),
            KeyCode::Char('l') => Ok(Some(Message::NextSong)),
            KeyCode::Char('h') => Ok(Some(Message::PrevSong)),
            KeyCode::Char('q') => Ok(Some(Message::ChangeScreen {
                new_screen: Box::new(QueueScreen::default()),
            })),
            KeyCode::Char('/') => Ok(Some(Message::ChangeScreen {
                new_screen: Box::new(SearchScreen::default()),
            })),
            _ => Ok(None),
        }
    }
}
