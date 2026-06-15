pub mod draw_toast;
pub mod favorites;
pub mod help;
pub mod library;
pub mod options;
pub mod overlay;
pub mod player;
pub mod search;
pub mod theme;
pub mod toast;
pub mod ui;

use std::io;
use std::time::Duration;

use anyhow::Result;

use crossterm::event::{self as ct_event, Event};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
    enable_raw_mode,
};

use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use crate::app::{App, Screen};
use crate::audio::player::AudioPlayer;
use crate::event;

struct TerminalGuard;

impl TerminalGuard {
    fn enter() -> Result<Self> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}

pub fn run(mut app: App) -> Result<()> {
    let _guard = TerminalGuard::enter()?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut audio_player = AudioPlayer::new()?;

    terminal.clear()?;

    while !app.should_quit {
        app.tick = app.tick.wrapping_add(1);
        app.tick_toasts();

        app.sync_speed(audio_player.speed_index());
        app.sync_playback_time(
            audio_player.position(),
            audio_player.duration(),
        );

        // auto play next song when curr finishes
        if audio_player.is_finished() {
            let next_id = match app.screen {
                Screen::Favorites { .. } => app.next_favorite_song_id(),
                Screen::Search { .. } => app.next_search_song_id(),
                _ => app.next_song_id(),
            };

            if let Some(next_id) = next_id {
                if let Some(path) = app.advance_to_song(next_id) {
                    let _ = audio_player.play(&path);
                    app.sync_playback_time(
                        audio_player.position(),
                        audio_player.duration(),
                    );
                }
            } else {
                audio_player.stop();
                app.stop_playback();
            }
        }

        terminal.draw(|frame| ui::draw(frame, &mut app))?;

        if ct_event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = ct_event::read()? {
                event::handle_key(key, &mut app, &mut audio_player)?;
            }
        }
    }

    audio_player.stop();

    Ok(())
}
