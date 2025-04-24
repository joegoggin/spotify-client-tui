use ratatui::{
    crossterm::event::KeyEvent,
    layout::{Constraint, Direction, Layout},
    style::Color,
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
        app::{App, AppResult},
        message::Message,
        spotify::{album::Album, now_playing::NowPlaying, song::Song},
    },
    screens::{Screen, ScreenType},
    widgets::block::create_block,
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

        if !self.song_list.album.is_empty() {
            title = format!(
                "{} - {}",
                self.song_list.album.name,
                self.song_list.album.get_first_artist()
            )
        }

        title
    }
}

impl Screen for ViewAlbumScreen {
    fn get_screen_type(&self) -> ScreenType {
        ScreenType::ViewAlbumScreen
    }
}

impl Component for ViewAlbumScreen {
    fn view(&mut self, app: &App, frame: &mut Frame) {
        ScreenBlock::new_with_color(self.get_title(), Color::Green).view(app, frame);

        if self.now_playing.is_empty() {
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

        self.song_list.set_area(&chunks[0]);
        self.info_window.set_area(&chunks[1]);

        self.song_list.view(app, frame);
        self.info_window.view(app, frame);
    }

    fn tick(&mut self, _: &mut App) -> AppResult<Option<Message>> {
        if self.now_playing.song_id != self.song_list.current_song_id {
            self.song_list.current_song_id = self.now_playing.song_id.clone();
        }

        if self.song_list.get_active_song_id() != self.info_window.song.id {
            self.info_window.song.id = self.song_list.get_active_song_id();

            return Ok(Some(Message::RefreshSong));
        }

        if self.now_playing.album_id != self.song_list.album.id {
            self.song_list
                .set_album_id(self.now_playing.album_id.clone());

            return Ok(Some(Message::RefreshAlbum));
        }

        Ok(Some(Message::RefreshNowPlaying))
    }

    fn handle_key_press(&mut self, app: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        self.song_list.handle_key_press(app, key)
    }

    fn get_now_playing(&mut self) -> Option<&mut NowPlaying> {
        Some(&mut self.now_playing)
    }

    fn get_album(&mut self) -> Option<&mut Album> {
        Some(&mut self.song_list.album)
    }

    fn get_song(&mut self) -> Option<&mut Song> {
        Some(&mut self.info_window.song)
    }
}
