use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType},
};

#[allow(dead_code)]
pub fn create_block<'a>(color: Color) -> Block<'a> {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(color))
}

pub fn create_titled_block<'a>(title: &str, title_alignment: Alignment, color: Color) -> Block<'a> {
    Block::bordered()
        .title(format!(" {title} "))
        .title_alignment(title_alignment)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(color))
}
