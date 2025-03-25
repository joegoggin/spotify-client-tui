use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Gauge, Paragraph, Wrap},
    Frame,
};

use crate::{
    components::{screen_block::ScreenBlock, Component},
    core::{
        app::App,
        config::Config,
        spotify::{client::SpotifyClient, now_playing::NowPlaying},
    },
    layout::rect::get_centered_rect,
    AppResult, Message,
};

use super::{
    auth::{create_config::CreateConfigFormScreen, show_link::ShowAuthLinkScreen},
    queue::QueueScreen,
    search::SearchScreen,
    Screen, ScreenType,
};

#[derive(Clone)]
pub struct NowPlayingScreen {
    now_playing: NowPlaying,
}

impl Default for NowPlayingScreen {
    fn default() -> Self {
        Self {
            now_playing: NowPlaying::default(),
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
}

impl Component for NowPlayingScreen {
    fn view(&mut self, app: &App, frame: &mut Frame) {
        ScreenBlock::new_with_color("Now Playing", Color::Green).view(app, frame);

        if self.now_playing.is_empty() {
            let paragraph = Paragraph::new("Now playing data not available")
                .centered()
                .wrap(Wrap { trim: false });
            let area = get_centered_rect(70, 50, frame.area());

            frame.render_widget(paragraph, area);

            return;
        }

        let song_string = format!("Song: {}", self.now_playing.song);
        let mut artist_string = "Artists: ".to_string();
        let album_string = format!("Album: {}", self.now_playing.album);
        let progress_string = self.now_playing.get_progress_string();
        let song_length_string = self.now_playing.get_song_length_string();
        let shuffle_string = self.now_playing.get_shuffle_string();

        for (index, value) in self.now_playing.artists.iter().enumerate() {
            if index == self.now_playing.artists.len() - 1 {
                artist_string = artist_string + &format!("{}", value);
            } else {
                artist_string = artist_string + &format!("{}, ", value);
            }
        }

        let song_paragraph = Paragraph::new(song_string)
            .centered()
            .wrap(Wrap { trim: false });
        let artist_paragraph = Paragraph::new(artist_string)
            .centered()
            .wrap(Wrap { trim: false });
        let album_paragrah = Paragraph::new(album_string)
            .centered()
            .wrap(Wrap { trim: false });
        let progress_paragraph = Paragraph::new(progress_string)
            .left_aligned()
            .wrap(Wrap { trim: false });
        let song_length_paragraph = Paragraph::new(song_length_string)
            .right_aligned()
            .wrap(Wrap { trim: false });
        let shuffle_paragraph = Paragraph::new(shuffle_string)
            .centered()
            .wrap(Wrap { trim: false });

        let progress_float: f64 = self.now_playing.progress as f64;
        let song_length_float: f64 = self.now_playing.song_length as f64;
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
        frame.render_widget(album_paragrah, chuncks[3]);
        frame.render_widget(progress_paragraph, progress_bar_chunks[0]);
        frame.render_widget(progress_bar_gauge, progress_bar_chunks[1]);
        frame.render_widget(song_length_paragraph, progress_bar_chunks[2]);
        frame.render_widget(shuffle_paragraph, chuncks[7]);
    }

    fn tick(&mut self, app: &mut App) -> AppResult<Option<Message>> {
        match app.spotify_client.clone() {
            Some(spotify_client) => {
                if spotify_client.credentials.is_none() {
                    let new_screen = Box::new(ShowAuthLinkScreen::new(spotify_client.auth_url));

                    return Ok(Some(Message::ChangeScreen { new_screen }));
                }

                return Ok(Some(Message::RefreshNowPlaying));
            }
            None => {
                let config = Config::new()?;
                let result = SpotifyClient::new(config.clone());

                match result {
                    Ok(spotify_client) => {
                        app.spotify_client = Some(spotify_client);

                        Ok(None)
                    }
                    Err(_) => {
                        let new_screen = Box::new(CreateConfigFormScreen::new(&config));

                        Ok(Some(Message::ChangeScreen { new_screen }))
                    }
                }
            }
        }
    }

    fn handle_key_press(&mut self, _: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        match key.code {
            KeyCode::Char(' ') => Ok(Some(Message::PausePlay)),
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
