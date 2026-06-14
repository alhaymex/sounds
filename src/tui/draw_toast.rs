use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

use crate::{app::App, tui::toast::ToastKind};

pub fn draw_toasts(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    let toast_width = 40u16;
    let toast_height = 3u16;
    let margin = 0u16;

    for (i, toast) in app.toasts.iter().rev().take(3).enumerate() {
        let x = area.width.saturating_sub(toast_width + margin);
        let y = area
            .height
            .saturating_sub((toast_height + margin) * (i as u16 + 1));

        let toast_area = Rect::new(x, y, toast_width, toast_height);

        let (icon, color) = match toast.kind {
            ToastKind::Success => (" ✓", Color::Green),
            ToastKind::Error => (" ✗", Color::Red),
            ToastKind::Info => (" i", Color::Blue),
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(color));

        let inner = block.inner(toast_area);

        let line = Line::from(vec![
            Span::styled(
                format!("{icon} "),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                toast.message.as_str(),
                Style::default().fg(Color::White),
            ),
        ]);

        frame.render_widget(Clear, toast_area);
        frame.render_widget(block, toast_area);
        frame.render_widget(Paragraph::new(line), inner);
    }
}
