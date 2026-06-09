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
        KeyCode::Down | KeyCode::Char('j') => match app.screen {
            Screen::Library | Screen::Playlist { .. } => {
                app.move_down();
            }
            Screen::Options => {
                app.options_down();
            }
            Screen::Help => {}
        },
        KeyCode::Up | KeyCode::Char('k') => match app.screen {
            Screen::Library | Screen::Playlist { .. } => {
                app.move_up();
            }
            Screen::Options => {
                app.options_up();
            }
            Screen::Help => {}
        },
        KeyCode::Backspace | KeyCode::Esc => app.back(),

        KeyCode::Char('?') => app.toggle_help(),

        KeyCode::Char('o') => app.open_options(),

        KeyCode::Enter => match app.screen {
            Screen::Options => {
                app.activate_selected_option()?;
            }

            Screen::Playlist { .. } => {
                app.enter();
                play_selected_song(app, audio_player)?;
            }

            Screen::Library => {
                app.enter();
            }

            Screen::Help => {}
        },

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
        KeyCode::Right | KeyCode::Char('l') => {
            audio_player.seek_forward()?;
            app.sync_playback_time(
                audio_player.position(),
                audio_player.duration(),
            );
        }
        KeyCode::Left | KeyCode::Char('h') => {
            audio_player.seek_backward()?;
            app.sync_playback_time(
                audio_player.position(),
                audio_player.duration(),
            );
        }
        KeyCode::Char('+') => {
            let volume = app.volume_up();
            audio_player.set_volume(volume);
        }
        KeyCode::Char('-') => {
            let volume = app.volume_down();
            audio_player.set_volume(volume);
        }

        _ => {}
    }

    Ok(())
}

fn play_selected_song(
    app: &mut App,
    audio_player: &mut AudioPlayer,
) -> Result<()> {
    let Some(path) = app.selected_song_path().map(|path| path.to_path_buf())
    else {
        return Ok(());
    };

    audio_player.play(&path)?;
    app.mark_selected_song_playing();
    app.sync_playback_time(audio_player.position(), audio_player.duration());

    Ok(())
}
