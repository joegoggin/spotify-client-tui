use log::debug;
use ratatui::{crossterm::event::KeyEvent, style::Color, Frame};

use crate::{
    components::{screen_block::ScreenBlock, Component},
    core::{app::App, spotify::SpotifyClient},
    AppResult, Message,
};

use super::{
    auth::{create_config::CreateConfigFormScreen, log_in::LogInFormScreen},
    Screen, ScreenType,
};

#[derive(Clone)]
pub struct HomeScreen;

impl Default for HomeScreen {
    fn default() -> Self {
        Self
    }
}

impl Screen for HomeScreen {
    fn get_screen_type(&self) -> ScreenType {
        ScreenType::Home
    }
}

impl Component for HomeScreen {
    fn view(&mut self, app: &App, frame: &mut Frame) {
        ScreenBlock::new_with_color("Spotify Client TUI", Color::Green).view(app, frame);
    }

    fn tick(&mut self, app: &mut App) -> AppResult<Option<Message>> {
        match app.spotify_client.clone() {
            Some(spotify_client) => {
                if spotify_client.access_token.is_none() {
                    let new_screen = Box::new(LogInFormScreen::new(spotify_client.auth_url));

                    return Ok(Some(Message::ChangeScreen { new_screen }));
                }

                Ok(None)
            }
            None => {
                let result = SpotifyClient::new(&app.config);

                match result {
                    Ok(spotify_client) => {
                        app.spotify_client = Some(spotify_client);

                        Ok(None)
                    }
                    Err(_) => {
                        let new_screen = Box::new(CreateConfigFormScreen::new(&app.config));

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
