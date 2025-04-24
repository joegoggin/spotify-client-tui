use ratatui::{
    crossterm::event::KeyEvent,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Paragraph, Wrap},
    Frame,
};

use crate::{
    components::Component,
    core::{
        message::Message,
        spotify::{
            album::Album, artist::Artist, device::Device, now_playing::NowPlaying, song::Song,
        },
    },
    App, AppResult,
};

use super::tab::Tab;

#[derive(Clone)]
pub struct TabbedView {
    pub tabs: Vec<Tab>,
    active_tab: usize,
}

impl Default for TabbedView {
    fn default() -> Self {
        Self {
            tabs: vec![],
            active_tab: 0,
        }
    }
}

impl TabbedView {
    pub fn new(tabs: Vec<Tab>) -> Self {
        Self {
            tabs,
            active_tab: 0,
        }
    }

    pub fn get_active_component(&mut self) -> Option<&mut Box<dyn Component>> {
        if self.active_tab < self.tabs.len() {
            return Some(&mut self.tabs[self.active_tab].component);
        }

        None
    }
}

impl Component for TabbedView {
    fn view(&mut self, app: &App, frame: &mut Frame) {
        let mut menu_options: Vec<Paragraph> = vec![];
        let mut constraints: Vec<Constraint> = vec![];

        for (i, tab) in self.tabs.iter().enumerate() {
            let paragraph_string = format!("{} - {}", tab.key.to_string(), tab.title);
            let mut style = Style::default().fg(Color::White);

            if self.active_tab == i {
                style = style.bg(Color::Green);
            }

            let paragraph = Paragraph::new(paragraph_string)
                .style(style)
                .centered()
                .wrap(Wrap { trim: true });

            menu_options.push(paragraph);
            constraints.push(Constraint::Min(1))
        }

        let verticle_chunks = Layout::default()
            .margin(2)
            .constraints(vec![
                Constraint::Max(1),
                Constraint::Max(1),
                Constraint::Min(1),
            ])
            .split(frame.area());

        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(verticle_chunks[0]);

        for (i, option) in menu_options.iter().enumerate() {
            frame.render_widget(option, horizontal_chunks[i]);
        }

        if let Some(component) = self.get_active_component() {
            component.set_area(verticle_chunks[2]);
            component.view(app, frame);
        }
    }

    fn tick(&mut self, app: &mut App) -> AppResult<Option<Message>> {
        if let Some(component) = self.get_active_component() {
            return component.tick(app);
        }

        Ok(None)
    }

    fn handle_key_press(&mut self, app: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        for (i, tab) in self.tabs.iter().enumerate() {
            if key.code == tab.key {
                self.active_tab = i;

                return Ok(None);
            }
        }

        if let Some(component) = self.get_active_component() {
            return component.handle_key_press(app, key);
        }

        Ok(None)
    }

    fn get_now_playing(&mut self) -> Option<&mut NowPlaying> {
        match self.get_active_component() {
            Some(component) => component.get_now_playing(),
            None => None,
        }
    }

    fn get_device(&mut self) -> Option<&mut Device> {
        match self.get_active_component() {
            Some(component) => component.get_device(),
            None => None,
        }
    }

    fn get_song(&mut self) -> Option<&mut Song> {
        match self.get_active_component() {
            Some(component) => component.get_song(),
            None => None,
        }
    }

    fn get_album(&mut self) -> Option<&mut Album> {
        match self.get_active_component() {
            Some(component) => component.get_album(),
            None => None,
        }
    }

    fn get_artist(&mut self) -> Option<&mut Artist> {
        match self.get_active_component() {
            Some(component) => component.get_artist(),
            None => None,
        }
    }
}
