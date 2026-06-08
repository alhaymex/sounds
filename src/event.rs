use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

pub fn handle_key(key: KeyEvent, app: &mut App) -> Result<()> {
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
        KeyCode::Enter => app.enter(),
        _ => {}
    }

    Ok(())
}
