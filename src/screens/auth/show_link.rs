use arboard::Clipboard;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout},
    style::Color,
};

use crate::{
    components::{menu::Menu, screen_block::ScreenBlock, Component},
    core::{app::App, message::Message},
    layout::rect::get_centered_rect,
    screens::{Screen, ScreenType},
    utils::vec::ToStringVec,
    widgets::paragraph::{create_centered_paragraph, create_paragraph},
    AppResult,
};

use super::enter_code::EnterAuthCodeScreen;

#[derive(Clone)]
pub struct ShowAuthLinkScreen {
    auth_url: String,
    menu: Menu,
    clipboard_is_copied: bool,
}

impl ShowAuthLinkScreen {
    pub fn new(auth_url: String) -> Self {
        let options = vec!["Copy To Clipboard", "Open In Browser", "Enter Code"].to_string_vec();

        Self {
            auth_url,
            menu: Menu::new(options),
            clipboard_is_copied: false,
        }
    }
}

impl Screen for ShowAuthLinkScreen {
    fn get_screen_type(&self) -> ScreenType {
        ScreenType::ShowAuthLinkScreen
    }
}

impl Component for ShowAuthLinkScreen {
    fn view(&mut self, app: &App, frame: &mut ratatui::Frame) {
        ScreenBlock::new("Log In").view(app, frame);

        let message_area = get_centered_rect(80, 80, frame.area());

        let message_chunks = Layout::default()
            .margin(2)
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(5), Constraint::Max(5), Constraint::Min(10)])
            .split(message_area);
        let text = "In order to use the app you will need to log into your Spotify account.\n
            You can log in by navigating to the following link in your browser:\n
            ";
        let paragraph = create_centered_paragraph(&text, Some(Color::White));
        let mut link = create_paragraph(&self.auth_url, Some(Color::Blue));

        if self.clipboard_is_copied {
            link = create_centered_paragraph("Link Copied to Clipboard!", Some(Color::Green));
        }

        frame.render_widget(paragraph, message_chunks[0]);
        frame.render_widget(link, message_chunks[1]);

        self.menu.area = Some(message_chunks[2]);
        self.menu.view(app, frame);
    }

    fn tick(&mut self, _: &mut App) -> AppResult<Option<Message>> {
        Ok(None)
    }

    fn handle_key_press(&mut self, app: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        if let Some(message) = self.menu.handle_key_press(app, key)? {
            return Ok(Some(message));
        }

        match key.code {
            KeyCode::Enter => match self.menu.get_current_item().as_str() {
                "Copy To Clipboard" => {
                    let mut clipboard = Clipboard::new()?;
                    clipboard.set_text(self.auth_url.clone())?;
                    self.clipboard_is_copied = true;

                    Ok(None)
                }
                "Open In Browser" => {
                    open::that(self.auth_url.clone())?;

                    Ok(None)
                }
                "Enter Code" => {
                    let new_screen = Box::new(EnterAuthCodeScreen::default());

                    Ok(Some(Message::ChangeScreen { new_screen }))
                }
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }
}
