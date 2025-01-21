use ratatui::{crossterm::event::KeyEvent, Frame};

use crate::{core::app::App, AppResult, Message};

pub trait Component {
    fn view(&mut self, app: &App, frame: &mut Frame);

    fn tick(&mut self, app: &mut App) -> AppResult<Option<Message>>;

    fn handle_key_press(&mut self, app: &mut App, key: KeyEvent) -> AppResult<Option<Message>>;
}
