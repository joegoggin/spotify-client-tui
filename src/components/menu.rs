use std::usize;

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

use crate::{core::app::App, widgets::block::create_block, AppResult, Message};

use super::Component;

#[derive(Clone)]
pub struct Menu {
    pub show_menu: bool,
    pub current_menu_index: usize,
    pub menu_items: Vec<String>,
    pub area: Option<Rect>,
}

impl Default for Menu {
    fn default() -> Self {
        Self {
            show_menu: true,
            current_menu_index: 0,
            menu_items: Vec::<String>::new(),
            area: None,
        }
    }
}

impl Menu {
    pub fn new(menu_items: Vec<String>) -> Self {
        Self {
            show_menu: true,
            current_menu_index: 0,
            menu_items,
            area: None,
        }
    }

    pub fn get_current_item(&self) -> String {
        self.menu_items[self.current_menu_index].clone()
    }

    fn get_item_color(&self, index: usize) -> Color {
        if self.current_menu_index == index {
            return Color::Blue;
        }

        Color::White
    }
}
impl Component for Menu {
    fn view(&mut self, _: &App, frame: &mut Frame) {
        if self.show_menu {
            let mut constraints: Vec<Constraint> = Vec::new();
            let mut items: Vec<Paragraph> = Vec::new();
            let mut area = frame.area();

            if let Some(menu_area) = self.area.clone() {
                area = menu_area;
            }

            constraints.push(Constraint::Percentage(10));

            for (index, item) in self.menu_items.iter().enumerate() {
                constraints.push(Constraint::Max(3));
                let color = self.get_item_color(index);
                let title = item.to_string();

                let block = create_block(color.clone());
                let paragraph = Paragraph::new(title)
                    .centered()
                    .block(block)
                    .style(Style::default().fg(color));

                items.push(paragraph);
            }

            let menu_chunks = Layout::default()
                .margin(5)
                .constraints(constraints)
                .split(area);

            for i in 0..self.menu_items.len() {
                let paragraph = items[i].clone();

                frame.render_widget(paragraph, menu_chunks[i + 1])
            }
        }
    }

    fn tick(&mut self, _: &mut App) -> AppResult<Option<Message>> {
        Ok(None)
    }

    fn handle_key_press(&mut self, _: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        match key.code {
            KeyCode::Char('j') => {
                if self.current_menu_index < self.menu_items.len() - 1 {
                    self.current_menu_index = self.current_menu_index + 1;
                } else {
                    self.current_menu_index = 0;
                }
                Ok(None)
            }
            KeyCode::Char('k') => {
                if self.current_menu_index > 0 {
                    self.current_menu_index = self.current_menu_index - 1;
                } else {
                    self.current_menu_index = self.menu_items.len() - 1;
                }
                Ok(None)
            }
            _ => Ok(None),
        }
    }
}
