use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::ListItem,
};

use crate::tui::theme::{key_inline, muted};
use crate::tui::{library::draw_library, player::draw_player};
use crate::{
    app::App,
    library::model::{Node, SongRef},
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let [body, player, footer] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(3),
        Constraint::Length(2),
    ])
    .areas(frame.area());

    draw_library(frame, app, body);
    draw_player(frame, app, player);
    draw_footer(frame, app, footer);
}

fn push_node_items(
    node: &Node,
    index: &[SongRef],
    depth: usize,
    items: &mut Vec<ListItem<'static>>,
) {
    let indent = " ".repeat(depth);

    match node {
        Node::Dir { name, children, .. } => {
            let line = Line::from(format!("{indent}📁 {name}/"));

            items.push(ListItem::new(line));

            for child in children {
                push_node_items(child, index, depth + 1, items);
            }
        }
        Node::Song { id } => {
            let title = index
                .get(*id)
                .map(|song| song.title.as_str())
                .unwrap_or("Unknown");

            let line = Line::from(format!("{indent}♪ {title}"));

            items.push(ListItem::new(line));
        }
    }
}

fn draw_footer(frame: &mut Frame, _app: &mut App, layout: Rect) {
    let line = Line::from(vec![
        key_inline("[j/k]"),
        muted(" move  "),
        key_inline("[⏎]"),
        muted(" play  "),
        key_inline("[h]"),
        muted(" back  "),
        key_inline("[f]"),
        muted(" search  "),
        key_inline("[y]"),
        muted(" youtube  "),
        key_inline("[?]"),
        muted(" help  "),
        key_inline("[q]"),
        muted(" quit"),
    ]);

    frame.render_widget(line.centered(), layout);
}
