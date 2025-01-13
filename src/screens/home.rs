use log::debug;
use ratatui::{crossterm::event::KeyEvent, Frame};

use crate::{
    components::{screen_block::ScreenBlock, Component},
    core::app::App,
};

use super::{Screen, ScreenType};

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
    fn view(&mut self, frame: &mut Frame) {
        ScreenBlock::new("Now Playing").view(frame);
    }

    fn tick(&mut self) {}

    fn handle_key_press(
        &mut self,
        _: &mut App,
        key: KeyEvent,
    ) -> crate::AppResult<Option<crate::Message>> {
        debug!("{:#?}", key);
        Ok(None)
    }
}
