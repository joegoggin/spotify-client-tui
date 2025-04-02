use ratatui::{
    crossterm::event::KeyEvent,
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{Paragraph, Wrap},
    Frame,
};

use crate::{
    components::{screen_block::ScreenBlock, spotify::album::song_list::AlbumSongList, Component},
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
}

impl Default for ViewAlbumScreen {
    fn default() -> Self {
        Self {
            now_playing: NowPlaying::default(),
            song_list: AlbumSongList::default(),
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
            let paragraph = Paragraph::new("Loading ...").centered();

            let chunks = Layout::default()
                .margin(5)
                .constraints(vec![Constraint::Min(1)])
                .split(frame.area());

            frame.render_widget(paragraph, chunks[0]);
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

        let song = &self.now_playing.album.songs[self.song_list.active_song_index];
        let song_string = format!("Song: {}", song.name);
        let artists_string = format!("Artists: {}", song.get_artists_string());
        let album_string = format!("Album: {}", self.now_playing.album.name);
        let year_string = format!("Year: {}", self.now_playing.album.year);
        let disk_string = format!("Disk: {}", song.disk_number);
        let track_string = format!("Track {}", song.track_number);

        let song_paragraph = Paragraph::new(song_string)
            .left_aligned()
            .wrap(Wrap { trim: false });
        let artists_paragraph = Paragraph::new(artists_string)
            .left_aligned()
            .wrap(Wrap { trim: false });
        let album_paragraph = Paragraph::new(album_string)
            .left_aligned()
            .wrap(Wrap { trim: false });
        let year_paragraph = Paragraph::new(year_string)
            .left_aligned()
            .wrap(Wrap { trim: false });
        let disk_paragraph = Paragraph::new(disk_string)
            .left_aligned()
            .wrap(Wrap { trim: false });
        let track_paragraph = Paragraph::new(track_string)
            .left_aligned()
            .wrap(Wrap { trim: false });

        let mut info_constraints = Vec::<Constraint>::new();

        for _ in 0..7 {
            info_constraints.push(Constraint::Min(5));
        }

        info_constraints.push(Constraint::Min(0));

        let info_chunks = Layout::default()
            .margin(1)
            .direction(Direction::Vertical)
            .constraints(info_constraints)
            .split(chunks[1]);

        frame.render_widget(song_paragraph, info_chunks[0]);
        frame.render_widget(artists_paragraph, info_chunks[1]);
        frame.render_widget(album_paragraph, info_chunks[2]);
        frame.render_widget(year_paragraph, info_chunks[3]);
        frame.render_widget(disk_paragraph, info_chunks[4]);
        frame.render_widget(track_paragraph, info_chunks[5]);
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
