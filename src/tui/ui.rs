use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    text::Line,
};

use crate::tui::{
    draw_toast::draw_toasts,
    favorites::draw_favorites,
    help::draw_help_screen,
    library::draw_library,
    options::draw_options_screen,
    overlay::{draw_confirm_overlay, draw_input_overlay},
    player::draw_player,
};
use crate::{app::App, tui::theme::draw_rain_background};
use crate::{
    app::Screen,
    tui::theme::{key_inline, muted},
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let top_margin = 3.min(frame.area().height / 4);

    let [_, body, player, footer] = Layout::vertical([
        Constraint::Length(top_margin),
        Constraint::Fill(1),
        Constraint::Length(3),
        Constraint::Length(2),
    ])
    .areas(frame.area());

    match app.screen {
        Screen::Library { .. } => {
            if app.config.rain_enabled {
                draw_rain_background(
                    frame,
                    frame.area(),
                    app.playback.volume,
                    app.tick,
                );
            }
            draw_library(frame, app, body);
        }
        Screen::Options => draw_options_screen(frame, app, body),
        Screen::Help { .. } => draw_help_screen(frame, app, body),
        Screen::Favorites => draw_favorites(frame, app, body),
    }

    draw_player(frame, app, player);
    draw_footer(frame, app, footer);

    if let Some(input) = &app.input {
        draw_input_overlay(frame, input, frame.area());
    }

    if let Some(confirm) = &app.confirm {
        draw_confirm_overlay(frame, confirm, frame.area());
    }

    draw_toasts(frame, app);
}

fn draw_footer(frame: &mut Frame, _app: &mut App, layout: Rect) {
    let line = Line::from(vec![
        key_inline("[j/k]"),
        muted(" move  "),
        key_inline("[⏎]"),
        muted(" play  "),
        key_inline("[esc]"),
        muted(" back  "),
        key_inline("[f]"),
        muted(" search  "),
        key_inline("[o]"),
        muted(" options  "),
        key_inline("[?]"),
        muted(" help  "),
        key_inline("[q]"),
        muted(" quit"),
    ]);

    frame.render_widget(line.centered(), layout);
}
