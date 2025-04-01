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
    pub current_page: usize,
}

impl Default for Menu {
    fn default() -> Self {
        Self {
            show_menu: true,
            current_menu_index: 0,
            menu_items: Vec::<String>::new(),
            area: None,
            current_page: 1,
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
            current_page: 1,
        }
    }

    pub fn get_current_item(&self) -> String {
        self.menu_items[self.current_menu_index].clone()
    }

    fn get_item_color(&self, index: usize) -> Color {
        if self.current_menu_index == index {
            return Color::Green;
        }

        Color::White
    }

    fn get_total_pages(&self) -> usize {
        let mut total_pages = self.menu_items.len() / 6;

        if self.menu_items.len() % 6 != 0 {
            total_pages += 1;
        }

        total_pages
    }

    fn get_start_index(&self) -> usize {
        if self.current_page == 1 {
            return 0;
        }

        (self.current_page - 1) * 6
    }

    fn get_end_index(&self) -> usize {
        let mut end_index = self.get_start_index() + 6;

        if end_index > self.menu_items.len() {
            end_index = self.menu_items.len();
        }

        end_index
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

            for _ in 0..6 {
                constraints.push(Constraint::Max(3));
            }

            for (index, item) in self.menu_items.iter().enumerate() {
                let color = self.get_item_color(index);
                let title = item.to_string();

                let block = create_block(color.clone());
                let paragraph = Paragraph::new(title)
                    .centered()
                    .block(block)
                    .style(Style::default().fg(color));

                items.push(paragraph);
            }

            for _ in 0..3 {
                constraints.push(Constraint::Min(1));
            }

            let menu_chunks = Layout::default()
                .margin(4)
                .constraints(constraints)
                .split(area);

            let start_index: usize = self.get_start_index().into();
            let end_index: usize = self.get_end_index().into();

            for i in start_index..end_index {
                let paragraph = items[i].clone();
                let mut menu_chunks_index = i;

                if self.current_page > 1 {
                    menu_chunks_index = i - ((self.current_page - 1) * 6);
                }

                frame.render_widget(paragraph, menu_chunks[menu_chunks_index]);
            }

            let page_count_string =
                format!("Page {} of {}", self.current_page, self.get_total_pages());
            let paragraph = Paragraph::new(page_count_string).centered();

            frame.render_widget(paragraph, menu_chunks[7]);
        }
    }

    fn tick(&mut self, _: &mut App) -> AppResult<Option<Message>> {
        Ok(None)
    }

    fn handle_key_press(&mut self, _: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        match key.code {
            KeyCode::Char('j') => {
                if self.current_menu_index >= self.get_end_index() - 1 {
                    if self.current_page + 1 <= self.get_total_pages() {
                        self.current_page = self.current_page + 1;
                    } else {
                        self.current_page = 1;
                    }
                }

                if self.current_menu_index < self.menu_items.len() - 1 {
                    self.current_menu_index = self.current_menu_index + 1;
                } else {
                    self.current_menu_index = 0;
                }
                Ok(None)
            }
            KeyCode::Char('k') => {
                if self.current_menu_index == self.get_start_index() {
                    if self.current_page == 1 {
                        self.current_page = self.get_total_pages();
                    } else {
                        self.current_page = self.current_page - 1;
                    }
                }

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
