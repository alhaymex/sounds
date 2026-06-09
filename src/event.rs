use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, PlaybackStatus, Screen};
use crate::audio::player::AudioPlayer;

pub fn handle_key(
    key: KeyEvent,
    app: &mut App,
    audio_player: &mut AudioPlayer,
) -> Result<()> {
    if key.modifiers.contains(KeyModifiers::CONTROL)
        && matches!(key.code, KeyCode::Char('c'))
    {
        app.quit();
        return Ok(());
    }

    match key.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Down | KeyCode::Char('j') => app.move_down(),
        KeyCode::Up | KeyCode::Char('k') => app.move_up(),
        KeyCode::Backspace | KeyCode::Esc => app.back(),

        KeyCode::Enter => {
            let playlist_screen = matches!(app.screen, Screen::Playlist { .. });

            app.enter();

            if playlist_screen {
                if let Some(path) = app.selected_song_path() {
                    audio_player.play(path)?;
                    app.mark_selected_song_playing();
                }
            }
        }

        KeyCode::Char(' ') => {
            if app.playback.status != PlaybackStatus::Stopped {
                audio_player.toggle_pause();
                app.toggle_pause();
            }
        }

        KeyCode::Char('s') => {
            audio_player.stop();
            app.stop_playback();
        }

        _ => {}
    }

    Ok(())
}
