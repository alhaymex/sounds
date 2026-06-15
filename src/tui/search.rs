use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph},
};

use crate::{
    app::{App, Screen},
    tui::theme::{CONTENT_MAX_WIDTH, center_area},
};

pub fn draw_search(frame: &mut Frame, app: &mut App, area: Rect) {
    let area = center_area(area, CONTENT_MAX_WIDTH);

    let (query, selected) = match &app.screen {
        Screen::Search { query, selected } => (query.clone(), *selected),
        _ => return,
    };

    let [input_area, results_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
    ])
    .areas(area);

    let input_block = Block::default()
        .title(" Search ")
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::BOLD),
        )
        .padding(Padding::horizontal(1));

    let input_text = Line::from(vec![
        Span::raw(&query),
        Span::styled(
            "▌",
            Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::RAPID_BLINK),
        ),
    ]);

    let input_paragraph = Paragraph::new(input_text).block(input_block);
    frame.render_widget(input_paragraph, input_area);

    let results = app.search_results(&query);

    let items: Vec<ListItem> = results
        .iter()
        .map(|song| {
            let is_current = app.playback.current_song == Some(song.id);
            let is_favorite = app.favorites.songs.contains(&song.path);

            let symbol = if is_favorite { "♥" } else { "♪" };

            let line = if is_current {
                Line::styled(
                    format!("{} {}", symbol, song.title),
                    Style::default().fg(Color::LightBlue),
                )
            } else {
                Line::from(vec![
                    Span::styled(
                        symbol,
                        Style::default().fg(if is_favorite {
                            Color::Yellow
                        } else {
                            Color::White
                        }),
                    ),
                    Span::raw(" "),
                    Span::raw(song.title.clone()),
                ])
            };

            ListItem::new(line)
        })
        .collect();

    let title = if query.is_empty() {
        " Results ".to_string()
    } else {
        format!(" Results: {} ", results.len())
    };

    let list = List::new(items)
        .block(
            Block::default()
                .title(title)
                .style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .highlight_symbol("▶ ")
        .highlight_style(
            Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().fg(Color::White));

    let selected = selected.min(results.len().saturating_sub(1));
    let mut state = ListState::default();
    if !results.is_empty() {
        state.select(Some(selected));
    }

    frame.render_stateful_widget(list, results_area, &mut state);
}
