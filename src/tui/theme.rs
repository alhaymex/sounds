use ratatui::{
    style::{Color, Style},
    text::Span,
};

pub fn key_inline(text: impl Into<String>) -> Span<'static> {
    Span::styled(text.into(), Style::default().fg(Color::LightBlue))
}

pub fn muted(text: impl Into<String>) -> Span<'static> {
    Span::styled(text.into(), Style::default().fg(Color::DarkGray))
}

pub fn gap() -> Span<'static> {
    Span::raw("  ")
}
