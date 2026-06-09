use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

pub const CONTENT_MAX_WIDTH: u16 = 80;

pub fn title(text: &'static str) -> Span<'static> {
    Span::styled(text, Style::default().add_modifier(Modifier::BOLD))
}

pub fn key_inline(text: impl Into<String>) -> Span<'static> {
    Span::styled(text.into(), Style::default().fg(Color::LightBlue))
}

pub fn muted(text: impl Into<String>) -> Span<'static> {
    Span::styled(text.into(), Style::default().fg(Color::DarkGray))
}

pub fn center_area(area: Rect, max_width: u16) -> Rect {
    let width = area.width.min(max_width);

    Rect {
        x: area.x + (area.width.saturating_sub(width)) / 2,
        y: area.y,
        width: width,
        height: area.height,
    }
}

pub fn draw_rain_background(
    frame: &mut Frame,
    area: Rect,
    volume: u8,
    tick: u64,
) {
    let density = match volume {
        0..=20 => 10,
        21..=50 => 25,
        _ => 50,
    };

    let width = area.width as usize;
    let height = area.height as usize;

    if width == 0 || height == 0 {
        return;
    }

    let mut lines = Vec::with_capacity(height);

    for y in 0..height {
        let mut spans = Vec::with_capacity(width);

        for x in 0..width {
            let seed = hash_u64(x as u64);

            let active = seed % 100 < density;

            if !active {
                spans.push(Span::raw(" "));
                continue;
            }

            let speed = 1 + (seed % 3);

            // Slightly longer trails than before.
            let trail_len = 6 + (seed % 6) as usize;

            let offset = seed % height as u64;

            let head_y = ((tick / speed + offset) % height as u64) as usize;

            let distance = distance_behind_head(y, head_y, height);

            if distance == 0 {
                // Rain head.
                spans.push(Span::styled("│", Style::default().fg(Color::Gray)));
            } else if distance < trail_len {
                let glyphs = ["│", "╎", "╵", "·"];

                let ch = glyphs[((seed as usize) + distance) % glyphs.len()];

                let color = match distance {
                    1 => Color::Gray,
                    2 | 3 => Color::DarkGray,

                    // Extra dim tail.
                    _ => Color::Rgb(60, 60, 60),
                };

                spans.push(Span::styled(ch, Style::default().fg(color)));
            } else {
                spans.push(Span::raw(" "));
            }
        }

        lines.push(Line::from(spans));
    }

    frame.render_widget(Paragraph::new(lines), area);
}

fn distance_behind_head(y: usize, head_y: usize, height: usize) -> usize {
    if y <= head_y {
        head_y - y
    } else {
        height - (y - head_y)
    }
}

fn hash_u64(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9E37_79B9_7F4A_7C15);
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^ (value >> 31)
}
