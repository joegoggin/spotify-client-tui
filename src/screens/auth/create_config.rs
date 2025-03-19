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
    core::{app::App, config::Config, spotify::client::SpotifyClient},
    layout::rect::get_centered_rect,
    screens::{Screen, ScreenType},
    AppResult, Message,
};

use super::show_link::ShowAuthLinkScreen;

#[derive(Clone)]
pub struct CreateConfigFormScreen {
    form: Form,
}

impl CreateConfigFormScreen {
    pub fn new(config: &Config) -> Self {
        let mut inputs = Vec::<Box<dyn Input>>::new();

        let client_id_placeholder = config.client_id.clone().unwrap_or_default();
        let client_secret_placeholder = config.client_secret.clone().unwrap_or_default();
        let redirect_uri_placeholder = config.redirect_uri.clone().unwrap_or_default();
        let scope_placeholder = config.scope.clone().unwrap_or_default();

        let client_id_input = TextInput::new("Client ID", &client_id_placeholder, false);
        let client_secret_input =
            TextInput::new("Client Secret", &client_secret_placeholder, false);
        let redirect_uri_input = TextInput::new("Redirect URI", &redirect_uri_placeholder, false);
        let scope_input = TextInput::new("Scope", &scope_placeholder, false);

        inputs.push(Box::new(client_id_input));
        inputs.push(Box::new(client_secret_input));
        inputs.push(Box::new(redirect_uri_input));
        inputs.push(Box::new(scope_input));

        let form = Form::new(inputs);

        Self { form }
    }

    fn get_client_id(&self) -> String {
        self.form.inputs[0].get_value().get_text()
    }

    fn get_client_secret(&self) -> String {
        self.form.inputs[1].get_value().get_text()
    }

    fn get_redirect_uri(&self) -> String {
        self.form.inputs[2].get_value().get_text()
    }

    fn get_scope(&self) -> String {
        self.form.inputs[3].get_value().get_text()
    }
}

impl Default for CreateConfigFormScreen {
    fn default() -> Self {
        let mut inputs = Vec::<Box<dyn Input>>::new();
        let client_id_input = TextInput::new("Client ID", "", false);
        let client_secret_input = TextInput::new("Client Secret", "", false);
        let redirect_uri_input = TextInput::new("Redirect URI", "", false);
        let scope_input = TextInput::new("Scope", "", false);

        inputs.push(Box::new(client_id_input));
        inputs.push(Box::new(client_secret_input));
        inputs.push(Box::new(redirect_uri_input));
        inputs.push(Box::new(scope_input));

        let form = Form::new(inputs);

        Self { form }
    }
}

impl Screen for CreateConfigFormScreen {
    fn get_screen_type(&self) -> ScreenType {
        ScreenType::CreateConfigFormScreen
    }
}

impl Component for CreateConfigFormScreen {
    fn view(&mut self, app: &App, frame: &mut Frame) {
        ScreenBlock::new("Create Config").view(app, frame);

        let rect = get_centered_rect(70, 60, frame.area());
        let menu_chunks = Layout::default()
            .margin(5)
            .constraints(vec![
                Constraint::Max(3),
                Constraint::Max(3),
                Constraint::Max(3),
                Constraint::Max(3),
            ])
            .split(rect);

        self.form.inputs[0].set_area(Some(menu_chunks[0]));
        self.form.inputs[1].set_area(Some(menu_chunks[1]));
        self.form.inputs[2].set_area(Some(menu_chunks[2]));
        self.form.inputs[3].set_area(Some(menu_chunks[3]));

        self.form.view(app, frame);
    }

    fn tick(&mut self, app: &mut App) -> AppResult<Option<Message>> {
        self.form.tick(app)?;

        if let Some(spotify_client) = app.spotify_client.clone() {
            let new_screen = Box::new(ShowAuthLinkScreen::new(spotify_client.auth_url));

            return Ok(Some(Message::ChangeScreen { new_screen }));
        }

        Ok(None)
    }

    fn handle_key_press(&mut self, app: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        if let Some(message) = self.form.handle_key_press(app, key)? {
            return Ok(Some(message));
        }

        match key.code {
            KeyCode::Enter => {
                if self.form.mode == FormMode::Normal {
                    let new_config = Config {
                        client_id: Some(self.get_client_id()),
                        client_secret: Some(self.get_client_secret()),
                        redirect_uri: Some(self.get_redirect_uri()),
                        scope: Some(self.get_scope()),
                    };

                    let mut spotify_client = SpotifyClient::new(new_config.clone())?;
                    spotify_client.config.update(new_config.clone())?;
                    app.spotify_client = Some(spotify_client);

                    if let Some(spotify_client) = app.spotify_client.clone() {
                        let new_screen =
                            Box::new(ShowAuthLinkScreen::new(spotify_client.auth_url.clone()));

                        return Ok(Some(Message::ChangeScreen { new_screen }));
                    }
                }

                Ok(None)
            }
            _ => Ok(None),
        }
    }
}
