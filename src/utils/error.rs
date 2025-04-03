use crate::{core::message::Message, screens::error::ErrorScreen, AppResult};

pub fn handle_error<T>(result: AppResult<T>) -> Option<Message> {
    match result {
        Ok(_) => None,
        Err(error) => {
            let new_screen = Box::new(ErrorScreen::new(error.to_string()));

            Some(Message::ChangeScreen { new_screen })
        }
    }
}

pub fn throw_no_spotify_client_error() -> Option<Message> {
    let new_screen = Box::new(ErrorScreen::new("No `SpotifyClient` set on `App`."));

    Some(Message::ChangeScreen { new_screen })
}

pub fn throw_no_now_playing_error() -> Option<Message> {
    let new_screen = Box::new(ErrorScreen::new("No `NowPlaying` set on current screen."));

    Some(Message::ChangeScreen { new_screen })
}

pub fn throw_no_device_error() -> Option<Message> {
    let new_screen = Box::new(ErrorScreen::new("No `Device` set on current screen."));

    Some(Message::ChangeScreen { new_screen })
}
