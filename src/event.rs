use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key(key: KeyEvent) -> Result<()> {
    println!("EVENT: {:?}", key.code);
    Ok(())
}
