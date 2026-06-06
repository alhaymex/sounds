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
        _ => {}
    }

    Ok(())
}
