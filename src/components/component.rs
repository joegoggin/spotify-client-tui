use ratatui::{crossterm::event::KeyEvent, Frame};

use crate::{core::app::App, AppResult, Message};

pub trait Component {
    fn view(&mut self, frame: &mut Frame);

    fn tick(&mut self);

    fn handle_key_press(&mut self, app: &mut App, key: KeyEvent) -> AppResult<Option<Message>>;
}
