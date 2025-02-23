use auth::server::AuthServer;
use clap::Parser;
use core::{
    app::App,
    clap::{Args, Command, ControlCommand},
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

pub async fn run() -> AppResult<()> {
    let args = Args::parse();

    install_panic_hook();

    setup_logging()?;

    let mut app = App::new()?;

    if let Some(command) = args.command {
        match command {
            Command::Control { control_command } => {
                app.spotify_client = Some(SpotifyClient::new(&app.config)?);

                if let Some(mut spotify_client) = app.spotify_client {
                    match control_command {
                        ControlCommand::PausePlay => {
                            spotify_client.toggle_pause_play(&app.config).await?;
                        }
                        ControlCommand::NextSong => {
                            spotify_client.next_song(&app.config).await?;
                        }
                        ControlCommand::PreviousSong => {
                            spotify_client.previous_song(&app.config).await?;
                        }
                        ControlCommand::Shuffle => {
                            spotify_client.toggle_shuffle(&app.config).await?;
                        }
                        ControlCommand::Devices => {
                            spotify_client.list_devices(&app.config).await?;
                        }
                        ControlCommand::Device { id } => {
                            spotify_client.set_device(id, &app.config).await?;
                        }
                    }
                }

                return Ok(());
            }
            _ => {}
        }
    }

    let mut terminal = init_terminal()?;
    let mut current_screen: Box<dyn Screen> = Box::new(HomeScreen::default());
    let mut auth_server = AuthServer::default();

    if app.config.client_id.is_none()
        || app.config.redirect_uri.is_none()
        || app.config.scope.is_none()
    {
        current_screen = Box::new(CreateConfigFormScreen::new(&app.config));
    }

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

                    if new_screen.get_screen_type() == ScreenType::ShowAuthLinkScreen {
                        auth_server.start(&app.config)?;
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
                        spotify_client
                            .set_code_and_access_token(code, &app.config.clone())
                            .await?;

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
