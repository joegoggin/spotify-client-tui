use std::time::Duration;

use clap::ValueEnum;
use ratatui::crossterm::event::{self, Event};

use crate::{
    components::Component,
    core::{
        app::{App, AppResult},
        message::Message,
        spotify::{device::Device, now_playing::NowPlaying},
    },
};

#[derive(ValueEnum, PartialEq, Debug, Clone)]
pub enum ScreenType {
    Home,
    Exit,
    CreateConfigFormScreen,
    ShowAuthLinkScreen,
    EnterAuthCodeScreen,
    NowPlayingScreen,
    ViewArtistScreen,
    ViewAlbumScreen,
    QueueScreen,
    SearchScreen,
    LibraryScreen,
    ErrorScreen,
    DevicesScreen,
}

pub trait Screen: ScreenClone + Component {
    fn get_screen_type(&self) -> ScreenType;

    fn get_default_key_press_enabled(&self) -> bool {
        true
    }

    fn handle_event(&mut self, app: &mut App) -> AppResult<Option<Message>> {
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    if self.get_default_key_press_enabled() {
                        if let Some(message) = app.handle_default_key_press(key)? {
                            return Ok(Some(message));
                        }
                    }

                    let message = self.handle_key_press(app, key)?;

                    return Ok(message);
                }
            }
        }

        Ok(None)
    }

    fn get_now_playing(&mut self) -> Option<&mut NowPlaying> {
        None
    }

    fn get_device(&mut self) -> Option<&mut Device> {
        None
    }
}

pub trait ScreenClone {
    fn clone_box(&self) -> Box<dyn Screen>;
}

impl<T> ScreenClone for T
where
    T: 'static + Screen + Clone,
{
    fn clone_box(&self) -> Box<dyn Screen> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Screen> {
    fn clone(&self) -> Box<dyn Screen> {
        self.clone_box()
    }
}
