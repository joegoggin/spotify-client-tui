use ratatui::{crossterm::event::KeyEvent, layout::Alignment, style::Color, Frame};

use crate::{core::app::App, widgets::block::create_titled_block, AppResult, Message};

use super::Component;

#[derive(Clone)]
pub struct ScreenBlock {
    title: String,
    color: Color,
}

impl ScreenBlock {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            color: Color::White,
        }
    }

    pub fn new_with_color(title: impl Into<String>, color: Color) -> Self {
        Self {
            title: title.into(),
            color,
        }
    }
}

impl Component for ScreenBlock {
    fn view(&mut self, _: &App, frame: &mut Frame) {
        let container = create_titled_block(&self.title, Alignment::Center, self.color);

        frame.render_widget(container, frame.area());
    }

    fn tick(&mut self, _: &mut App) -> AppResult<Option<Message>> {
        Ok(None)
    }

    fn handle_key_press(&mut self, _: &mut App, _: KeyEvent) -> AppResult<Option<Message>> {
        Ok(None)
    }
}
