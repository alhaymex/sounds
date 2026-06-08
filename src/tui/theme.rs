use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Span,
};

pub const CONTENT_MAX_WIDTH: u16 = 80;

pub fn key_inline(text: impl Into<String>) -> Span<'static> {
    Span::styled(text.into(), Style::default().fg(Color::LightBlue))
}

pub fn muted(text: impl Into<String>) -> Span<'static> {
    Span::styled(text.into(), Style::default().fg(Color::DarkGray))
}

pub fn center_area(area: Rect, max_width: u16) -> Rect {
    let width = area.width.min(max_width);

    Rect {
        x: area.x + (area.width.saturating_sub(width)) / 2,
        y: area.y,
        width: width,
        height: area.height,
    }
}
