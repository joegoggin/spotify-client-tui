use crate::{
    core::{app::AppResult, message::Message},
    screens::error::ErrorScreen,
};

pub fn handle_error<T>(result: AppResult<T>) -> Option<Message> {
    match result {
        Ok(_) => None,
        Err(error) => {
            let new_screen = Box::new(ErrorScreen::new(error.to_string()));

            Some(Message::ChangeScreen { new_screen })
        }
    }
}

pub fn not_set_on_screen_message(struct_name: &str) -> String {
    format!("No `{}` set on current screen.", struct_name)
}

pub fn throw_no_spotify_client_error() -> Option<Message> {
    let new_screen = Box::new(ErrorScreen::new("No `SpotifyClient` set on `App`."));

    Some(Message::ChangeScreen { new_screen })
}

pub fn throw_no_now_playing_error() -> Option<Message> {
    let new_screen = Box::new(ErrorScreen::new(not_set_on_screen_message("NowPlaying")));

    Some(Message::ChangeScreen { new_screen })
}

pub fn throw_no_device_error() -> Option<Message> {
    let new_screen = Box::new(ErrorScreen::new(not_set_on_screen_message("Device")));

    Some(Message::ChangeScreen { new_screen })
}

pub fn throw_no_song_error() -> Option<Message> {
    let new_screen = Box::new(ErrorScreen::new(not_set_on_screen_message("Song")));

    Some(Message::ChangeScreen { new_screen })
}

pub fn throw_no_album_error() -> Option<Message> {
    let new_screen = Box::new(ErrorScreen::new(not_set_on_screen_message("Album")));

    Some(Message::ChangeScreen { new_screen })
}
