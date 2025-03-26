use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    style::Color,
    Frame,
};

use crate::{
    components::{prompt::Prompt, Component},
    core::app::App,
    AppResult, Message,
};

use super::{Screen, ScreenType};

#[derive(Clone)]
pub struct ErrorScreen {
    message: Option<String>,
}

impl Default for ErrorScreen {
    fn default() -> Self {
        Self { message: None }
    }
}

impl ErrorScreen {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: Some(message.into()),
        }
    }
}

impl Screen for ErrorScreen {
    fn get_screen_type(&self) -> ScreenType {
        ScreenType::QueueScreen
    }
}

impl Component for ErrorScreen {
    fn view(&mut self, app: &App, frame: &mut Frame) {
        let message = match &self.message {
            Some(message) => {
                format!(
                    "Something went wrong!\n\nError: {}\n\nWould you like to try again?\n\n",
                    message
                )
            }
            None => {
                format!("Something went wrong!\n\nWould you like to try again?\n\n")
            }
        };

        Prompt::new_with_color(message, Color::Red).view(app, frame);
    }

    fn tick(&mut self, _: &mut App) -> AppResult<Option<Message>> {
        Ok(None)
    }

    fn handle_key_press(&mut self, app: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        match key.code {
            KeyCode::Char('y') => Ok(Some(Message::GoToPrevScreen)),
            KeyCode::Char('n') => {
                app.is_running = false;
                Ok(None)
            }
            _ => Ok(None),
        }
    }
}
