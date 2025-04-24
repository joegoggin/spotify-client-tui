use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::Frame;

use crate::components::loading::Loading;
use crate::core::message::Message;
use crate::core::spotify::NameAndId;
use crate::{App, AppResult};

use super::Component;

#[derive(Clone, Debug)]
pub struct List {
    pub items: Vec<NameAndId>,
    pub current_item_id: Option<String>,
    pub active_index: usize,
    area: Rect,
    max_items: u16,
    start_index: usize,
    end_index: usize,
    items_changed: bool,
}

impl Default for List {
    fn default() -> Self {
        Self {
            items: vec![],
            current_item_id: None,
            area: Rect::default(),
            max_items: 0,
            active_index: 0,
            start_index: 0,
            end_index: 0,
            items_changed: false,
        }
    }
}

impl List {
    pub fn new(items: Vec<NameAndId>, current_item_id: Option<String>) -> Self {
        Self {
            items,
            current_item_id,
            area: Rect::default(),
            max_items: 0,
            active_index: 0,
            start_index: 0,
            end_index: 0,
            items_changed: true,
        }
    }

    fn get_item_style(&self, index: usize) -> Style {
        let mut style = Style::default().fg(Color::Green);

        if self.active_index == index {
            style = Style::default().fg(Color::White).bg(Color::Green);
        }

        style
    }

    pub fn set_items(&mut self, items: Vec<NameAndId>) {
        self.items = items;
        self.items_changed = true;
    }

    pub fn get_active_item(&self) -> NameAndId {
        match self.items.get(self.active_index) {
            Some(item) => item.clone(),
            None => (String::new(), String::new()),
        }
    }
}

impl Component for List {
    fn view(&mut self, app: &App, frame: &mut Frame) {
        if self.items.is_empty() {
            let mut loading = Loading::default();

            loading.set_area(&self.area);
            loading.view(app, frame);
            return;
        }

        let mut constraits: Vec<Constraint> = vec![];
        let mut paragraphs: Vec<Paragraph> = vec![];

        for _ in 0..self.max_items {
            constraits.push(Constraint::Max(1));
        }

        for i in self.start_index..self.end_index {
            if i < self.items.len() {
                let item = &self.items[i];
                let mut name = item.0.clone();

                if let Some(current_item_id) = self.current_item_id.clone() {
                    if item.1 == current_item_id {
                        name = format!("* {} *", name);
                    }
                }

                let paragraph = Paragraph::new(name)
                    .left_aligned()
                    .style(self.get_item_style(i))
                    .wrap(Wrap { trim: false });

                paragraphs.push(paragraph);
            }
        }

        let chunks = Layout::default()
            .margin(1)
            .direction(Direction::Vertical)
            .constraints(constraits)
            .split(self.area);

        for i in 0..self.items.len() {
            if i < paragraphs.len() {
                frame.render_widget(paragraphs[i].clone(), chunks[i]);
            }
        }
    }

    fn tick(&mut self, _: &mut App) -> AppResult<Option<Message>> {
        Ok(None)
    }

    fn handle_key_press(&mut self, _: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        match key.code {
            KeyCode::Char('j') => {
                if self.active_index >= self.end_index - 1 {
                    self.start_index = self.start_index + 1;
                    self.end_index = self.end_index + 1;
                }
                if self.active_index < self.items.len() - 1 {
                    self.active_index = self.active_index + 1;
                } else {
                    self.active_index = 0;
                    self.start_index = 0;
                    self.end_index = self.max_items.into();
                }

                Ok(None)
            }
            KeyCode::Char('k') => {
                if self.active_index <= self.start_index && self.active_index != 0 {
                    self.start_index = self.start_index - 1;
                    self.end_index = self.end_index - 1;
                }

                if self.active_index == 0 {
                    self.active_index = self.items.len() - 1;
                    self.end_index = self.items.len();

                    if self.items.len() < self.max_items as usize {
                        self.start_index = 0;
                    } else {
                        self.start_index = (self.items.len() - self.max_items as usize) as usize;
                    }
                } else {
                    self.active_index = self.active_index - 1;
                }

                Ok(None)
            }
            _ => Ok(None),
        }
    }

    fn set_area(&mut self, area: Rect) {
        let max_items = area.height - 2;

        self.area = area;
        self.max_items = max_items;

        if self.items_changed {
            self.end_index = max_items.into();
            self.items_changed = false;
        }
    }

    fn get_area(&mut self) -> Rect {
        self.area
    }
}
