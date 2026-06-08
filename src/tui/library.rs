use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{List, ListItem, ListState},
};

use crate::{
    app::{App, Screen},
    tui::theme::{CONTENT_MAX_WIDTH, center_area},
};

pub fn draw_library(frame: &mut Frame, app: &mut App, area: Rect) {
    match &app.screen() {
        Screen::Library => draw_playlist_selector(frame, app, area),
        Screen::Playlist { .. } => {}
    }
}

pub fn draw_playlist_selector(frame: &mut Frame, app: &App, area: Rect) {
    let area = center_area(area, CONTENT_MAX_WIDTH);

    let playlist_rows = app.playlist_rows();

    let items: Vec<ListItem> = playlist_rows
        .iter()
        .map(|row| ListItem::new(Line::from(format!("{}", row.name))))
        .collect();

    let list = List::new(items)
        .highlight_symbol("▶ ")
        .highlight_style(Style::default().fg(Color::LightBlue));

    let mut state = ListState::default();

    if !playlist_rows.is_empty() {
        state.select(Some(app.selected_library));
    }

    frame.render_stateful_widget(list, area, &mut state);
}
