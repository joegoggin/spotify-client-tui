use auth::server::AuthServer;
use clap::Parser;
use core::{
    app::App,
    clap::{Args, Command, ControlCommand},
    config::Config,
    logging::setup_logging,
    spotify::SpotifyClient,
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
    SetAuthCode { code: String },
}

fn is_control_command(args: &Args) -> bool {
    if let Some(command) = args.command.clone() {
        match command {
            Command::Control { .. } => return true,
            _ => {}
        }
    }
    return false;
}

async fn handle_control_command(args: &Args, app: &App) -> AppResult<bool> {
    if let Some(command) = args.command.clone() {
        match command {
            Command::Control { control_command } => {
                if let Some(mut spotify_client) = app.spotify_client.clone() {
                    match control_command {
                        ControlCommand::PausePlay => {
                            spotify_client.toggle_pause_play().await?;
                        }
                        ControlCommand::NextSong => {
                            spotify_client.next_song().await?;
                        }
                        ControlCommand::PreviousSong => {
                            spotify_client.previous_song().await?;
                        }
                        ControlCommand::Shuffle => {
                            spotify_client.toggle_shuffle().await?;
                        }
                        ControlCommand::Devices => {
                            spotify_client.list_devices().await?;
                        }
                        ControlCommand::Device { id } => {
                            spotify_client.set_device(id).await?;
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

        if is_control_command(&args) {
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

                    if new_screen.get_screen_type() == ScreenType::Home && is_control_command(&args)
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
                _ => {}
            }

            current_message = current_screen.handle_event(&mut app)?
        }
    }

    restore_terminal()?;

    Ok(())
}
