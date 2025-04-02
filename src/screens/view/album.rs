use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Paragraph, Wrap},
    Frame,
};

use crate::{
    components::{screen_block::ScreenBlock, Component},
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
    album_id: String,
    acitve_song_index: usize,
    max_songs_shown: u16,
    song_start_index: usize,
    song_end_index: usize,
}

impl Default for ViewAlbumScreen {
    fn default() -> Self {
        Self {
            now_playing: NowPlaying::default(),
            album_id: String::new(),
            acitve_song_index: 0,
            max_songs_shown: 0,
            song_start_index: 0,
            song_end_index: 0,
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

    fn get_song_style(&self, index: usize) -> Style {
        let mut style = Style::default().fg(Color::Green);

        if self.acitve_song_index == index {
            style = Style::default().fg(Color::White).bg(Color::Green);
        }

        style
    }

    fn reset_song_list(&mut self, area: &Rect) {
        let max_songs_shown = area.height - 2;

        self.max_songs_shown = max_songs_shown;
        self.song_end_index = max_songs_shown.into();
        self.song_start_index = 0;
        self.acitve_song_index = 0;
        self.album_id = self.now_playing.album.id.clone();
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

        let mut song_constraits = Vec::<Constraint>::new();
        let mut song_paragraphs = Vec::<Paragraph>::new();

        if self.album_id != self.now_playing.album.id {
            self.reset_song_list(&chunks[0]);
        }

        for _ in 0..self.max_songs_shown {
            song_constraits.push(Constraint::Max(1))
        }

        for i in self.song_start_index..self.song_end_index {
            if i < self.now_playing.album.total_songs as usize {
                let song = &self.now_playing.album.songs[i];
                let mut song_string = format!("{}. {}", song.track_number, song.name);

                if song.id == self.now_playing.song.id {
                    song_string = format!("* {} *", song_string);
                }

                let paragraph = Paragraph::new(song_string)
                    .left_aligned()
                    .style(self.get_song_style(i))
                    .wrap(Wrap { trim: false });

                song_paragraphs.push(paragraph);
            }
        }

        let song_chunks = Layout::default()
            .margin(1)
            .direction(Direction::Vertical)
            .constraints(song_constraits)
            .split(chunks[0]);

        for i in 0..song_chunks.len() {
            if i < song_paragraphs.len() {
                frame.render_widget(song_paragraphs[i].clone(), song_chunks[i]);
            }
        }

        let song = &self.now_playing.album.songs[self.acitve_song_index];
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

    fn handle_key_press(&mut self, _: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        match key.code {
            KeyCode::Char('j') => {
                if self.acitve_song_index >= self.song_end_index - 1 {
                    self.song_start_index = self.song_start_index + 1;
                    self.song_end_index = self.song_end_index + 1;
                }

                if self.acitve_song_index < self.now_playing.album.songs.len() - 1 {
                    self.acitve_song_index = self.acitve_song_index + 1;
                } else {
                    self.acitve_song_index = 0;
                    self.song_start_index = 0;
                    self.song_end_index = self.max_songs_shown.into();
                }

                Ok(None)
            }
            KeyCode::Char('k') => {
                if self.acitve_song_index <= self.song_start_index && self.acitve_song_index != 0 {
                    self.song_start_index = self.song_start_index - 1;
                    self.song_end_index = self.song_end_index - 1;
                }

                if self.acitve_song_index == 0 {
                    self.acitve_song_index = self.now_playing.album.songs.len() - 1;
                    self.song_end_index = self.now_playing.album.songs.len();

                    if self.now_playing.album.total_songs < self.max_songs_shown as u64 {
                        self.song_start_index = 0;
                    } else {
                        self.song_start_index = (self.now_playing.album.total_songs
                            - self.max_songs_shown as u64)
                            as usize;
                    }
                } else {
                    self.acitve_song_index = self.acitve_song_index - 1;
                }

                Ok(None)
            }
            KeyCode::Enter => {
                let track_number = self.now_playing.album.songs[self.acitve_song_index]
                    .clone()
                    .track_number;
                let album_id = self.now_playing.album.id.clone();

                Ok(Some(Message::PlaySongOnAlbum {
                    track_number,
                    album_id,
                }))
            }
            _ => Ok(None),
        }
    }
}
