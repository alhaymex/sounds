use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState},
};

use crate::{
    app::App,
    tui::theme::{CONTENT_MAX_WIDTH, center_area},
};

pub fn draw_favorites(frame: &mut Frame, app: &mut App, area: Rect) {
    let area = center_area(area, CONTENT_MAX_WIDTH);

    let favorite_songs = app.favorite_songs();

    let items: Vec<ListItem> = favorite_songs
        .iter()
        .map(|song| {
            let line = Line::from(vec![
                Span::styled("♥ ", Style::default().fg(Color::Yellow)),
                Span::raw(song.title.clone()),
            ]);
            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default().title(" Favorites ").style(
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .highlight_symbol("▶ ")
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().fg(Color::White));

    let mut state = ListState::default();
    state.select(Some(app.favorites_selected));

    frame.render_stateful_widget(list, area, &mut state);
}
