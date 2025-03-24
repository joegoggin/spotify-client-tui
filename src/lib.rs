use auth::server::AuthServer;
use clap::Parser;
use core::{
    app::App,
    clap::{Args, Command, PlayerCommand},
    config::Config,
    logging::setup_logging,
    spotify::{client::SpotifyClient, now_playing::NowPlaying, player::SpotifyPlayer},
    tui::{init_terminal, install_panic_hook, restore_terminal},
};
use screens::{auth::create_config::CreateConfigFormScreen, home::HomeScreen, Screen, ScreenType};

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

async fn handle_control_command(args: &Args, app: &App) -> AppResult<bool> {
    if let Some(command) = args.command.clone() {
        match command {
            Command::Player { player_command } => {
                if let Some(spotify_client) = app.spotify_client.clone() {
                    let mut player = SpotifyPlayer::new(spotify_client.clone());

                    match player_command {
                        PlayerCommand::PausePlay => {
                            player.toggle_pause_play().await?;
                        }
                        PlayerCommand::NextSong => {
                            player.next_song().await?;
                        }
                        PlayerCommand::PreviousSong => {
                            player.previous_song().await?;
                        }
                        PlayerCommand::Shuffle => {
                            player.toggle_shuffle().await?;
                        }
                        PlayerCommand::Devices => {
                            player.list_devices().await?;
                        }
                        PlayerCommand::Device { id } => {
                            player.set_device(id).await?;
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
        app.spotify_client = Some(SpotifyClient::new(config)?);

        if is_player_command(&args) {
            handle_control_command(&args, &app).await?;
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
                        handle_control_command(&args, &app).await?;
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
                        spotify_client.set_code_and_access_token(code).await?;

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
                    if let Some(mut spotify_client) = app.spotify_client.clone() {
                        match current_screen.get_now_playing() {
                            Some(now_playing) => {
                                now_playing.refresh(&mut spotify_client).await?;
                            }
                            None => {
                                let mut now_playing = NowPlaying::default();

                                now_playing.refresh(&mut spotify_client).await?;
                                current_screen.set_now_playing(Some(now_playing))?;
                            }
                        }
                    }
                }
                Message::PausePlay => {
                    if let Some(spotify_client) = app.spotify_client.clone() {
                        let mut player = SpotifyPlayer::new(spotify_client);

                        player.toggle_pause_play().await?;
                    }
                }
                Message::Shuffle => {
                    if let Some(spotify_client) = app.spotify_client.clone() {
                        let mut player = SpotifyPlayer::new(spotify_client);

                        player.toggle_shuffle().await?;
                    }
                }
                Message::NextSong => {
                    if let Some(spotify_client) = app.spotify_client.clone() {
                        let mut player = SpotifyPlayer::new(spotify_client);

                        player.next_song().await?;
                    }
                }
                Message::PrevSong => {
                    if let Some(spotify_client) = app.spotify_client.clone() {
                        let mut player = SpotifyPlayer::new(spotify_client);

                        player.previous_song().await?;
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
