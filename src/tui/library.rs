use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, List, ListItem, ListState},
};

use crate::{
    app::{App, Screen},
    tui::theme::{CONTENT_MAX_WIDTH, center_area},
};

pub fn draw_library(frame: &mut Frame, app: &mut App, area: Rect) {
    match &app.screen {
        Screen::Library => draw_playlist_selector(frame, app, area),
        Screen::Playlist { path } => draw_song_selector(frame, app, area, path),

        Screen::Options => {}
        Screen::Help => {}
    }
}

fn draw_playlist_selector(frame: &mut Frame, app: &App, area: Rect) {
    let area = center_area(area, CONTENT_MAX_WIDTH);

    let playlist_rows = app.playlist_rows();

    let items: Vec<ListItem> = playlist_rows
        .iter()
        .map(|row| ListItem::new(Line::from(format!("{}", row.name))))
        .collect();

    let list = List::new(items)
        .highlight_symbol("▶ ")
        .highlight_style(Style::default().fg(Color::LightBlue))
        .style(Style::default().fg(Color::White));

    let mut state = ListState::default();

    if !playlist_rows.is_empty() {
        state.select(Some(app.selected_library));
    }

    frame.render_stateful_widget(list, area, &mut state);
}

fn draw_song_selector(
    frame: &mut Frame,
    app: &App,
    area: Rect,
    path: &std::path::Path,
) {
    let area = center_area(area, CONTENT_MAX_WIDTH);

    let song_ids = app.current_playing_song_ids();

    let items: Vec<ListItem> = song_ids
        .iter()
        .map(|song_id| {
            let title = app
                .library
                .index
                .get(*song_id)
                .map(|song| song.title.as_str())
                .unwrap_or("Unknown");

            let is_current = app.playback.current_song == Some(*song_id);

            let item = ListItem::new(Line::from(format!("♪ {title}")));

            if is_current {
                item.style(Style::default().fg(Color::LightBlue))
            } else {
                item
            }
        })
        .collect();

    let title = format!(
        "Playlist - {}",
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Unknown")
    );

    let list = List::new(items)
        .block(Block::default().title(title))
        .highlight_symbol("▶ ")
        .highlight_style(Style::default().fg(Color::LightBlue))
        .style(Style::default().fg(Color::White));

    let mut state = ListState::default();

    if !song_ids.is_empty() {
        state.select(Some(app.selected_playlist));
    }

    frame.render_stateful_widget(list, area, &mut state);
}
