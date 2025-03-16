use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    screens::{exit::ExitScreen, Screen},
    AppResult, Message,
};

use super::spotify::SpotifyClient;

#[derive(Clone)]
pub struct App {
    pub is_running: bool,
    pub history: History,
    pub spotify_client: Option<SpotifyClient>,
    pub default_key_press_enabled: bool,
}

impl App {
    pub fn new() -> AppResult<Self> {
        Ok(Self {
            is_running: true,
            history: History::default(),
            spotify_client: None,
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
                KeyCode::Esc => {
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
