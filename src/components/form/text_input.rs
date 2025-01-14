use std::time::{Duration, Instant};

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Alignment, Rect},
    Frame,
};

use crate::{
    components::Component,
    core::app::App,
    widgets::{block::create_titled_block, paragraph::create_paragraph_with_block},
    AppResult, Message,
};

use super::form::{FormMode, Input, InputValue};

#[derive(Clone)]
pub struct TextInput {
    pub title: String,
    pub value: String,
    pub mode: FormMode,
    pub is_focused: bool,
    pub is_password: bool,
    pub area: Option<Rect>,
    pub scroll_offset: u16,
    pub cursor_position: usize,
    pub cursor_visible: bool,
    pub last_cursor_blink: Instant,
}

impl TextInput {
    pub fn new(title: &str, placeholder: &str, is_password: bool) -> Self {
        Self {
            title: title.into(),
            value: placeholder.into(),
            mode: FormMode::Normal,
            is_focused: false,
            is_password,
            area: None,
            scroll_offset: 0,
            cursor_position: 0,
            cursor_visible: true,
            last_cursor_blink: Instant::now(),
        }
    }

    pub fn update_cursor_blink(&mut self) {
        let blink_interval = Duration::from_millis(500);

        if self.last_cursor_blink.elapsed() >= blink_interval {
            self.cursor_visible = !self.cursor_visible;
            self.last_cursor_blink = Instant::now();
        }
    }
}

impl Input for TextInput {
    fn get_is_focused(&self) -> bool {
        self.is_focused
    }

    fn set_is_focused(&mut self, focus: bool) {
        self.is_focused = focus;
    }

    fn get_mode(&self) -> FormMode {
        self.mode.clone()
    }

    fn set_mode(&mut self, mode: FormMode) {
        self.mode = mode;
    }

    fn set_area(&mut self, area: Option<Rect>) {
        self.area = area;
    }

    fn get_value(&self) -> InputValue {
        InputValue::Text(self.value.clone())
    }
}

impl Component for TextInput {
    fn view(&mut self, frame: &mut Frame) {
        let text = match self.is_password {
            true => {
                let mut string = String::new();

                for _ in 0..self.value.len() {
                    string.push('*');
                }

                string
            }
            false => self.value.clone(),
        };

        if let Some(area) = self.area {
            let available_width = area.width as usize;
            let mut visual_text = text.clone();

            let start = self.scroll_offset as usize;
            let end = (start + available_width).min(text.len());
            visual_text = visual_text[start..end].to_string();

            if self.mode == FormMode::Insert && self.is_focused {
                let cursor_char = match self.cursor_visible {
                    true => '|',
                    false => ' ',
                };

                let cursor_in_view = self.cursor_position.saturating_sub(start);
                if cursor_in_view < visual_text.len() {
                    visual_text.insert(cursor_in_view, cursor_char);
                } else {
                    visual_text.push(cursor_char);
                }
            }

            let block = create_titled_block(&self.title, Alignment::Left, self.get_color());
            let paragraph = create_paragraph_with_block(&visual_text, block, self.get_color());

            frame.render_widget(paragraph, area);
        }
    }

    fn handle_key_press(&mut self, _app: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        match key.code {
            KeyCode::Char(c) => {
                if self.mode == FormMode::Insert && self.is_focused {
                    if self.cursor_position == self.value.len() {
                        self.value.push(c);
                    } else {
                        self.value.insert(self.cursor_position, c);
                    }

                    if let Some(area) = self.area {
                        let available_width = area.width as usize;

                        if self.value.len() > available_width - 3 {
                            self.scroll_offset = self.scroll_offset + 1;
                        }

                        self.cursor_position = self.cursor_position + 1;
                    }
                }

                Ok(None)
            }
            KeyCode::Backspace => {
                if self.is_focused {
                    if self.cursor_position == self.value.len() {
                        self.value.pop();
                    } else {
                        self.value.remove(self.cursor_position.saturating_sub(1));
                    }

                    if let Some(area) = self.area {
                        if self.value.len() >= area.width as usize - 3 {
                            self.scroll_offset = self.scroll_offset.saturating_sub(1);
                        }

                        self.cursor_position = self.cursor_position.saturating_sub(1);
                    }
                }

                Ok(None)
            }
            KeyCode::Left => {
                if self.mode == FormMode::Insert && self.is_focused {
                    self.cursor_position = self.cursor_position.saturating_sub(1);
                }

                Ok(None)
            }
            KeyCode::Right => {
                if self.mode == FormMode::Insert
                    && self.is_focused
                    && self.value.len() - 1 >= self.cursor_position
                {
                    self.cursor_position = self.cursor_position + 1;
                }

                Ok(None)
            }
            KeyCode::Down => {
                if self.mode == FormMode::Insert && self.is_focused {
                    self.cursor_position = self.value.len();
                }

                Ok(None)
            }
            KeyCode::Up => {
                if self.mode == FormMode::Insert && self.is_focused {
                    self.cursor_position = 0;
                }

                Ok(None)
            }
            _ => Ok(None),
        }
    }

    fn tick(&mut self) {
        self.update_cursor_blink()
    }
}
