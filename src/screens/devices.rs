use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    style::Color,
    Frame,
};

use crate::{
    components::{menu::Menu, screen_block::ScreenBlock, Component},
    core::{
        app::{App, AppResult},
        config::Config,
        message::Message,
        spotify::{client::SpotifyClient, device::Device},
    },
    utils::vec::ToStringVec,
};

use super::{
    auth::{create_config::CreateConfigFormScreen, show_link::ShowAuthLinkScreen},
    Screen, ScreenType,
};

#[derive(Clone)]
pub struct DevicesScreen {
    pub device: Device,
    pub menu: Menu,
    pub menu_initalized: bool,
}

impl Default for DevicesScreen {
    fn default() -> Self {
        Self {
            device: Device::default(),
            menu: Menu::new(vec![].to_string_vec()),
            menu_initalized: false,
        }
    }
}

impl Screen for DevicesScreen {
    fn get_screen_type(&self) -> ScreenType {
        ScreenType::DevicesScreen
    }

    fn get_device(&mut self) -> Option<&mut Device> {
        Some(&mut self.device)
    }
}

impl Component for DevicesScreen {
    fn view(&mut self, app: &App, frame: &mut Frame) {
        ScreenBlock::new_with_color("Devices", Color::Green).view(app, frame);

        self.menu.view(app, frame);
    }

    fn tick(&mut self, app: &mut App) -> AppResult<Option<Message>> {
        match app.spotify_client.clone() {
            Some(spotify_client) => {
                if spotify_client.credentials.is_none() {
                    let new_screen = Box::new(ShowAuthLinkScreen::new(spotify_client.auth_url));

                    return Ok(Some(Message::ChangeScreen { new_screen }));
                }
            }
            None => {
                let config = Config::new()?;
                let result = SpotifyClient::new(config.clone());

                match result {
                    Ok(spotify_client) => {
                        app.spotify_client = Some(spotify_client);

                        return Ok(None);
                    }
                    Err(_) => {
                        let new_screen = Box::new(CreateConfigFormScreen::new(&config));

                        return Ok(Some(Message::ChangeScreen { new_screen }));
                    }
                }
            }
        }

        self.menu.tick(app)?;
        self.menu.menu_items = self.device.get_available_devices_names();

        if !self.menu_initalized {
            self.menu_initalized = true;

            return Ok(Some(Message::RefreshDevice));
        }

        Ok(None)
    }

    fn handle_key_press(&mut self, app: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        if let Some(message) = self.menu.handle_key_press(app, key)? {
            return Ok(Some(message));
        }

        match key.code {
            KeyCode::Char('r') => Ok(Some(Message::RefreshDevice)),
            KeyCode::Enter => {
                let name = self.menu.get_current_item();

                if let Some(current_device_name) = &self.device.current_device_name {
                    if name == format!("* {} *", current_device_name) {
                        return Ok(None);
                    }
                }

                let id = self.device.available_devices[&name].clone();

                Ok(Some(Message::SetDevice { id, name }))
            }
            _ => Ok(None),
        }
    }
}
