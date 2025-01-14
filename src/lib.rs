use clap::Parser;
use core::{
    app::App,
    clap::{Args, Command},
    logging::setup_logging,
    tui::{init_terminal, install_panic_hook, restore_terminal},
};
use log::debug;
use screens::{create_config::CreateConfigFormScreen, home::HomeScreen, Screen, ScreenType};

mod components;
mod core;
mod layout;
mod screens;
mod widgets;

pub type AppResult<T> = color_eyre::Result<T>;

#[derive(Clone)]
pub enum Message {
    Quit,
    ChangeScreen { new_screen: Box<dyn Screen> },
    GoToPrevScreen,
    GoToNextScreen,
}

pub async fn run() -> AppResult<()> {
    let args = Args::parse();

    if let Some(command) = args.command {
        match command {
            Command::Control { control_command } => {
                println!("{:#?}", control_command);
                return Ok(());
            }
            _ => {}
        }
    }

    install_panic_hook();

    setup_logging()?;

    let mut app = App::new()?;
    let mut terminal = init_terminal()?;
    let mut current_screen: Box<dyn Screen> = Box::new(HomeScreen::default());

    if app.config.client_id.is_none() || app.config.redirect_uri.is_none() {
        current_screen = Box::new(CreateConfigFormScreen::new(&app.config));
    }

    debug!("{:#?}", app.config);

    while app.is_running {
        current_screen.tick();
        terminal.draw(|frame| current_screen.view(frame))?;

        let mut current_message = current_screen.handle_event(&mut app)?;

        while current_message.is_some() {
            match current_message.clone().unwrap() {
                Message::ChangeScreen { new_screen } => {
                    app.history.prev.push(current_screen);
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
                _ => {}
            }

            current_message = current_screen.handle_event(&mut app)?
        }
    }

    restore_terminal()?;

    Ok(())
}
