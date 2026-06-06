use ratatui::{
    Frame,
    text::Line,
    widgets::{List, ListItem},
};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let items: Vec<ListItem> = app
        .library
        .index
        .iter()
        .map(|song| ListItem::new(Line::from(song.title.clone())))
        .collect();

    let title = format!(
        "Library - {} songs - {}",
        app.library.index.len(),
        app.library.root.display()
    );

    let list = List::new(items);

    frame.render_widget(list, frame.area());
}
