use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout},
    style::Color,
    Frame,
};

use crate::{
    components::{
        form::{form::FormMode, text_input::TextInput},
        list::List,
        screen_block::ScreenBlock,
        Component,
    },
    core::{
        app::{App, AppResult},
        message::Message,
        spotify::search_results::SearchResults,
    },
    widgets::block::create_block,
};

use super::{Screen, ScreenType};

#[derive(Clone, Debug)]
pub struct SearchScreen {
    search_bar: TextInput,
    refresh_results: bool,
    search_results: SearchResults,
    result_list: List,
}

impl Default for SearchScreen {
    fn default() -> Self {
        let mut search_bar = TextInput::new("Search", "", false);
        search_bar.is_focused = true;
        search_bar.mode = FormMode::Insert;

        Self {
            search_bar,
            refresh_results: false,
            search_results: SearchResults::default(),
            result_list: List::default(),
        }
    }
}

impl SearchScreen {
    fn set_query(&mut self) {
        self.refresh_results = true;
        self.search_results.set_query(self.search_bar.value.clone());
    }
}

impl Screen for SearchScreen {
    fn get_screen_type(&self) -> ScreenType {
        ScreenType::SearchScreen
    }
}

impl Component for SearchScreen {
    fn view(&mut self, app: &App, frame: &mut Frame) {
        ScreenBlock::new_with_color("Search", Color::Green).view(app, frame);

        let vertical_chunks = Layout::default()
            .margin(3)
            .constraints(vec![Constraint::Max(3), Constraint::Fill(1)])
            .split(frame.area());

        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(vertical_chunks[1]);

        let block_color = match self.search_bar.mode {
            FormMode::Insert => Color::White,
            FormMode::Normal => Color::Green,
        };

        let result_block = create_block(block_color);
        let info_block = create_block(block_color);

        frame.render_widget(result_block, horizontal_chunks[0]);
        frame.render_widget(info_block, horizontal_chunks[1]);

        self.search_bar.set_area(vertical_chunks[0]);
        self.search_bar.view(app, frame);

        if self.search_bar.mode == FormMode::Normal {
            self.result_list.set_area(horizontal_chunks[0]);
            self.result_list.view(app, frame);
        }
    }

    fn tick(&mut self, app: &mut App) -> AppResult<Option<Message>> {
        if let Some(message) = self.search_bar.tick(app)? {
            return Ok(Some(message));
        }

        if self.refresh_results {
            self.refresh_results = false;

            return Ok(Some(Message::RefreshSearchResults));
        }

        if self.search_results.get_top_results() != self.result_list.items
            && self.search_bar.mode == FormMode::Normal
        {
            self.result_list
                .set_items(self.search_results.get_top_results());
        }

        if self.search_bar.mode == FormMode::Insert && !self.result_list.items.is_empty() {
            self.result_list.set_items(vec![]);
            self.result_list.active_index = 0;
        }

        Ok(None)
    }

    fn handle_key_press(&mut self, app: &mut App, key: KeyEvent) -> AppResult<Option<Message>> {
        if let Some(message) = self.search_bar.handle_key_press(app, key)? {
            return Ok(Some(message));
        }

        if let Some(message) = self.result_list.handle_key_press(app, key)? {
            return Ok(Some(message));
        }

        match key.code {
            KeyCode::Enter => {
                self.search_bar.mode = FormMode::Normal;
                self.set_query();
            }
            KeyCode::Char('/') => {
                if self.search_bar.mode == FormMode::Normal {
                    self.search_bar.mode = FormMode::Insert
                }
            }
            _ => {}
        }

        Ok(None)
    }

    fn get_search_results(&mut self) -> Option<&mut SearchResults> {
        Some(&mut self.search_results)
    }
}
