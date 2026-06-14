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

pub fn draw_favorites(frame: &mut Frame, _app: &mut App, area: Rect) {
    let area = center_area(area, CONTENT_MAX_WIDTH);

    let favorites = vec![
        "Nujabes - Feather",
        "Daft Punk - Voyager",
        "Tomppabeats - Monday Loop",
        "Idealism - Controlla",
        "Kanye West - POWER",
        "Eminem - Till I Collapse",
    ];

    let items: Vec<ListItem> = favorites
        .iter()
        .map(|title| {
            let line = Line::from(vec![
                Span::styled("♥ ", Style::default().fg(Color::Yellow)),
                Span::raw(title.to_string()),
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
    state.select(Some(0));

    frame.render_stateful_widget(list, area, &mut state);
}
