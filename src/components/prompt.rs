use ratatui::{
    crossterm::event::KeyEvent,
    layout::{Constraint, Layout},
    style::Color,
    Frame,
};

use crate::{
    core::{app::App, message::Message},
    layout::rect::get_centered_rect,
    widgets::{block::create_block, paragraph::create_centered_paragraph},
    AppResult,
};

use super::Component;

#[derive(Clone)]
pub struct Prompt {
    prompt: String,
    color: Color,
}

impl Prompt {
    #[allow(dead_code)]
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            color: Color::White,
        }
    }

    pub fn new_with_color(prompt: impl Into<String>, color: Color) -> Self {
        Self {
            prompt: prompt.into(),
            color,
        }
    }
}

impl Component for Prompt {
    fn view(&mut self, _: &App, frame: &mut Frame) {
        let container_area = get_centered_rect(70, 60, frame.area());
        let container = create_block(self.color);
        let prompt_paragraph = create_centered_paragraph(&self.prompt, Some(self.color));
        let options = "Press 'y' for yes or 'n' for no";
        let options_paragraph = create_centered_paragraph(&options, Some(self.color));

        let chunks = Layout::default()
            .margin(5)
            .constraints([Constraint::Min(5), Constraint::Max(10), Constraint::Min(1)])
            .split(container_area);

        frame.render_widget(container, container_area);
        frame.render_widget(prompt_paragraph, chunks[0]);
        frame.render_widget(options_paragraph, chunks[2]);
    }

    fn tick(&mut self, _: &mut App) -> AppResult<Option<Message>> {
        Ok(None)
    }

    fn handle_key_press(&mut self, _: &mut App, _: KeyEvent) -> AppResult<Option<Message>> {
        Ok(None)
    }
}
