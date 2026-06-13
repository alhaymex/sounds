use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, List, ListItem, ListState, Padding, Paragraph},
};

use crate::{
    app::{App, Screen},
    tui::theme::{CONTENT_MAX_WIDTH, center_area, key_inline, muted, title},
};

pub fn draw_help_screen(frame: &mut Frame, app: &mut App, area: Rect) {
    let area = center_area(area, CONTENT_MAX_WIDTH);

    let items = vec![
        ListItem::new(""),
        ListItem::new(Line::from(vec![title("Navigation")])),
        ListItem::new(Line::from(vec![
            key_inline("[j / ↓]  "),
            muted(" Move down"),
        ])),
        ListItem::new(Line::from(vec![
            key_inline("[k / ↑]  "),
            muted(" Move up"),
        ])),
        ListItem::new(Line::from(vec![
            key_inline("[⏎]      "),
            muted(" Enter / Open / Play"),
        ])),
        ListItem::new(Line::from(vec![
            key_inline("[esc / ⌫]"),
            muted(" Go back"),
        ])),
        ListItem::new(""),
        ListItem::new(Line::from(vec![title("Playback")])),
        ListItem::new(Line::from(vec![
            key_inline("[space]  "),
            muted(" Play / Pause"),
        ])),
        ListItem::new(Line::from(vec![
            key_inline("[s]      "),
            muted(" Stop playback"),
        ])),
        ListItem::new(Line::from(vec![
            key_inline("[n]      "),
            muted(" Next song"),
        ])),
        ListItem::new(Line::from(vec![
            key_inline("[p]      "),
            muted(" Previous song"),
        ])),
        ListItem::new(Line::from(vec![
            key_inline("[h / ←]  "),
            muted(" Seek backward"),
        ])),
        ListItem::new(Line::from(vec![
            key_inline("[l / →]  "),
            muted(" Seek forward"),
        ])),
        ListItem::new(Line::from(vec![
            key_inline("[+]      "),
            muted(" Volume up"),
        ])),
        ListItem::new(Line::from(vec![
            key_inline("[-]      "),
            muted(" Volume down"),
        ])),
        ListItem::new(Line::from(vec![
            key_inline("[>]      "),
            muted(" Faster playback"),
        ])),
        ListItem::new(Line::from(vec![
            key_inline("[<]      "),
            muted(" Slower playback"),
        ])),
        ListItem::new(""),
        ListItem::new(Line::from(vec![title("Screens")])),
        ListItem::new(Line::from(vec![
            key_inline("[o]      "),
            muted(" Open options"),
        ])),
        ListItem::new(Line::from(vec![
            key_inline("[?]      "),
            muted(" Toggle help"),
        ])),
        ListItem::new(""),
        ListItem::new(Line::from(vec![title("Library Management")])),
        ListItem::new(Line::from(vec![
            key_inline("[r]      "),
            muted(" Rename song or playlist"),
        ])),
        ListItem::new(Line::from(vec![
            key_inline("[a]      "),
            muted(" Create playlist"),
        ])),
        ListItem::new(Line::from(vec![
            key_inline("[d]      "),
            muted(" Delete song or playlist"),
        ])),
        ListItem::new(""),
        ListItem::new(Line::from(vec![title("Application")])),
        ListItem::new(Line::from(vec![
            key_inline("[q]      "),
            muted(" Quit"),
        ])),
    ];

    let list = List::new(items)
        .block(
            Block::default()
                .title(" HELP ")
                .padding(Padding::horizontal(2)),
        )
        .highlight_style(Style::default())
        .highlight_symbol("▶  ");

    let selected = match app.screen {
        Screen::Help { selected } => selected,
        _ => 0,
    };

    let mut state = ListState::default();
    state.select(Some(selected));

    frame.render_stateful_widget(list, area, &mut state);
}
