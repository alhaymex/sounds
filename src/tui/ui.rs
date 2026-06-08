use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    text::Line,
};

use crate::app::App;
use crate::tui::theme::{key_inline, muted};
use crate::tui::{library::draw_library, player::draw_player};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let top_margin = 3.min(frame.area().height / 4);

    let [_, body, player, footer] = Layout::vertical([
        Constraint::Length(top_margin),
        Constraint::Fill(1),
        Constraint::Length(3),
        Constraint::Length(2),
    ])
    .areas(frame.area());

    draw_library(frame, app, body);
    draw_player(frame, app, player);
    draw_footer(frame, app, footer);
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
