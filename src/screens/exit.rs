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
pub struct ExitScreen;

impl Default for ExitScreen {
    fn default() -> Self {
        Self
    }
}

impl Screen for ExitScreen {
    fn get_screen_type(&self) -> ScreenType {
        ScreenType::Exit
    }

    fn get_default_key_press_enabled(&self) -> bool {
        false
    }
}

impl Component for ExitScreen {
    fn view(&mut self, app: &App, frame: &mut Frame) {
        let prompt = "Are you sure you want to exit?";

        Prompt::new_with_color(prompt, Color::Red).view(app, frame);
    }

    fn tick(&mut self, _: &mut App) -> AppResult<Option<Message>> {
        Ok(None)
    }

    fn handle_key_press(&mut self, app: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        match key.code {
            KeyCode::Char('y') => {
                app.is_running = false;
                Ok(None)
            }
            KeyCode::Char('n') => {
                return Ok(Some(Message::GoToPrevScreen));
            }
            _ => Ok(None),
        }
    }
}
