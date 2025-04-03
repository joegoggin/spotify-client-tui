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
        spotify::client::SpotifyClient,
    },
    utils::vec::ToStringVec,
};

use super::{
    auth::{create_config::CreateConfigFormScreen, show_link::ShowAuthLinkScreen},
    devices::DevicesScreen,
    error::ErrorScreen,
    library::LibraryScreen,
    now_playing::NowPlayingScreen,
    queue::QueueScreen,
    search::SearchScreen,
    view::album::ViewAlbumScreen,
    view::artist::ViewArtistScreen,
    Screen, ScreenType,
};

#[derive(Clone)]
pub struct HomeScreen {
    menu: Menu,
}

impl Default for HomeScreen {
    fn default() -> Self {
        let menu_items = vec![
            "Now Playing",
            "View Artist",
            "View Album",
            "Queue",
            "Search",
            "Library",
            "Devices",
        ];

        Self {
            menu: Menu::new(menu_items.to_string_vec()),
        }
    }
}

impl Screen for HomeScreen {
    fn get_screen_type(&self) -> ScreenType {
        ScreenType::Home
    }
}

impl Component for HomeScreen {
    fn view(&mut self, app: &App, frame: &mut Frame) {
        ScreenBlock::new_with_color("Spotify Client TUI", Color::Green).view(app, frame);
        self.menu.view(app, frame);
    }

    fn tick(&mut self, app: &mut App) -> AppResult<Option<Message>> {
        match app.spotify_client.clone() {
            Some(spotify_client) => {
                if spotify_client.credentials.is_none() {
                    let new_screen = Box::new(ShowAuthLinkScreen::new(spotify_client.auth_url));

                    return Ok(Some(Message::ChangeScreen { new_screen }));
                }

                Ok(None)
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
    }

    fn handle_key_press(&mut self, app: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        if let Some(message) = self.menu.handle_key_press(app, key)? {
            return Ok(Some(message));
        }

        match key.code {
            KeyCode::Enter => match self.menu.get_current_item().as_str() {
                "Now Playing" => Ok(Some(Message::ChangeScreen {
                    new_screen: Box::new(NowPlayingScreen::default()),
                })),
                "View Artist" => Ok(Some(Message::ChangeScreen {
                    new_screen: Box::new(ViewArtistScreen::default()),
                })),
                "View Album" => Ok(Some(Message::ChangeScreen {
                    new_screen: Box::new(ViewAlbumScreen::default()),
                })),
                "Queue" => Ok(Some(Message::ChangeScreen {
                    new_screen: Box::new(QueueScreen::default()),
                })),
                "Search" => Ok(Some(Message::ChangeScreen {
                    new_screen: Box::new(SearchScreen::default()),
                })),
                "Library" => Ok(Some(Message::ChangeScreen {
                    new_screen: Box::new(LibraryScreen::default()),
                })),
                "Devices" => Ok(Some(Message::ChangeScreen {
                    new_screen: Box::new(DevicesScreen::default()),
                })),
                _ => Ok(None),
            },
            KeyCode::Char('e') => Ok(Some(Message::ChangeScreen {
                new_screen: Box::new(ErrorScreen::default()),
            })),
            _ => Ok(None),
        }
    }
}
