use log::debug;
use ratatui::{
    crossterm::event::KeyEvent,
    layout::{Constraint, Layout},
    style::Color,
    widgets::{Paragraph, Wrap},
    Frame,
};

use crate::{
    components::{screen_block::ScreenBlock, Component},
    core::{app::App, config::Config, spotify::SpotifyClient},
    AppResult, Message,
};

use super::{
    auth::{create_config::CreateConfigFormScreen, show_link::ShowAuthLinkScreen},
    Screen, ScreenType,
};

#[derive(Clone)]
pub struct NowPlayingScreen;

impl Default for NowPlayingScreen {
    fn default() -> Self {
        Self
    }
}

impl Screen for NowPlayingScreen {
    fn get_screen_type(&self) -> ScreenType {
        ScreenType::NowPlayingScreen
    }
}

impl Component for NowPlayingScreen {
    fn view(&mut self, app: &App, frame: &mut Frame) {
        ScreenBlock::new_with_color("Now Playing", Color::Green).view(app, frame);

        if let Some(spotify_client) = app.spotify_client.clone() {
            if let Some(now_playing) = spotify_client.now_playing {
                let song_string = format!("Song: {}", now_playing.song);
                let mut artist_string = "Artists: ".to_string();
                let album_string = format!("Album: {}", now_playing.album);

                for (index, value) in now_playing.artists.iter().enumerate() {
                    if index == now_playing.artists.len() - 1 {
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

                let chuncks = Layout::default()
                    .margin(5)
                    .constraints(vec![
                        Constraint::Min(1),
                        Constraint::Min(1),
                        Constraint::Min(1),
                    ])
                    .split(frame.area());

                frame.render_widget(song_paragraph, chuncks[0]);
                frame.render_widget(artist_paragraph, chuncks[1]);
                frame.render_widget(album_paragrah, chuncks[2]);
            }
        }
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
        debug!("{:#?}", key);
        Ok(None)
    }
}
