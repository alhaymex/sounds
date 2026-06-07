use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

use crate::app::App;
use crate::tui::theme::muted;

pub fn draw_player(frame: &mut Frame, app: &App, area: Rect) {
    let [info_area, bar_area] =
        Layout::vertical([Constraint::Length(2), Constraint::Length(1)])
            .areas(area);

    draw_player_info(frame, app, info_area);
    draw_progress_bar(frame, app, bar_area);
}

fn draw_player_info(frame: &mut Frame, _app: &App, area: Rect) {
    let [title_area, meta_area] =
        Layout::vertical([Constraint::Length(1), Constraint::Length(1)])
            .areas(area);

    let title = Line::from(vec![
        Span::styled("▶ ", Style::default().fg(Color::Gray)),
        Span::styled(
            "One Piece - The Very Very Very Strongest",
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ]);

    frame.render_widget(title.centered(), title_area);

    let metadata = Line::from(vec![
        muted("Hiroshi Kitadani"),
        muted(" • "),
        muted("One Piece OST"),
        muted(" • "),
        muted("03/24"),
    ]);

    frame.render_widget(metadata.centered(), meta_area);
}

fn draw_progress_bar(frame: &mut Frame, _app: &App, area: Rect) {
    let max_width = 80u16;

    let bar_width = area.width.min(max_width);

    let x = area.x + (area.width.saturating_sub(bar_width)) / 2;

    let bar_area = Rect {
        x,
        y: area.y,
        width: bar_width,
        height: 1,
    };

    let current = "1:33";
    let total = "4:00";

    let reserved = (current.len() + total.len() + 2) as u16;

    let progress_width = bar_area.width.saturating_sub(reserved);

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
