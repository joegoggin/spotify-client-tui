use log::debug;
use ratatui::{crossterm::event::KeyEvent, style::Color, Frame};

use crate::{
    components::{screen_block::ScreenBlock, Component},
    core::{
        app::{App, AppResult},
        config::Config,
        message::Message,
        spotify::client::SpotifyClient,
    },
};

use super::{
    auth::{create_config::CreateConfigFormScreen, show_link::ShowAuthLinkScreen},
    Screen, ScreenType,
};

#[derive(Clone)]
pub struct QueueScreen;

impl Default for QueueScreen {
    fn default() -> Self {
        Self
    }
}

impl Screen for QueueScreen {
    fn get_screen_type(&self) -> ScreenType {
        ScreenType::QueueScreen
    }
}

impl Component for QueueScreen {
    fn view(&mut self, app: &App, frame: &mut Frame) {
        ScreenBlock::new_with_color("Queue", Color::Green).view(app, frame);
    }

    fn tick(&mut self, app: &mut App) -> AppResult<Option<Message>> {
        match app.spotify_client.clone() {
            Some(spotify_client) => {
                if spotify_client.credentials.is_none() {
                    let new_screen = Box::new(ShowAuthLinkScreen::new(spotify_client.auth_url));

                    return Ok(Some(Message::ChangeScreen { new_screen }));
                }

                Ok(None)
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
