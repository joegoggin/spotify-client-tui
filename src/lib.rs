use clap::Parser;
use core::{
    app::App,
    clap::{Args, Command},
    tui::{init_terminal, install_panic_hook, restore_terminal},
};
use screens::{home::HomeScreen, Screen, ScreenType};

mod components;
mod core;
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
    let mut app = App::default();
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

    let mut terminal = init_terminal()?;
    let mut current_screen: Box<dyn Screen> = Box::new(HomeScreen::default());

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
