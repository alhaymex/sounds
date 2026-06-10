use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, List, ListItem, ListState, Padding},
};

use crate::{
    app::App,
    state::input::InputTarget,
    tui::theme::{CONTENT_MAX_WIDTH, center_area},
};

pub fn draw_options_screen(frame: &mut Frame, app: &mut App, area: Rect) {
    let area = center_area(area, CONTENT_MAX_WIDTH);

    let library_path = match &app.input {
        Some(input) if input.target == InputTarget::LibraryPath => {
            format!("{}▌", input.value)
        }
        _ => app.library.root.display().to_string(),
    };

    let rain = if app.config.rain_enabled {
        "[x] Enabled"
    } else {
        "[ ] Disabled"
    };

    let items = vec![
        ListItem::new(Line::from(format!(
            "{:<16}{}",
            "Library Path", library_path
        ))),
        ListItem::new(Line::from(format!("{:<16}{}", "Rain Effect", rain))),
    ];

    let list = List::new(items)
        .block(
            Block::default()
                .title(" OPTIONS ")
                .style(Style::default().fg(Color::White))
                .padding(Padding::horizontal(2)),
        )
        .highlight_symbol("▶ ")
        .highlight_style(
            Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().fg(Color::White));

    let mut state = ListState::default();
    state.select(Some(app.options.selected()));

    frame.render_stateful_widget(list, area, &mut state);
}
