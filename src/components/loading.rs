use ratatui::{
    layout::{Constraint, Layout},
    style::Color,
    widgets::Paragraph,
    Frame,
};

use crate::{core::app::App, widgets::paragraph::create_centered_paragraph, AppResult, Message};

use super::Component;

#[derive(Debug, Clone)]
pub struct Loading;

impl Default for Loading {
    fn default() -> Self {
        Self
    }
}

impl Component for Loading {
    fn view(&mut self, _: &App, frame: &mut Frame) {
        let paragraph = create_centered_paragraph("Loading ...", Some(Color::Green));

        let chunks = Layout::default()
            .margin(5)
            .constraints(vec![Constraint::Min(1)])
            .split(frame.area());

        frame.render_widget(paragraph, chunks[0]);
    }

    fn tick(&mut self, _: &mut App) -> AppResult<Option<Message>> {
        Ok(None)
    }

    fn handle_key_press(
        &mut self,
        _: &mut App,
        _: ratatui::crossterm::event::KeyEvent,
    ) -> crate::AppResult<Option<crate::Message>> {
        Ok(None)
    }
}
