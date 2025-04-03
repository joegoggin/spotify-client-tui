use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    style::Color,
    Frame,
};

use crate::{
    components::Component,
    core::{
        app::{App, AppResult},
        message::Message,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub enum FormMode {
    Insert,
    Normal,
}

pub enum InputValue {
    Text(String),
    Boolean(bool),
}

impl InputValue {
    pub fn get_text(&self) -> String {
        match self {
            InputValue::Text(value) => value.to_string(),
            _ => "".to_string(),
        }
    }

    pub fn get_boolean(&self, default: bool) -> bool {
        match self {
            InputValue::Boolean(value) => value.to_owned(),
            _ => default,
        }
    }
}

pub trait Input: InputClone + Component {
    fn get_color(&self) -> Color {
        if !self.get_is_focused() {
            return Color::White;
        }

        match self.get_mode().clone() {
            FormMode::Insert => Color::Yellow,
            FormMode::Normal => Color::Blue,
        }
    }

    fn get_is_focused(&self) -> bool;
    fn set_is_focused(&mut self, focus: bool);
    fn get_mode(&self) -> FormMode;
    fn set_mode(&mut self, mode: FormMode);
    fn set_area(&mut self, area: Option<Rect>);
    fn get_value(&self) -> InputValue;
}

pub trait InputClone {
    fn clone_box(&self) -> Box<dyn Input>;
}

impl<T> InputClone for T
where
    T: 'static + Input + Clone,
{
    fn clone_box(&self) -> Box<dyn Input> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Input> {
    fn clone(&self) -> Box<dyn Input> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct Form {
    pub inputs: Vec<Box<dyn Input>>,
    pub focused_input_index: usize,
    pub mode: FormMode,
}

impl Form {
    pub fn new(inputs: Vec<Box<dyn Input>>) -> Self {
        let mut inputs = inputs.clone();
        inputs[0].set_is_focused(true);

        Self {
            inputs,
            focused_input_index: 0,
            mode: FormMode::Normal,
        }
    }
}

impl Component for Form {
    fn view(&mut self, app: &App, frame: &mut Frame) {
        for i in 0..self.inputs.len() {
            self.inputs[i].view(app, frame)
        }
    }

    fn tick(&mut self, app: &mut App) -> AppResult<Option<Message>> {
        for i in 0..self.inputs.len() {
            self.inputs[i].tick(app)?;
        }

        if self.mode == FormMode::Insert && app.default_key_press_enabled {
            app.default_key_press_enabled = false;
        }

        if self.mode == FormMode::Normal && !app.default_key_press_enabled {
            app.default_key_press_enabled = true;
        }

        Ok(None)
    }

    fn handle_key_press(&mut self, app: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        for i in 0..self.inputs.len() {
            if let Some(message) = self.inputs[i].handle_key_press(app, key)? {
                return Ok(Some(message));
            }
        }

        match key.code {
            KeyCode::Char('j') => {
                if self.mode == FormMode::Normal {
                    self.inputs[self.focused_input_index].set_is_focused(false);

                    if self.focused_input_index < self.inputs.len() - 1 {
                        self.focused_input_index = self.focused_input_index + 1;
                    } else {
                        self.focused_input_index = 0;
                    }

                    self.inputs[self.focused_input_index].set_is_focused(true);
                }

                Ok(None)
            }
            KeyCode::Char('k') => {
                if self.mode == FormMode::Normal {
                    self.inputs[self.focused_input_index].set_is_focused(false);

                    if self.focused_input_index > 0 {
                        self.focused_input_index = self.focused_input_index - 1;
                    } else {
                        self.focused_input_index = self.inputs.len() - 1;
                    }

                    self.inputs[self.focused_input_index].set_is_focused(true);
                }

                Ok(None)
            }
            KeyCode::Char('i') => {
                self.mode = FormMode::Insert;

                for i in 0..self.inputs.len() {
                    self.inputs[i].set_mode(FormMode::Insert);
                }

                Ok(None)
            }
            KeyCode::Esc => {
                self.mode = FormMode::Normal;

                for i in 0..self.inputs.len() {
                    self.inputs[i].set_mode(FormMode::Normal);
                }

                Ok(None)
            }
            KeyCode::Tab => {
                if self.focused_input_index < self.inputs.len() - 1 && self.mode == FormMode::Insert
                {
                    self.inputs[self.focused_input_index].set_is_focused(false);
                    self.focused_input_index = self.focused_input_index + 1;
                    self.inputs[self.focused_input_index].set_is_focused(true);
                }

                Ok(None)
            }
            KeyCode::BackTab => {
                if self.focused_input_index > 0 && self.mode == FormMode::Insert {
                    self.inputs[self.focused_input_index].set_is_focused(false);
                    self.focused_input_index = self.focused_input_index - 1;
                    self.inputs[self.focused_input_index].set_is_focused(true);
                }

                Ok(None)
            }
            _ => Ok(None),
        }
    }
}
