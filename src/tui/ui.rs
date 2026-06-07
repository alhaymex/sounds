use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};

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

fn draw_footer(frame: &mut Frame, app: &mut App, layout: Rect) {
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

fn draw_player(frame: &mut Frame, _app: &App, area: Rect) {
    let [title_area, bar_row] =
        Layout::vertical([Constraint::Length(1), Constraint::Length(1)])
            .areas(area);

    let title = Line::from(vec![
        Span::styled("▶ ", Style::default().fg(Color::Green)),
        Span::raw("One Piece - The Very Very Very Strongest"),
    ]);

    frame.render_widget(title.centered(), title_area);

    let max_width = 80;
    let bar_width = bar_row.width.min(max_width);

    let x = bar_row.x + (bar_row.width.saturating_sub(bar_width)) / 2;

    let bar_area = Rect {
        x,
        y: bar_row.y,
        width: bar_width,
        height: 1,
    };

    let current = "1:33";
    let total = "4:00";

    // Space occupied by timestamps and surrounding spaces:
    // "1:33 " + " 4:00"
    let reserved_width = (current.len() + total.len() + 2) as u16;

    let progress_width = bar_area.width.saturating_sub(reserved_width);

    let progress = seek_bar(93, 240, progress_width as usize);

    let line = Line::from(vec![
        Span::styled(format!("{current} "), Style::default().fg(Color::Gray)),
        Span::raw(progress),
        Span::styled(format!(" {total}"), Style::default().fg(Color::Gray)),
    ]);

    frame.render_widget(line, bar_area);
}

fn seek_bar(current: u64, total: u64, width: usize) -> String {
    if width == 0 {
        return String::new();
    }

    let pos = if total > 0 {
        ((current as f64 / total as f64) * (width - 1) as f64) as usize
    } else {
        0
    };

    let mut s = String::with_capacity(width);

    for i in 0..width {
        if i < pos {
            s.push('━');
        } else if i == pos {
            s.push('●');
        } else {
            s.push('─');
        }
    }

    s
}

fn key_inline(text: impl Into<String>) -> Span<'static> {
    Span::styled(text.into(), Style::default().fg(Color::LightBlue))
}

fn muted(text: impl Into<String>) -> Span<'static> {
    Span::styled(text.into(), Style::default().fg(Color::DarkGray))
}

fn gap() -> Span<'static> {
    Span::raw("  ")
}
