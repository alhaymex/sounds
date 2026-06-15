use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::app::{App, ConfirmAction, PlaybackStatus, Screen};
use crate::audio::player::AudioPlayer;
use crate::state::input::InputTarget;
use crate::tui::toast::{Toast, ToastKind};

pub fn handle_key(
    key: KeyEvent,
    app: &mut App,
    audio_player: &mut AudioPlayer,
) -> Result<()> {
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        match key.code {
            KeyCode::Char('c') => {
                app.quit();
                return Ok(());
            }
            KeyCode::Char('r') => {
                let scanning_id = app.push_toast(Toast::persistent(
                    "Scanning library…",
                    ToastKind::Info,
                ));
                app.rescan_library()?;
                app.dismiss_toast(scanning_id);
                app.push_toast(Toast::success("Library scanned"));
                return Ok(());
            }
            KeyCode::Char('e') => {
                if matches!(app.screen, Screen::Library { .. }) {
                    app.start_rename();
                }
            }
            KeyCode::Char('d') => {
                if matches!(app.screen, Screen::Library { .. }) {
                    app.start_delete();
                }
            }
            KeyCode::Char('n') => {
                if matches!(app.screen, Screen::Library { .. }) {
                    app.start_new_playlist();
                }
            }
            KeyCode::Char('f') => {
                if matches!(
                    app.screen,
                    Screen::Library { .. }
                        | Screen::Favorites { .. }
                        | Screen::Search { .. }
                ) {
                    app.add_to_favorites()?;
                }
            }
            _ => {}
        }
    }

    if key.kind != KeyEventKind::Press {
        return Ok(());
    }

    if key.modifiers.contains(KeyModifiers::CONTROL) {
        return Ok(());
    };

    if app.is_confirming() {
        match key.code {
            KeyCode::Char('y') | KeyCode::Enter => {
                if let Some(confirm) = app.confirm.take() {
                    match confirm {
                        ConfirmAction::Delete { path, .. } => {
                            if app
                                .playback
                                .current_song
                                .and_then(|id| app.library.index.get(id))
                                .map(|song| song.path == path)
                                .unwrap_or(false)
                            {
                                audio_player.stop();
                                app.stop_playback();
                            }
                            app.delete_entry(&path)?;
                        }
                    }
                }
            }
            _ => app.cancel_confirm(),
        }
        return Ok(());
    }

    if app.is_input_active() {
        let is_search = app
            .input
            .as_ref()
            .is_some_and(|i| i.target == InputTarget::Search);

        if is_search {
            match key.code {
                KeyCode::Enter => {
                    if let Some(path) = app.play_selected_search_result() {
                        audio_player.play(&path)?;
                        app.sync_speed(audio_player.speed_index());
                        app.sync_playback_time(
                            audio_player.position(),
                            audio_player.duration(),
                        );
                    }
                    return Ok(());
                }
                KeyCode::Esc => {
                    if app
                        .input
                        .as_ref()
                        .map(|i| i.value.is_empty())
                        .unwrap_or(true)
                    {
                        app.cancel_input();
                        app.back();
                    } else {
                        app.clear_search_input();
                    }
                    return Ok(());
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    app.search_up();
                    return Ok(());
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    app.search_down();
                    return Ok(());
                }
                KeyCode::Backspace => {
                    app.search_input_backspace();
                    return Ok(());
                }
                KeyCode::Char(ch) => {
                    app.search_input_char(ch);
                    return Ok(());
                }
                _ => {}
            }
        } else {
            match key.code {
                KeyCode::Enter => app.submit_input()?,
                KeyCode::Esc => app.cancel_input(),
                KeyCode::Backspace => app.input_backspace(),
                KeyCode::Char(ch) => app.input_char(ch),
                _ => {}
            }

            return Ok(());
        }
    }

    match key.code {
        KeyCode::Char('q') => app.quit(),

        KeyCode::Down | KeyCode::Char('j') => match app.screen {
            Screen::Library { .. } => {
                app.move_down();
            }
            Screen::Options => {
                app.options_down();
            }
            Screen::Help { .. } => app.help_down(),
            Screen::Favorites { .. } => app.favorites_down(),
            Screen::Search { .. } => app.search_down(),
        },
        KeyCode::Up | KeyCode::Char('k') => match app.screen {
            Screen::Library { .. } => {
                app.move_up();
            }
            Screen::Options => {
                app.options_up();
            }
            Screen::Help { .. } => app.help_up(),
            Screen::Favorites { .. } => app.favorites_up(),
            Screen::Search { .. } => app.search_up(),
        },
        KeyCode::Backspace | KeyCode::Esc => app.back(),
        KeyCode::Char('?') => app.toggle_help(),
        KeyCode::Char('o') => app.open_options(),
        KeyCode::Char('f') => app.open_favorites(),
        KeyCode::Char('/') => app.start_search(),

        KeyCode::Enter => match app.screen {
            Screen::Options => {
                app.activate_selected_option()?;
            }

            Screen::Library { .. } => {
                if app.selected_song_id().is_some() {
                    play_selected_song(app, audio_player)?;
                } else {
                    app.enter();
                }
            }

            Screen::Favorites { .. } => {
                if let Some(path) = app.play_selected_favorite() {
                    audio_player.play(&path)?;
                    app.sync_speed(audio_player.speed_index());
                    app.sync_playback_time(
                        audio_player.position(),
                        audio_player.duration(),
                    );
                }
            }
            Screen::Search { .. } => {
                if let Some(path) = app.play_selected_search_result() {
                    audio_player.play(&path)?;
                    app.sync_speed(audio_player.speed_index());
                    app.sync_playback_time(
                        audio_player.position(),
                        audio_player.duration(),
                    );
                }
            }
            Screen::Help { .. } => {}
        },

        // we can add playback keys too
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
        KeyCode::Char('n') => {
            let next_id = match app.screen {
                Screen::Favorites { .. } => app.next_favorite_song_id(),
                Screen::Search { .. } => app.next_search_song_id(),
                _ => app.next_song_id(),
            };

            if let Some(next_id) = next_id {
                if let Some(path) = app.advance_to_song(next_id) {
                    audio_player.play(&path)?;
                    app.sync_speed(audio_player.speed_index());
                    app.sync_playback_time(
                        audio_player.position(),
                        audio_player.duration(),
                    );
                }
            }
        }
        KeyCode::Char('p') => {
            let prev_id = match app.screen {
                Screen::Favorites { .. } => app.prev_favorite_song_id(),
                Screen::Search { .. } => app.prev_search_song_id(),
                _ => app.prev_song_id(),
            };

            if let Some(prev_id) = prev_id {
                if let Some(path) = app.advance_to_song(prev_id) {
                    audio_player.play(&path)?;
                    app.sync_speed(audio_player.speed_index());
                    app.sync_playback_time(
                        audio_player.position(),
                        audio_player.duration(),
                    );
                }
            }
        }
        KeyCode::Char('>') => {
            audio_player.increase_speed();
            app.playback.speed_index = audio_player.speed_index();
        }
        KeyCode::Char('<') => {
            audio_player.decrease_speed();
            app.playback.speed_index = audio_player.speed_index();
        }

        KeyCode::Right | KeyCode::Char('l') => {
            audio_player.seek_forward()?;
            app.sync_speed(audio_player.speed_index());
            app.sync_playback_time(
                audio_player.position(),
                audio_player.duration(),
            );
        }
        KeyCode::Left | KeyCode::Char('h') => {
            audio_player.seek_backward()?;
            app.sync_speed(audio_player.speed_index());
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
    app.sync_speed(audio_player.speed_index());
    app.sync_playback_time(audio_player.position(), audio_player.duration());

    Ok(())
}
