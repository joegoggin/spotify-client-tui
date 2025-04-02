use ratatui::{
    style::{Color, Style},
    widgets::{Block, Paragraph, Wrap},
};

pub fn create_paragraph(text: &str, color: Option<Color>) -> Paragraph {
    match color {
        Some(color) => Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(color)),
        None => Paragraph::new(text).wrap(Wrap { trim: true }),
    }
}

pub fn create_centered_paragraph(text: &str, color: Option<Color>) -> Paragraph {
    match color {
        Some(color) => Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .centered()
            .style(Style::default().fg(color)),
        None => Paragraph::new(text).wrap(Wrap { trim: true }).centered(),
    }
}

pub fn create_left_aligned_paragraph(text: &str, color: Option<Color>) -> Paragraph {
    match color {
        Some(color) => Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .left_aligned()
            .style(Style::default().fg(color)),
        None => Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .left_aligned(),
    }
}

pub fn create_right_aligned_paragraph(text: &str, color: Option<Color>) -> Paragraph {
    match color {
        Some(color) => Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .right_aligned()
            .style(Style::default().fg(color)),
        None => Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .right_aligned(),
    }
}

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
