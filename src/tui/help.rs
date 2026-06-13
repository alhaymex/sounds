use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, List, ListItem, ListState, Padding},
};

use crate::{
    app::{App, Screen},
    tui::theme::{CONTENT_MAX_WIDTH, key_inline, muted, title},
};

pub enum HelpItem {
    Header(&'static str),
    Entry(Line<'static>),
    Spacer,
}

pub fn draw_help_screen(frame: &mut Frame, app: &mut App, area: Rect) {
    let area = Rect {
        x: area.x + (area.width.saturating_sub(CONTENT_MAX_WIDTH)) / 2,
        y: area.y,
        width: CONTENT_MAX_WIDTH,
        height: area.height,
    };

    let list_items: Vec<ListItem> = app
        .help_items
        .iter()
        .map(|item| match item {
            HelpItem::Header(text) => {
                ListItem::new(Line::from(vec![title(text)]))
            }
            HelpItem::Entry(line) => ListItem::new(line.clone()),
            HelpItem::Spacer => ListItem::new(Line::from("")),
        })
        .collect();

    let selected = match app.screen {
        Screen::Help { selected } => {
            selected.min(list_items.len().saturating_sub(1))
        }
        _ => 0,
    };

    let list = List::new(list_items)
        .block(
            Block::default()
                .title(" HELP ")
                .padding(Padding::horizontal(2)),
        )
        .highlight_style(Style::default())
        .highlight_symbol("▶ ");

    let mut state = ListState::default();
    let map = help_entry_map(app);

    let selected_list_index = map.get(selected).copied().unwrap_or(0);

    state.select(Some(selected_list_index));

    frame.render_stateful_widget(list, area, &mut state);
}

pub fn help_items() -> Vec<HelpItem> {
    vec![
        HelpItem::Spacer,
        HelpItem::Header("Navigation"),
        HelpItem::Entry(Line::from(vec![
            key_inline("[j / ↓]  "),
            muted(" Move down"),
        ])),
        HelpItem::Entry(Line::from(vec![
            key_inline("[k / ↑]  "),
            muted(" Move up"),
        ])),
        HelpItem::Entry(Line::from(vec![
            key_inline("[⏎]      "),
            muted(" Enter / Open / Play"),
        ])),
        HelpItem::Entry(Line::from(vec![
            key_inline("[esc / ⌫]"),
            muted(" Go back"),
        ])),
        HelpItem::Spacer,
        HelpItem::Header("Playback"),
        HelpItem::Entry(Line::from(vec![
            key_inline("[space]  "),
            muted(" Play / Pause"),
        ])),
        HelpItem::Entry(Line::from(vec![
            key_inline("[s]      "),
            muted(" Stop playback"),
        ])),
        HelpItem::Entry(Line::from(vec![
            key_inline("[n]      "),
            muted(" Next song"),
        ])),
        HelpItem::Entry(Line::from(vec![
            key_inline("[p]      "),
            muted(" Previous song"),
        ])),
        HelpItem::Entry(Line::from(vec![
            key_inline("[h / ←]  "),
            muted(" Seek backward"),
        ])),
        HelpItem::Entry(Line::from(vec![
            key_inline("[l / →]  "),
            muted(" Seek forward"),
        ])),
        HelpItem::Entry(Line::from(vec![
            key_inline("[+]      "),
            muted(" Volume up"),
        ])),
        HelpItem::Entry(Line::from(vec![
            key_inline("[-]      "),
            muted(" Volume down"),
        ])),
        HelpItem::Entry(Line::from(vec![
            key_inline("[>]      "),
            muted(" Increase speed"),
        ])),
        HelpItem::Entry(Line::from(vec![
            key_inline("[<]      "),
            muted(" Decrease speed"),
        ])),
        HelpItem::Spacer,
        HelpItem::Header("Screens"),
        HelpItem::Entry(Line::from(vec![
            key_inline("[o]      "),
            muted(" Open options"),
        ])),
        HelpItem::Entry(Line::from(vec![
            key_inline("[?]      "),
            muted(" Toggle help"),
        ])),
        HelpItem::Spacer,
        HelpItem::Header("Library Management"),
        HelpItem::Entry(Line::from(vec![
            key_inline("[r]      "),
            muted(" Rename song or playlist"),
        ])),
        HelpItem::Entry(Line::from(vec![
            key_inline("[a]      "),
            muted(" Create playlist"),
        ])),
        HelpItem::Entry(Line::from(vec![
            key_inline("[d]      "),
            muted(" Delete song or playlist"),
        ])),
        HelpItem::Spacer,
        HelpItem::Header("Application"),
        HelpItem::Entry(Line::from(vec![
            key_inline("[q]      "),
            muted(" Quit"),
        ])),
    ]
}

fn help_entry_map(app: &App) -> Vec<usize> {
    app.help_items
        .iter()
        .enumerate()
        .filter_map(|(i, item)| match item {
            HelpItem::Entry(_) => Some(i),
            _ => None,
        })
        .collect()
}
