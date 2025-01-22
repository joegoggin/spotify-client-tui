use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout},
    Frame,
};

use crate::{
    components::{
        form::{
            form::{Form, Input},
            text_input::TextInput,
        },
        screen_block::ScreenBlock,
        Component,
    },
    core::app::App,
    layout::rect::get_centered_rect,
    screens::{Screen, ScreenType},
    AppResult, Message,
};

#[derive(Clone)]
pub struct EnterAuthCodeScreen {
    form: Form,
}

impl EnterAuthCodeScreen {
    fn get_code(&self) -> String {
        self.form.inputs[0].get_value().get_text()
    }
}

impl Default for EnterAuthCodeScreen {
    fn default() -> Self {
        let mut inputs = Vec::<Box<dyn Input>>::new();
        let code_input = TextInput::new("Code", "", false);
        inputs.push(Box::new(code_input));

        let form = Form::new(inputs);

        Self { form }
    }
}

impl Screen for EnterAuthCodeScreen {
    fn get_screen_type(&self) -> ScreenType {
        ScreenType::EnterAuthCodeScreen
    }
}

impl Component for EnterAuthCodeScreen {
    fn view(&mut self, app: &App, frame: &mut Frame) {
        ScreenBlock::new("Enter Code").view(app, frame);

        let rect = get_centered_rect(70, 50, frame.area());
        let menu_chunks = Layout::default()
            .margin(5)
            .constraints(vec![Constraint::Max(3)])
            .split(rect);

        self.form.inputs[0].set_area(Some(menu_chunks[0]));

        self.form.view(app, frame);
    }

    fn tick(&mut self, app: &mut App) -> AppResult<Option<Message>> {
        self.form.tick(app)?;

        Ok(None)
    }

    fn handle_key_press(&mut self, app: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        if let Some(message) = self.form.handle_key_press(app, key)? {
            return Ok(Some(message));
        }

        match key.code {
            KeyCode::Enter => {
                return Ok(Some(Message::SetAuthCode {
                    code: self.get_code(),
                }))
            }
            _ => {}
        }

        Ok(None)
    }
}
