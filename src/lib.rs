use auth::server::AuthServer;
use clap::Parser;
use core::{
    app::App,
    clap::Args,
    config::Config,
    logging::setup_logging,
    message::{handler::MessageHandler, Message},
    spotify::client::SpotifyClient,
    tui::{init_terminal, install_panic_hook, restore_terminal},
};
use screens::{
    auth::create_config::CreateConfigFormScreen, error::ErrorScreen, home::HomeScreen, Screen,
};

mod auth;
mod components;
mod core;
mod layout;
mod screens;
mod utils;
mod widgets;

pub type AppResult<T> = color_eyre::Result<T>;

pub async fn run() -> AppResult<()> {
    let args = Args::parse();

    install_panic_hook();

    setup_logging()?;

    let mut app = App::new()?;
    let config = Config::new()?;

    let mut current_screen: Box<dyn Screen> = Box::new(HomeScreen::default());

    if config.client_id.is_none() || config.redirect_uri.is_none() || config.scope.is_none() {
        current_screen = Box::new(CreateConfigFormScreen::new(&config));
    } else {
        let result = SpotifyClient::new(config);

        match result {
            Ok(spotify_client) => app.spotify_client = Some(spotify_client),
            Err(_) => {
                current_screen = Box::new(ErrorScreen::new("Failed to create Spotify client."))
            }
        }

        if let Some(command) = args.command.clone() {
            command
                .handle_command(&mut app, &mut current_screen)
                .await?;

            if command.is_player_command() {
                return Ok(());
            }
        }
    }

    let mut terminal = init_terminal()?;
    let mut auth_server = AuthServer::default();

    while app.is_running {
        terminal.draw(|frame| current_screen.view(&app, frame))?;

        let mut message_handler =
            MessageHandler::new(&mut app, &mut current_screen, &mut auth_server, &args);

        message_handler.handle_message().await?;
    }

    restore_terminal()?;

    Ok(())
}
