use auth::server::AuthServer;
use clap::Parser;
use core::{
    app::App,
    clap::{Args, Command, PlayerCommand},
    config::Config,
    logging::setup_logging,
    spotify::{client::SpotifyClient, player::SpotifyPlayer},
    tui::{init_terminal, install_panic_hook, restore_terminal},
};
use screens::{
    auth::create_config::CreateConfigFormScreen, error::ErrorScreen, home::HomeScreen, Screen,
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

#[derive(Clone)]
pub enum Message {
    Quit,
    ChangeScreen { new_screen: Box<dyn Screen> },
    GoToPrevScreen,
    GoToNextScreen,
    ListenForAuthCode,
    RefreshNowPlaying,
    SetAuthCode { code: String },
    PausePlay,
    Shuffle,
    NextSong,
    PrevSong,
}

fn is_player_command(args: &Args) -> bool {
    if let Some(command) = args.command.clone() {
        match command {
            Command::Player { .. } => return true,
            _ => {}
        }
    }
    return false;
}

async fn handle_control_command(args: &Args, app: &mut App) -> AppResult<bool> {
    if let Some(command) = args.command.clone() {
        match command {
            Command::Player { player_command } => {
                if let Some(mut spotify_client) = app.spotify_client.as_mut() {
                    let mut player = SpotifyPlayer::new();

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
                            player.list_devices(&mut spotify_client).await?;
                        }
                        PlayerCommand::Device { id } => {
                            player.set_device(&mut spotify_client, id).await?;
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
            handle_control_command(&args, &mut app).await?;
            return Ok(());
        }
    }

    let mut terminal = init_terminal()?;
    let mut auth_server = AuthServer::default();

    while app.is_running {
        let mut current_message = current_screen.tick(&mut app)?;
        terminal.draw(|frame| current_screen.view(&app, frame))?;

        if current_message.is_none() {
            current_message = current_screen.handle_event(&mut app)?
        }

        while current_message.is_some() {
            match current_message.clone().unwrap() {
                Message::ChangeScreen { new_screen } => {
                    app.history.prev.push(current_screen);

                    if let Some(spotify_client) = &app.spotify_client {
                        if new_screen.get_screen_type() == ScreenType::ShowAuthLinkScreen {
                            auth_server.start(&spotify_client.config)?;
                        }
                    }

                    if new_screen.get_screen_type() == ScreenType::Home && is_player_command(&args)
                    {
                        app.is_running = false;
                        handle_control_command(&args, &mut app).await?;
                    }

                    current_screen = new_screen;
                    break;
                }
                Message::GoToPrevScreen => {
                    if let Some(last_screen) = app.history.prev.pop() {
                        if current_screen.get_screen_type() != ScreenType::Exit {
                            app.history.next.push(current_screen.clone_box());
                        }

                        current_screen = last_screen;
                    }
                }
                Message::GoToNextScreen => {
                    if let Some(next_screen) = app.history.next.pop() {
                        if current_screen.get_screen_type() != ScreenType::Exit {
                            app.history.prev.push(current_screen.clone_box())
                        }

                        current_screen = next_screen;
                    }
                }
                Message::SetAuthCode { code } => {
                    if let Some(mut spotify_client) = app.spotify_client.clone() {
                        let result = spotify_client.set_code_and_access_token(code).await;

                        if let Some(message) = handle_error(result) {
                            current_message = Some(message);
                            continue;
                        }

                        if spotify_client.credentials.is_some() {
                            app.spotify_client = Some(spotify_client);

                            auth_server.stop()?;

                            let new_screen = Box::new(HomeScreen::default());

                            current_message = Some(Message::ChangeScreen { new_screen });
                            continue;
                        }
                    }
                }
                Message::RefreshNowPlaying => {
                    if let Some(mut spotify_client) = app.spotify_client.as_mut() {
                        if let Some(now_playing) = current_screen.get_now_playing() {
                            let result = now_playing.refresh(&mut spotify_client).await;

                            if let Some(message) = handle_error(result) {
                                current_message = Some(message);
                                continue;
                            }
                        }
                    }
                }
                Message::PausePlay => {
                    if let Some(mut spotify_client) = app.spotify_client.as_mut() {
                        let player = SpotifyPlayer::new();

                        let result = player.toggle_pause_play(&mut spotify_client).await;

                        if let Some(message) = handle_error(result) {
                            current_message = Some(message);
                            continue;
                        }
                    }
                }
                Message::Shuffle => {
                    if let Some(mut spotify_client) = app.spotify_client.as_mut() {
                        let player = SpotifyPlayer::new();

                        let result = player.toggle_shuffle(&mut spotify_client).await;

                        if let Some(message) = handle_error(result) {
                            current_message = Some(message);
                            continue;
                        }
                    }
                }
                Message::NextSong => {
                    if let Some(mut spotify_client) = app.spotify_client.as_mut() {
                        let player = SpotifyPlayer::new();

                        let result = player.next_song(&mut spotify_client).await;

                        if let Some(message) = handle_error(result) {
                            current_message = Some(message);
                            continue;
                        }
                    }
                }
                Message::PrevSong => {
                    if let Some(mut spotify_client) = app.spotify_client.as_mut() {
                        let player = SpotifyPlayer::new();

                        let result = player.previous_song(&mut spotify_client).await;

                        if let Some(message) = handle_error(result) {
                            current_message = Some(message);
                            continue;
                        }
                    }
                }
                _ => {}
            }

            current_message = current_screen.handle_event(&mut app)?
        }
    }

    restore_terminal()?;

    Ok(())
}
