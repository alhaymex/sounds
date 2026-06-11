use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Padding, Paragraph},
};

use crate::{
    app::ConfirmAction,
    state::input::{InputTarget, TextInputState},
};

pub fn draw_input_overlay(
    frame: &mut Frame,
    input: &TextInputState,
    area: Rect,
) {
    let title = match &input.target {
        InputTarget::Rename { .. } => " Rename ",
        InputTarget::NewPlaylist { .. } => " New Playlist ",
        InputTarget::Search => " Search ",
        InputTarget::LibraryPath => return,
    };

    let popup_area = centered_popup(area, 60, 5);

    let content = Line::from(vec![
        Span::raw(&input.value),
        Span::styled(
            "█",
            Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::RAPID_BLINK),
        ),
    ]);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1))
        .border_style(
            Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::BOLD),
        );

    let paragraph = Paragraph::new(content).block(block);

    frame.render_widget(Clear, popup_area);
    frame.render_widget(paragraph, popup_area);
}

pub fn draw_confirm_overlay(
    frame: &mut Frame,
    confirm: &ConfirmAction,
    area: Rect,
) {
    let message = match confirm {
        ConfirmAction::Delete { name, .. } => {
            vec![
                Line::from(vec![
                    Span::styled(
                        "⚠  ",
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(format!("Delete \"{}\"?", name)),
                ]),
                Line::default(),
                Line::from(vec![
                    Span::styled(
                        "[Y]",
                        Style::default()
                            .fg(Color::LightRed)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" Confirm    "),
                    Span::styled("[N]", Style::default().fg(Color::DarkGray)),
                    Span::raw(" Cancel"),
                ]),
            ]
        }
    };

    let popup_area = centered_popup(area, 65, 6);

    let block = Block::default()
        .title(" Confirm Action ")
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1))
        .border_style(
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        );

    let paragraph = Paragraph::new(message).block(block);

    frame.render_widget(Clear, popup_area);
    frame.render_widget(paragraph, popup_area);
}

// TODO: draw_error_overlay

fn centered_popup(area: Rect, width: u16, height: u16) -> Rect {
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width.min(area.width), height.min(area.height))
}
