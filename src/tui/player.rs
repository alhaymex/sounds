use std::time::Duration;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

use crate::tui::theme::{CONTENT_MAX_WIDTH, center_area, muted};
use crate::{
    app::{App, PlaybackStatus},
    audio::player::SPEEDS,
};

pub fn draw_player(frame: &mut Frame, app: &App, area: Rect) {
    let [info_area, bar_area] =
        Layout::vertical([Constraint::Length(2), Constraint::Length(1)])
            .areas(area);

    draw_player_info(frame, app, info_area);
    draw_progress_bar(frame, app, bar_area);
}

fn draw_player_info(frame: &mut Frame, app: &App, area: Rect) {
    let [title_area, meta_area] =
        Layout::vertical([Constraint::Length(1), Constraint::Length(1)])
            .areas(area);

    let current_song = app
        .playback
        .current_song
        .and_then(|song_id| app.library.index.get(song_id));

    let title_text = current_song
        .map(|song| song.title.as_str())
        .unwrap_or("No song playing");

    let icon = match app.playback.status {
        PlaybackStatus::Playing => "⏸ ",
        PlaybackStatus::Paused => "▶ ",
        PlaybackStatus::Stopped => "■ ",
    };

    let icon_color = match app.playback.status {
        PlaybackStatus::Playing => Color::Yellow,
        PlaybackStatus::Paused => Color::LightBlue,
        PlaybackStatus::Stopped => Color::Gray,
    };

    let title = Line::from(vec![
        Span::styled(icon, Style::default().fg(icon_color)),
        Span::styled(
            title_text,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ]);

    frame.render_widget(title.centered(), title_area);

    let metadata = if let Some(song) = current_song {
        let artist = song.metadata.artist.as_deref();
        let album = song.metadata.album.as_deref();
        let speed = SPEEDS[app.playback.speed_index];
        let speed_span = if (speed - 1.0).abs() < f32::EPSILON {
            muted(format!("{}×", speed))
        } else {
            Span::styled(
                format!("{}×", speed),
                Style::default().fg(Color::Yellow),
            )
        };

        let mut spans: Vec<Span> = Vec::new();

        match (artist, album) {
            (Some(a), Some(b)) => {
                spans.push(muted(a));
                spans.push(muted(" • "));
                spans.push(muted(b));
                spans.push(muted(" • "));
            }
            (Some(a), None) => {
                spans.push(muted(a));
                spans.push(muted(" • "));
            }
            (None, Some(b)) => {
                spans.push(muted(b));
                spans.push(muted(" • "));
            }
            (None, None) => {
                spans.push(muted("No metadata"));
                spans.push(muted(" • "));
            }
        }

        spans.push(muted(format!("Volume {}%", app.playback.volume)));
        spans.push(muted(" • "));
        spans.push(speed_span);

        Line::from(spans)
    } else {
        let speed = SPEEDS[app.playback.speed_index];
        let speed_span = if (speed - 1.0).abs() < f32::EPSILON {
            muted(format!("{}×", speed))
        } else {
            Span::styled(
                format!("{}×", speed),
                Style::default().fg(Color::Yellow),
            )
        };
        Line::from(vec![
            muted("No metadata"),
            muted(" • "),
            muted(format!("Volume {}%", app.playback.volume)),
            muted(" • "),
            speed_span,
        ])
    };

    frame.render_widget(metadata.centered(), meta_area);
}

fn draw_progress_bar(frame: &mut Frame, app: &App, area: Rect) {
    let area = center_area(area, CONTENT_MAX_WIDTH);

    let position = app.playback.position;
    let duration = app.playback.duration.unwrap_or(Duration::ZERO);

    let current = format_duration(position);
    let total = if duration.is_zero() {
        "--:--".to_string()
    } else {
        format_duration(duration)
    };

    let reserved = (current.len() + total.len() + 2) as u16;
    let progress_width = area.width.saturating_sub(reserved);

    let progress = seek_bar(
        position.as_secs(),
        duration.as_secs(),
        progress_width as usize,
    );

    let line = Line::from(vec![
        Span::styled(format!("{current} "), Style::default().fg(Color::Gray)),
        Span::raw(progress).style(Style::default().fg(Color::White)),
        Span::styled(format!(" {total}"), Style::default().fg(Color::Gray)),
    ]);

    frame.render_widget(line, area);
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

fn format_duration(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let minutes = total_secs / 60;
    let seconds = total_secs % 60;

    format!("{minutes}:{seconds:02}")
}
