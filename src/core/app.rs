use clap::Parser;
use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    auth::server::AuthServer,
    screens::{
        auth::create_config::CreateConfigFormScreen, error::ErrorScreen, exit::ExitScreen,
        home::HomeScreen, Screen,
    },
};

use super::{
    clap::Args,
    config::Config,
    logging::setup_logging,
    message::{handler::MessageHandler, Message},
    spotify::client::SpotifyClient,
    tui::{init_terminal, install_panic_hook, restore_terminal},
};

pub type AppResult<T> = color_eyre::Result<T>;

#[derive(Clone)]
pub struct App {
    pub is_running: bool,
    pub history: History,
    pub spotify_client: Option<SpotifyClient>,
    pub default_key_press_enabled: bool,
}

impl App {
    pub fn new() -> AppResult<Self> {
        Ok(Self {
            is_running: true,
            history: History::default(),
            spotify_client: None,
            default_key_press_enabled: true,
        })
    }

    pub async fn run(&mut self) -> AppResult<()> {
        let args = Args::parse();

        install_panic_hook();
        setup_logging()?;

        let config = Config::new()?;
        let mut current_screen: Box<dyn Screen> = Box::new(HomeScreen::default());

        if config.client_id.is_none() || config.redirect_uri.is_none() || config.scope.is_none() {
            current_screen = Box::new(CreateConfigFormScreen::new(&config));
        } else {
            let result = SpotifyClient::new(config);

            match result {
                Ok(spotify_client) => self.spotify_client = Some(spotify_client),
                Err(_) => {
                    current_screen = Box::new(ErrorScreen::new("Failed to create Spotify client."))
                }
            }

            if let Some(command) = args.command.clone() {
                command.handle_command(self, &mut current_screen).await?;

                if command.is_player_command() {
                    return Ok(());
                }
            }
        }

        let mut terminal = init_terminal()?;
        let mut auth_server = AuthServer::default();

        while self.is_running {
            terminal.draw(|frame| current_screen.view(&self, frame))?;

            let mut message_handler =
                MessageHandler::new(self, &mut current_screen, &mut auth_server, &args);

            message_handler.handle_message().await?;
        }

        restore_terminal()?;

        Ok(())
    }
}

impl App {
    pub fn handle_default_key_press(&self, key: KeyEvent) -> AppResult<Option<Message>> {
        if self.default_key_press_enabled {
            match key.code {
                KeyCode::Char('H') => return Ok(Some(Message::GoToPrevScreen)),
                KeyCode::Char('L') => return Ok(Some(Message::GoToNextScreen)),
                KeyCode::Esc => {
                    return Ok(Some(Message::ChangeScreen {
                        new_screen: Box::new(ExitScreen::default()),
                    }))
                }
                _ => {}
            }
        }

        Ok(None)
    }
}

#[derive(Clone)]
pub struct History {
    pub prev: Vec<Box<dyn Screen>>,
    pub next: Vec<Box<dyn Screen>>,
}

impl Default for History {
    fn default() -> Self {
        Self {
            prev: Vec::<Box<dyn Screen>>::new(),
            next: Vec::<Box<dyn Screen>>::new(),
        }
    }
}
