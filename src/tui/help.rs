use ratatui::{Frame, layout::Rect, text::Line, widgets::Paragraph};

use crate::{
    app::App,
    tui::theme::{CONTENT_MAX_WIDTH, center_area, key_inline, muted, title},
};

pub fn draw_help_screen(frame: &mut Frame, _app: &mut App, area: Rect) {
    let area = center_area(area, CONTENT_MAX_WIDTH);

    let lines = vec![
        Line::from(""),
        Line::from(vec![title("Navigation")]),
        Line::from(vec![key_inline("[j / ↓]  "), muted(" Move down")]),
        Line::from(vec![key_inline("[k / ↑]  "), muted(" Move up")]),
        Line::from(vec![
            key_inline("[⏎]      "),
            muted(" Enter / Open / Play"),
        ]),
        Line::from(vec![key_inline("[esc / ⌫]"), muted(" Go back")]),
        Line::from(""),
        Line::from(vec![title("Playback")]),
        Line::from(vec![key_inline("[space]  "), muted(" Play / Pause")]),
        Line::from(vec![key_inline("[s]      "), muted(" Stop playback")]),
        Line::from(vec![key_inline("[n]      "), muted(" Next Song")]),
        Line::from(vec![key_inline("[n]      "), muted(" Previous Song")]),
        Line::from(vec![key_inline("[h / ←]  "), muted(" Seek backward")]),
        Line::from(vec![key_inline("[l / →]  "), muted(" Seek forward")]),
        Line::from(vec![key_inline("[+]      "), muted(" Volume up")]),
        Line::from(vec![key_inline("[-]      "), muted(" Volume down")]),
        Line::from(""),
        Line::from(vec![title("Screens")]),
        Line::from(vec![key_inline("[o]      "), muted(" Open options")]),
        Line::from(vec![key_inline("[?]      "), muted(" Toggle help")]),
        Line::from(""),
        Line::from(vec![title("Application")]),
        Line::from(vec![key_inline("[q]      "), muted(" Quit")]),
    ];

    frame.render_widget(Paragraph::new(lines), area);
}
