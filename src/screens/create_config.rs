use log::debug;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout},
    Frame,
};

use crate::{
    components::{
        form::{
            form::{Form, FormMode, Input},
            text_input::TextInput,
        },
        screen_block::ScreenBlock,
        Component,
    },
    core::{app::App, config::Config},
    layout::rect::get_centered_rect,
    screens::{Screen, ScreenType},
    AppResult, Message,
};

use super::home::HomeScreen;

#[derive(Clone)]
pub struct CreateConfigFormScreen {
    form: Form,
}

impl CreateConfigFormScreen {
    pub fn new(config: &Config) -> Self {
        let mut inputs = Vec::<Box<dyn Input>>::new();
        let client_id = config.client_id.clone().unwrap_or("".to_string());
        let redirect_uri = config.redirect_uri.clone().unwrap_or("".to_string());
        let client_id_input = TextInput::new("Client ID", &client_id, false);
        let redirect_uri = TextInput::new("Redirect URI", &redirect_uri, false);

        inputs.push(Box::new(client_id_input));
        inputs.push(Box::new(redirect_uri));

        let form = Form::new(inputs);

        Self { form }
    }

    fn get_client_id(&self) -> String {
        self.form.inputs[0].get_value().get_text()
    }

    fn get_redirect_uri(&self) -> String {
        self.form.inputs[1].get_value().get_text()
    }
}

impl Default for CreateConfigFormScreen {
    fn default() -> Self {
        let mut inputs = Vec::<Box<dyn Input>>::new();
        let client_id_input = TextInput::new("Client ID", "", false);
        let redirect_uri = TextInput::new("Redirect URI", "", false);

        inputs.push(Box::new(client_id_input));
        inputs.push(Box::new(redirect_uri));

        let form = Form::new(inputs);

        Self { form }
    }
}

impl Screen for CreateConfigFormScreen {
    fn get_screen_type(&self) -> ScreenType {
        ScreenType::ClientIdFormScreen
    }
}

impl Component for CreateConfigFormScreen {
    fn view(&mut self, frame: &mut Frame) {
        ScreenBlock::new("Create Config").view(frame);

        let rect = get_centered_rect(70, 50, frame.area());
        let menu_chunks = Layout::default()
            .margin(5)
            .constraints(vec![
                Constraint::Max(3),
                Constraint::Max(3),
                Constraint::Max(1),
            ])
            .split(rect);

        self.form.inputs[0].set_area(Some(menu_chunks[0]));
        self.form.inputs[1].set_area(Some(menu_chunks[1]));

        self.form.view(frame);
    }

    fn tick(&mut self) {
        self.form.tick();
    }

    fn handle_key_press(&mut self, app: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        if let Some(message) = self.form.handle_key_press(app, key)? {
            return Ok(Some(message));
        }

        match key.code {
            KeyCode::Enter => {
                if self.form.mode == FormMode::Normal {
                    app.config
                        .update(self.get_client_id(), self.get_redirect_uri())?;

                    return Ok(Some(Message::ChangeScreen {
                        new_screen: Box::new(HomeScreen::default()),
                    }));
                }

                Ok(None)
            }
            _ => Ok(None),
        }
    }
}
