use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    Frame,
};

use crate::{components::Component, core::message::Message, App, AppResult};

#[derive(Clone)]
pub struct Tab {
    pub title: String,
    pub key: KeyCode,
    pub component: Box<dyn Component>,
}

impl Tab {
    pub fn new(title: impl Into<String>, key: KeyCode, component: Box<dyn Component>) -> Self {
        Self {
            title: title.into(),
            key,
            component,
        }
    }

    pub fn view(&mut self, app: &App, frame: &mut Frame) {
        self.component.view(app, frame)
    }

    pub fn tick(&mut self, app: &mut App) -> AppResult<Option<Message>> {
        self.component.tick(app)
    }

    pub fn handle_key_press(&mut self, app: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        self.component.handle_key_press(app, key)
    }
}
