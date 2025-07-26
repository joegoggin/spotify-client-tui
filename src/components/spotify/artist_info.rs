use ratatui::{
    crossterm::event::KeyEvent,
    layout::{Constraint, Layout, Rect},
    style::Color,
    Frame,
};

use crate::{
    components::{loading::Loading, Component},
    core::{
        message::Message,
        spotify::{artist::Artist, now_playing::NowPlaying},
    },
    utils::string::Capitalize,
    widgets::paragraph::create_centered_paragraph,
    App, AppResult,
};

#[derive(Clone)]
pub struct ArtistInfo {
    area: Rect,
    now_playing: NowPlaying,
    artist: Artist,
}

impl Default for ArtistInfo {
    fn default() -> Self {
        Self {
            area: Rect::default(),
            now_playing: NowPlaying::default(),
            artist: Artist::default(),
        }
    }
}

impl Component for ArtistInfo {
    fn view(&mut self, app: &App, frame: &mut Frame) {
        if self.now_playing.is_empty() || self.artist.is_empty() {
            let mut loading = Loading::default();

            loading.set_area(&self.area);
            loading.view(app, frame);
            return;
        }

        let followers_string = format!("{} Followers", self.artist.followers);
        let listeners_string = format!("{} Monthly Listeners", self.artist.monthly_listeners);
        let mut genre_string = "Genres: ".to_string();

        for (i, genre) in self.artist.genres.iter().enumerate() {
            if i == self.artist.genres.len() - 1 {
                genre_string += &genre.capitalize();
            } else {
                genre_string += &format!("{}, ", genre.capitalize());
            }
        }

        let name_paragraph = create_centered_paragraph(&self.artist.name, Some(Color::Green));
        let followers_paragraph = create_centered_paragraph(&followers_string, Some(Color::Green));
        let listeners_paragraph = create_centered_paragraph(&listeners_string, Some(Color::Green));
        let genres_paragraph = create_centered_paragraph(&genre_string, Some(Color::Green));

        let chunks = Layout::default()
            .margin(3)
            .constraints(vec![
                Constraint::Min(1),
                Constraint::Min(1),
                Constraint::Min(1),
                Constraint::Min(1),
            ])
            .split(self.area);

        frame.render_widget(name_paragraph, chunks[0]);
        frame.render_widget(followers_paragraph, chunks[1]);
        frame.render_widget(listeners_paragraph, chunks[2]);
        frame.render_widget(genres_paragraph, chunks[3]);
    }

    fn tick(&mut self, _: &mut App) -> AppResult<Option<Message>> {
        if !self.now_playing.is_empty() && self.now_playing.artist_ids[0] != self.artist.id {
            self.artist.id = self.now_playing.artist_ids[0].clone();

            return Ok(Some(Message::RefreshArtist));
        }

        Ok(Some(Message::RefreshNowPlaying))
    }

    fn handle_key_press(&mut self, _: &mut App, _: KeyEvent) -> AppResult<Option<Message>> {
        Ok(None)
    }

    fn get_area(&mut self) -> Rect {
        self.area
    }

    fn set_area(&mut self, area: Rect) {
        self.area = area;
    }

    fn get_now_playing(&mut self) -> Option<&mut NowPlaying> {
        Some(&mut self.now_playing)
    }

    fn get_artist(&mut self) -> Option<&mut Artist> {
        Some(&mut self.artist)
    }
}
