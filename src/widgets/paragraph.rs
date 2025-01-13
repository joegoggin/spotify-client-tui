use ratatui::{
    style::{Color, Style},
    widgets::{Block, Paragraph, Wrap},
};

#[allow(dead_code)]
pub fn create_paragraph(text: &str, color: Color) -> Paragraph {
    Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(color))
}

#[allow(dead_code)]
pub fn create_centered_paragraph(text: &str, color: Color) -> Paragraph {
    Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .centered()
        .style(Style::default().fg(color))
}

#[allow(dead_code)]
pub fn create_paragraph_with_block<'a>(
    text: &str,
    block: Block<'a>,
    color: Color,
) -> Paragraph<'a> {
    Paragraph::new(text.to_string())
        .wrap(Wrap { trim: true })
        .block(block)
        .style(Style::default().fg(color))
}
