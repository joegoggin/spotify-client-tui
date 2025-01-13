use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    style::Color,
    Frame,
};

use crate::{
    components::{screen_block::ScreenBlock, Component},
    core::app::App,
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
        ScreenType::Home
    }

    fn get_default_key_press_enabled(&self) -> bool {
        false
    }
}

impl Component for ExitScreen {
    fn view(&mut self, frame: &mut Frame) {
        ScreenBlock::new_with_color("Exit Screen", Color::Red).view(frame);
    }

    fn tick(&mut self) {}

    fn handle_key_press(
        &mut self,
        app: &mut App,
        key: KeyEvent,
    ) -> crate::AppResult<Option<crate::Message>> {
        match key.code {
            KeyCode::Char('q') => {
                app.is_running = false;
                Ok(None)
            }
            _ => Ok(None),
        }
    }
}
