use auth::server::AuthServer;
use clap::Parser;
use core::{
    app::App,
    clap::{Args, Command, PlayerCommand, ViewCommand},
    config::Config,
    logging::setup_logging,
    message::{handler::MessageHandler, Message},
    spotify::{client::SpotifyClient, device::Device, player::SpotifyPlayer},
    tui::{init_terminal, install_panic_hook, restore_terminal},
};
use screens::{
    auth::create_config::CreateConfigFormScreen, devices::DevicesScreen, error::ErrorScreen,
    home::HomeScreen, library::LibraryScreen, now_playing::NowPlayingScreen, queue::QueueScreen,
    search::SearchScreen, view::album::ViewAlbumScreen, view::artist::ViewArtistScreen, Screen,
    ScreenType,
};

mod auth;
mod components;
mod core;
mod layout;
mod screens;
mod utils;
mod widgets;

pub type AppResult<T> = color_eyre::Result<T>;

fn is_player_command(args: &Args) -> bool {
    if let Some(command) = args.command.clone() {
        match command {
            Command::Player { .. } => return true,
            _ => {}
        }
    }
    return false;
}

async fn handle_player_command(args: &Args, app: &mut App) -> AppResult<bool> {
    if let Some(command) = args.command.clone() {
        match command {
            Command::Player { player_command } => {
                if let Some(mut spotify_client) = app.spotify_client.as_mut() {
                    let player = SpotifyPlayer::new();
                    let mut device = Device::default();

                    device.refresh(&mut spotify_client).await?;

                    match player_command {
                        PlayerCommand::PausePlay => {
                            player.toggle_pause_play(&mut spotify_client).await?;
                        }
                        PlayerCommand::NextSong => {
                            player.next_song(&mut spotify_client).await?;
                        }
                        PlayerCommand::PreviousSong => {
                            player.previous_song(&mut spotify_client).await?;
                        }
                        PlayerCommand::Shuffle => {
                            player.toggle_shuffle(&mut spotify_client).await?;
                        }
                        PlayerCommand::Devices => {
                            device.print_devices(&mut spotify_client).await?;
                        }
                        PlayerCommand::SetDevice { id } => {
                            device.set_current_device(&mut spotify_client, id).await?;
                        }
                    }
                }

                return Ok(true);
            }
            _ => {}
        }
    }

    return Ok(false);
}

fn handle_error<T>(result: AppResult<T>) -> Option<Message> {
    match result {
        Ok(_) => None,
        Err(error) => {
            let new_screen = Box::new(ErrorScreen::new(error.to_string()));

            Some(Message::ChangeScreen { new_screen })
        }
    }
}

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

        if is_player_command(&args) {
            handle_player_command(&args, &mut app).await?;
            return Ok(());
        }

        if let Some(command) = args.command.clone() {
            match command {
                Command::NowPlaying => {
                    app.history.prev.push(current_screen.clone_box());
                    current_screen = Box::new(NowPlayingScreen::default());
                }
                Command::View { view_command } => match view_command {
                    ViewCommand::Album => {
                        app.history.prev.push(current_screen.clone_box());
                        current_screen = Box::new(ViewAlbumScreen::default());
                    }
                    ViewCommand::Artist => {
                        app.history.prev.push(current_screen.clone_box());
                        current_screen = Box::new(ViewArtistScreen::default());
                    }
                },
                Command::Queue => {
                    app.history.prev.push(current_screen.clone_box());
                    current_screen = Box::new(QueueScreen::default());
                }
                Command::Search => {
                    app.history.prev.push(current_screen.clone_box());
                    current_screen = Box::new(SearchScreen::default());
                }
                Command::Library => {
                    app.history.prev.push(current_screen.clone());
                    current_screen = Box::new(LibraryScreen::default());
                }
                Command::Devices => {
                    app.history.prev.push(current_screen.clone());
                    current_screen = Box::new(DevicesScreen::default());
                }
                _ => {}
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
