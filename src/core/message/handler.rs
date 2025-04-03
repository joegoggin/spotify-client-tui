use crate::{
    auth::server::AuthServer,
    core::{app::App, clap::Args, spotify::player::SpotifyPlayer},
    screens::{home::HomeScreen, Screen, ScreenType},
    utils::error::{
        handle_error, throw_no_device_error, throw_no_now_playing_error,
        throw_no_spotify_client_error,
    },
    AppResult,
};

use super::Message;

pub struct MessageHandler<'a> {
    pub current_message: Option<Message>,
    pub app: &'a mut App,
    pub current_screen: &'a mut Box<dyn Screen>,
    pub auth_server: &'a mut AuthServer,
    pub args: &'a Args,
}

impl<'a> MessageHandler<'a> {
    pub fn new(
        app: &'a mut App,
        current_screen: &'a mut Box<dyn Screen>,
        auth_server: &'a mut AuthServer,
        args: &'a Args,
    ) -> Self {
        Self {
            current_message: None,
            app,
            current_screen,
            auth_server,
            args,
        }
    }

    pub async fn handle_message(&mut self) -> AppResult<()> {
        self.current_message = self.current_screen.tick(&mut self.app)?;

        if self.current_message.is_none() {
            self.current_message = self.current_screen.handle_event(&mut self.app)?
        }

        while self.current_message.is_some() {
            self.current_message = match self.current_message.clone().unwrap() {
                Message::ChangeScreen { new_screen } => self.change_screen(new_screen).await?,
                Message::GoToPrevScreen => self.go_to_prev_screen(),
                Message::GoToNextScreen => self.go_to_next_screen(),
                Message::SetAuthCode { code } => self.set_auth_code(code).await?,
                Message::RefreshNowPlaying => self.refresh_now_playing().await,
                Message::PausePlay => self.pause_play().await,
                Message::Shuffle => self.shuffle().await,
                Message::NextSong => self.next_song().await,
                Message::PrevSong => self.prev_song().await,
                Message::RefreshDevice => self.refresh_device().await,
                Message::SetDevice { name, id } => self.set_device(name, id).await,
                Message::PlaySongOnAlbum {
                    track_number,
                    album_id,
                } => self.play_song_on_album(track_number, album_id).await,
            };

            if self.current_message.is_some() {
                continue;
            }

            self.current_message = self.current_screen.handle_event(&mut self.app)?
        }

        Ok(())
    }

    async fn change_screen(&mut self, new_screen: Box<dyn Screen>) -> AppResult<Option<Message>> {
        self.app.history.prev.push(self.current_screen.to_owned());

        if let Some(spotify_client) = &self.app.spotify_client {
            if new_screen.get_screen_type() == ScreenType::ShowAuthLinkScreen {
                self.auth_server.start(&spotify_client.config)?;
            }
        }

        if let Some(command) = &self.args.command {
            if new_screen.get_screen_type() == ScreenType::Home && command.is_player_command() {
                self.app.is_running = false;
                command
                    .handle_command(&mut self.app, &mut self.current_screen)
                    .await?;
            }
        }

        *self.current_screen = new_screen;

        Ok(None)
    }

    fn go_to_prev_screen(&mut self) -> Option<Message> {
        if let Some(last_screen) = self.app.history.prev.pop() {
            if self.current_screen.get_screen_type() != ScreenType::Exit {
                self.app.history.next.push(self.current_screen.clone_box());
            }

            *self.current_screen = last_screen;
        }

        None
    }

    fn go_to_next_screen(&mut self) -> Option<Message> {
        if let Some(next_screen) = self.app.history.next.pop() {
            if self.current_screen.get_screen_type() != ScreenType::Exit {
                self.app.history.prev.push(self.current_screen.clone_box())
            }

            *self.current_screen = next_screen;
        }

        None
    }

    async fn set_auth_code(&mut self, code: String) -> AppResult<Option<Message>> {
        match self.app.spotify_client.as_mut() {
            Some(spotify_client) => {
                let result = spotify_client.set_code_and_access_token(code).await;

                if spotify_client.credentials.is_some() {
                    self.auth_server.stop()?;

                    let new_screen = Box::new(HomeScreen::default());

                    return Ok(Some(Message::ChangeScreen { new_screen }));
                }

                Ok(handle_error(result))
            }
            None => Ok(throw_no_spotify_client_error()),
        }
    }

    async fn refresh_now_playing(&mut self) -> Option<Message> {
        match self.app.spotify_client.as_mut() {
            Some(mut spotify_client) => match self.current_screen.get_now_playing() {
                Some(now_playing) => {
                    let result = now_playing.refresh(&mut spotify_client).await;

                    handle_error(result)
                }
                None => throw_no_now_playing_error(),
            },
            None => throw_no_spotify_client_error(),
        }
    }

    async fn pause_play(&mut self) -> Option<Message> {
        match self.app.spotify_client.as_mut() {
            Some(mut spotify_client) => {
                let player = SpotifyPlayer::new();
                let result = player.toggle_pause_play(&mut spotify_client).await;

                handle_error(result)
            }
            None => throw_no_spotify_client_error(),
        }
    }

    async fn shuffle(&mut self) -> Option<Message> {
        match self.app.spotify_client.as_mut() {
            Some(mut spotify_client) => {
                let player = SpotifyPlayer::new();
                let result = player.toggle_shuffle(&mut spotify_client).await;

                handle_error(result)
            }
            None => throw_no_spotify_client_error(),
        }
    }

    async fn next_song(&mut self) -> Option<Message> {
        match self.app.spotify_client.as_mut() {
            Some(mut spotify_client) => {
                let player = SpotifyPlayer::new();
                let result = player.next_song(&mut spotify_client).await;

                handle_error(result)
            }
            None => throw_no_spotify_client_error(),
        }
    }

    async fn prev_song(&mut self) -> Option<Message> {
        match self.app.spotify_client.as_mut() {
            Some(mut spotify_client) => {
                let player = SpotifyPlayer::new();
                let result = player.previous_song(&mut spotify_client).await;

                handle_error(result)
            }
            None => throw_no_spotify_client_error(),
        }
    }

    async fn refresh_device(&mut self) -> Option<Message> {
        match self.app.spotify_client.as_mut() {
            Some(mut spotify_client) => match self.current_screen.get_device() {
                Some(device) => {
                    let result = device.refresh(&mut spotify_client).await;

                    handle_error(result)
                }
                None => throw_no_device_error(),
            },
            None => throw_no_spotify_client_error(),
        }
    }

    async fn set_device(&mut self, name: String, id: String) -> Option<Message> {
        match self.app.spotify_client.as_mut() {
            Some(mut spotify_client) => match self.current_screen.get_device() {
                Some(device) => {
                    let result = device.set_current_device(&mut spotify_client, id).await;

                    device.current_device_name = Some(name.to_string());

                    handle_error(result)
                }
                None => throw_no_device_error(),
            },
            None => throw_no_spotify_client_error(),
        }
    }

    async fn play_song_on_album(&mut self, track_number: u64, album_id: String) -> Option<Message> {
        match self.app.spotify_client.as_mut() {
            Some(mut spotify_client) => {
                let player = SpotifyPlayer::new();
                let result = player
                    .play_song_on_album(&mut spotify_client, track_number, album_id)
                    .await;

                handle_error(result)
            }
            None => throw_no_spotify_client_error(),
        }
    }
}
