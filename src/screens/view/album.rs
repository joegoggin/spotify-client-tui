use ratatui::{
    crossterm::event::KeyEvent,
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{Paragraph, Wrap},
    Frame,
};

use crate::{
    components::{
        loading::Loading,
        screen_block::ScreenBlock,
        spotify::{album_song_list::AlbumSongList, song_info_window::SongInfoWindow},
        Component,
    },
    core::{
        app::App,
        config::Config,
        spotify::{client::SpotifyClient, now_playing::NowPlaying},
    },
    screens::{
        auth::{create_config::CreateConfigFormScreen, show_link::ShowAuthLinkScreen},
        Screen, ScreenType,
    },
    widgets::block::create_block,
    AppResult, Message,
};

#[derive(Debug, Clone)]
pub struct ViewAlbumScreen {
    now_playing: NowPlaying,
    song_list: AlbumSongList,
    info_window: SongInfoWindow,
}

impl Default for ViewAlbumScreen {
    fn default() -> Self {
        Self {
            now_playing: NowPlaying::default(),
            song_list: AlbumSongList::default(),
            info_window: SongInfoWindow::default(),
        }
    }
}

impl ViewAlbumScreen {
    fn get_title(&self) -> String {
        let mut title = "Veiw Album".to_string();

        if !self.now_playing.album.is_empty() {
            title = format!(
                "{} - {}",
                self.now_playing.album.name,
                self.now_playing.album.get_first_artist()
            )
        }

        title
    }
}

impl Screen for ViewAlbumScreen {
    fn get_screen_type(&self) -> ScreenType {
        ScreenType::ViewAlbumScreen
    }

    fn get_now_playing(&mut self) -> Option<&mut NowPlaying> {
        Some(&mut self.now_playing)
    }
}

impl Component for ViewAlbumScreen {
    fn view(&mut self, app: &App, frame: &mut Frame) {
        ScreenBlock::new_with_color(self.get_title(), Color::Green).view(app, frame);

        if self.now_playing.album.is_empty() {
            Loading::default().view(app, frame);
            return;
        }

        let song_list_block = create_block(Color::Green);
        let info_block = create_block(Color::Green);

        let chunks = Layout::default()
            .margin(5)
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.area());

        frame.render_widget(song_list_block.clone(), chunks[0]);
        frame.render_widget(info_block, chunks[1]);

        self.song_list.refresh(
            self.now_playing.album.clone(),
            Some(self.now_playing.song.clone()),
            &chunks[0],
        );
        self.song_list.view(app, frame);

        let song = self.now_playing.album.songs[self.song_list.active_song_index].clone();

        self.info_window
            .refresh(self.now_playing.album.clone(), song, &chunks[1]);
        self.info_window.view(app, frame);
    }

    fn tick(&mut self, app: &mut App) -> AppResult<Option<Message>> {
        match app.spotify_client.clone() {
            Some(spotify_client) => {
                if spotify_client.credentials.is_none() {
                    let new_screen = Box::new(ShowAuthLinkScreen::new(spotify_client.auth_url));

                    return Ok(Some(Message::ChangeScreen { new_screen }));
                }

                Ok(Some(Message::RefreshNowPlaying))
            }
            None => {
                let config = Config::new()?;
                let result = SpotifyClient::new(config.clone());

                match result {
                    Ok(spotify_client) => {
                        app.spotify_client = Some(spotify_client);

                        Ok(None)
                    }
                    Err(_) => {
                        let new_screen = Box::new(CreateConfigFormScreen::new(&config));

                        Ok(Some(Message::ChangeScreen { new_screen }))
                    }
                }
            }
        }
    }

    fn handle_key_press(&mut self, app: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        self.song_list.handle_key_press(app, key)
    }
}
