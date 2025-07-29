use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

use crate::core::{
    app::{App, AppResult},
    message::Message,
    spotify::{album::Album, artist::Artist, device::Device, now_playing::NowPlaying, song::Song},
};

pub trait Component: ComponentClone {
    fn view(&mut self, app: &App, frame: &mut Frame);

    fn tick(&mut self, app: &mut App) -> AppResult<Option<Message>>;

    fn handle_key_press(&mut self, app: &mut App, key: KeyEvent) -> AppResult<Option<Message>>;

    #[allow(unused_variables)]
    fn set_area(&mut self, area: Rect) {}

    fn get_area(&mut self) -> Rect {
        Rect::default()
    }

    fn get_now_playing(&mut self) -> Option<&mut NowPlaying> {
        None
    }

    fn get_device(&mut self) -> Option<&mut Device> {
        None
    }

    fn get_song(&mut self) -> Option<&mut Song> {
        None
    }

    fn get_album(&mut self) -> Option<&mut Album> {
        None
    }

    fn get_artist(&mut self) -> Option<&mut Artist> {
        None
    }
}

pub trait ComponentClone {
    fn clone_component_box(&self) -> Box<dyn Component>;
}

impl<T> ComponentClone for T
where
    T: 'static + Component + Clone,
{
    fn clone_component_box(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Component> {
    fn clone(&self) -> Box<dyn Component> {
        self.clone_component_box()
    }
}
