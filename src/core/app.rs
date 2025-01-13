use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    screens::{exit::ExitScreen, Screen},
    AppResult, Message,
};

#[derive(Clone)]
pub struct App {
    pub is_running: bool,
    pub history: History,
}

impl Default for App {
    fn default() -> Self {
        Self {
            is_running: true,
            history: History::default(),
        }
    }
}

impl App {
    pub fn handle_default_key_press(&self, key: KeyEvent) -> AppResult<Option<Message>> {
        match key.code {
            KeyCode::Char('H') => Ok(Some(Message::GoToPrevScreen)),
            KeyCode::Char('L') => Ok(Some(Message::GoToNextScreen)),
            KeyCode::Char('q') => Ok(Some(Message::ChangeScreen {
                new_screen: Box::new(ExitScreen::default()),
            })),
            _ => Ok(None),
        }
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
