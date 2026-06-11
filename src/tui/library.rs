use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, List, ListItem, ListState},
};

use crate::{
    app::{App, DirEntry, Screen},
    tui::theme::{CONTENT_MAX_WIDTH, center_area},
};

pub fn draw_library(frame: &mut Frame, app: &mut App, area: Rect) {
    let Screen::Library { ref path, selected } = app.screen else {
        return;
    };

    let area = center_area(area, CONTENT_MAX_WIDTH);
    let path = path.clone();
    let entries = app.dir_entries(&path);

    let items: Vec<ListItem> = entries
        .iter()
        .map(|entry| match entry {
            DirEntry::Dir {
                name, song_count, ..
            } => ListItem::new(Line::from(format!(
                "📁 {} ({})",
                name, song_count
            ))),
            DirEntry::Song { id, title } => {
                let is_current = app.playback.current_song == Some(*id);
                let item = ListItem::new(Line::from(format!("♪ {}", title)));
                if is_current {
                    item.style(Style::default().fg(Color::LightBlue))
                } else {
                    item
                }
            }
        })
        .collect();

    let dir_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Library");

    let list = List::new(items)
        .block(
            Block::default()
                .title(format!(" {} ", dir_name))
                .style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .highlight_symbol("▶ ")
        .highlight_style(Style::default().fg(Color::LightBlue))
        .style(Style::default().fg(Color::White));

    let mut state = ListState::default();
    if !entries.is_empty() {
        state.select(Some(selected));
    }

    frame.render_stateful_widget(list, area, &mut state);
}
