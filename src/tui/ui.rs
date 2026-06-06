use ratatui::{
    Frame,
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem},
};

use crate::{
    app::App,
    library::model::{Node, SongRef},
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let items = match app.library.tree.as_ref() {
        Some(root) => {
            let mut items = Vec::new();
            push_node_items(root, &app.library.index, 0, &mut items);
            items
        }
        None => vec![ListItem::new(Line::from("None"))],
    };

    let title = format!(
        "Library - {} songs - {}",
        app.library.index.len(),
        app.library.root.display()
    );

    let list = List::new(items)
        .block(Block::default().title(title).borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    frame.render_widget(list, frame.area());
}

fn push_node_items(
    node: &Node,
    index: &[SongRef],
    depth: usize,
    items: &mut Vec<ListItem<'static>>,
) {
    let indent = " ".repeat(depth);

    match node {
        Node::Dir { name, children, .. } => {
            let line = Line::from(format!("{indent}📁 {name}/"));

            items.push(ListItem::new(line));

            for child in children {
                push_node_items(child, index, depth + 1, items);
            }
        }
        Node::Song { id } => {
            let title = index
                .get(*id)
                .map(|song| song.title.as_str())
                .unwrap_or("Unknown");

            let line = Line::from(format!("{indent}♪ {title}"));

            items.push(ListItem::new(line));
        }
    }
}
