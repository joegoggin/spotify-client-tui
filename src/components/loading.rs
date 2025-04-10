use ratatui::{
    crossterm::event::KeyEvent,
    layout::{Constraint, Layout, Rect},
    style::Color,
    Frame,
};

use crate::{
    core::{
        app::{App, AppResult},
        message::Message,
    },
    widgets::paragraph::create_centered_paragraph,
};

use super::Component;

#[derive(Debug, Clone)]
pub struct Loading {
    area: Rect,
}

impl Default for Loading {
    fn default() -> Self {
        Self {
            area: Rect::default(),
        }
    }
}

impl Loading {
    pub fn set_area(&mut self, area: &Rect) {
        self.area = area.to_owned();
    }
}

impl Component for Loading {
    fn view(&mut self, _: &App, frame: &mut Frame) {
        let paragraph = create_centered_paragraph("Loading ...", Some(Color::Green));

        let chunks = Layout::default()
            .margin(5)
            .constraints(vec![Constraint::Min(1)])
            .split(self.area);

        frame.render_widget(paragraph, chunks[0]);
    }

    fn tick(&mut self, _: &mut App) -> AppResult<Option<Message>> {
        Ok(None)
    }

    fn handle_key_press(&mut self, _: &mut App, _: KeyEvent) -> AppResult<Option<Message>> {
        Ok(None)
    }
}
