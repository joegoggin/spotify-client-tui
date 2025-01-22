use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    auth::server::AuthServer,
    screens::{exit::ExitScreen, Screen},
    AppResult, Message,
};

use super::{config::Config, spotify::SpotifyClient};

#[derive(Clone)]
pub struct App {
    pub is_running: bool,
    pub history: History,
    pub config: Config,
    pub spotify_client: Option<SpotifyClient>,
    pub auth_server: AuthServer,
    pub default_key_press_enabled: bool,
}

impl App {
    pub fn new() -> AppResult<Self> {
        let config = Config::new()?;

        Ok(Self {
            is_running: true,
            history: History::default(),
            config,
            spotify_client: None,
            auth_server: AuthServer::default(),
            default_key_press_enabled: true,
        })
    }
}

impl App {
    pub fn handle_default_key_press(&self, key: KeyEvent) -> AppResult<Option<Message>> {
        if self.default_key_press_enabled {
            match key.code {
                KeyCode::Char('H') => return Ok(Some(Message::GoToPrevScreen)),
                KeyCode::Char('L') => return Ok(Some(Message::GoToNextScreen)),
                KeyCode::Char('q') => {
                    return Ok(Some(Message::ChangeScreen {
                        new_screen: Box::new(ExitScreen::default()),
                    }))
                }
                _ => {}
            }
        }

        Ok(None)
    }
}

#[derive(Clone)]
pub struct History {
    pub prev: Vec<Box<dyn Screen>>,
    pub next: Vec<Box<dyn Screen>>,
}

impl Default for History {
    fn default() -> Self {
        Self {
            prev: Vec::<Box<dyn Screen>>::new(),
            next: Vec::<Box<dyn Screen>>::new(),
        }
    }
}
