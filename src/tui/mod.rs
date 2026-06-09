pub mod help;
pub mod library;
pub mod options;
pub mod player;
pub mod theme;
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

use crate::app::App;
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

        app.sync_playback_time(
            audio_player.position(),
            audio_player.duration(),
        );

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
