use ratatui::{
    crossterm::event::KeyEvent,
    layout::{Alignment, Rect},
    style::Color,
    Frame,
};

use crate::{
    components::Component, core::message::Message, widgets::block::create_titled_block, App,
    AppResult,
};

#[derive(Clone)]
pub struct ArtistAlbums {
    area: Rect,
}

impl Default for ArtistAlbums {
    fn default() -> Self {
        Self {
            area: Rect::default(),
        }
    }
}

impl Component for ArtistAlbums {
    fn view(&mut self, _: &App, frame: &mut Frame) {
        let block = create_titled_block("Albums", Alignment::Center, Color::Green);

        frame.render_widget(block, self.area);
    }

    fn tick(&mut self, _: &mut App) -> AppResult<Option<Message>> {
        Ok(None)
    }

    fn handle_key_press(&mut self, _: &mut App, _: KeyEvent) -> AppResult<Option<Message>> {
        Ok(None)
    }

    fn get_area(&mut self) -> Rect {
        self.area
    }

    fn set_area(&mut self, area: Rect) {
        self.area = area;
    }
}
