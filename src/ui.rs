use crate::app::{App, AppState};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tui_big_text::{BigText, PixelSize};

pub fn render_main_ui(f: &mut Frame, app: &mut App) {
    let size = f.area();
    app.init_stars(size.width, size.height);
    render_starfield(f, app);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(size);

    let wave_offset = (app.frame_count / 5) % 3;
    let header_text = match wave_offset {
        0 => "~~ VALENTINE ~~",
        1 => "~* VALENTINE *~",
        _ => "*~ VALENTINE ~*",
    };

    let big_header = BigText::builder()
        .pixel_size(PixelSize::Quadrant)
        .style(Style::default().fg(Color::LightRed)
            .add_modifier(Modifier::BOLD))
        .lines(vec![header_text.into()])
        .alignment(Alignment::Center)
        .build();

    f.render_widget(big_header, chunks[0]);

    match app.state {
        AppState::Question => render_question(f, chunks[1], app),
        AppState::AcceptedYes => render_accepted_yes(f, chunks[1], app),
        AppState::AcceptedMaybe => render_accepted_maybe(f, chunks[1], app),
        AppState::TryingNo => render_trying_no(f, chunks[1], app),
    }

    let footer = Paragraph::new("Press 'Q' or 'Esc' to exit")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);

    f.render_widget(footer, chunks[2]);
}

fn render_starfield(f: &mut Frame, app: &App) {
    for star in &app.stars {
        if star.x < f.area().width && star.y < f.area().height {
            let char = match star.brightness {
                0..=3 => ".", 4..=6 => "*", 7..=8 => "+", _ => "o",
            };
            let color = match star.brightness {
                0..=3 => Color::DarkGray, 
                4..=6 => Color::Gray, 
                7..=8 => Color::White, 
                _ => Color::LightYellow,
            };
            f.render_widget(Paragraph::new(char)
                .style(Style::default()
                    .fg(color)), Rect::new(star.x, star.y, 1, 1));
        }
    }
}

fn render_question(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(13), 
            Constraint::Length(7), 
            Constraint::Min(1)
        ])
        .split(area);

    let scale = app.get_heart_beat_scale();
    let pulse = if scale > 1.1 { "**" } else { "  " };

    let heart = vec![
        Line::from(""),
        Line::from(vec![Span::styled(format!("  {}**     **    **     **{}  ", pulse, pulse), Style::default().fg(Color::Red))]),
        Line::from(vec![Span::styled("  **  ** **  ** **  ** **  **  ", Style::default().fg(Color::Red))]),
        Line::from(vec![Span::styled(" **    ***    ***    ***    ** ", Style::default().fg(Color::LightRed))]),
        Line::from(vec![Span::styled(" **                          ** ", Style::default().fg(Color::LightRed))]),
        Line::from(vec![Span::styled("  **                        **  ", Style::default().fg(Color::Magenta))]),
        Line::from(vec![Span::styled("    **                    **    ", Style::default().fg(Color::Magenta))]),
        Line::from(vec![Span::styled("      **                **      ", Style::default().fg(Color::LightMagenta))]),
        Line::from(vec![Span::styled("        **            **        ", Style::default().fg(Color::LightMagenta))]),
        Line::from(vec![Span::styled("          **        **          ", Style::default().fg(Color::LightMagenta))]),
        Line::from(vec![Span::styled("            **    **            ", Style::default().fg(Color::LightMagenta))]),
        Line::from(vec![Span::styled("              ****              ", Style::default().fg(Color::LightMagenta))]),
        Line::from(vec![Span::styled("               **               ", Style::default().fg(Color::LightMagenta))]),
    ];

    f.render_widget(
        Paragraph::new(heart).alignment(Alignment::Center),
        chunks[0],
    );

    let question = BigText::builder()
        .pixel_size(PixelSize::Quadrant)
        .style(Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD))
        .lines(vec!["Be Mine?".into()])
        .alignment(Alignment::Center)
        .build();

    f.render_widget(question, chunks[1]);

    let button_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(chunks[2]);

    let glow = if (app.frame_count / 15) % 2 == 0 {
        Modifier::BOLD | Modifier::RAPID_BLINK
    } else {
        Modifier::BOLD
    };

    f.render_widget(
        Paragraph::new(">>> YES! <<<\n  (Press Y)")
        .style(Style::default().fg(Color::Green).add_modifier(glow))
        .alignment(Alignment::Center)
        .block(
            Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Green)),
        ),
        button_chunks[0],
    );

    f.render_widget(
        Paragraph::new("  Maybe?\n (Press M)")
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(
            Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Yellow)),
        ),
        button_chunks[1],
    );

    f.render_widget(
        Paragraph::new("    No\n (Press N)")
        .style(Style::default().fg(Color::Red))
        .alignment(Alignment::Center)
        .block(
            Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Red)),
        ),
        button_chunks[2],
    );
}

fn render_accepted_yes(f: &mut Frame, area: Rect, app: &App) {
    let center_x = area.width as f32 / 2.0;
    let center_y = area.height as f32 / 2.0;

    for particle in &app.particles {
        let x = (center_x + particle.x) as u16;
        let y = (center_y + particle.y) as u16;
        if x < area.width && y < area.height {
            let p_widget = Paragraph::new(particle.char.to_string())
                .style(Style::default().fg(particle.color));
            let p_pos = Rect::new(area.x + x, area.y + y, 1, 1);
            f.render_widget(p_widget, p_pos);
        }
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Min(1)])
        .split(area);

    let yay_text = BigText::builder()
        .pixel_size(PixelSize::Quadrant)
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .lines(vec!["YAY!!!".into()])
        .alignment(Alignment::Center)
        .build();

    f.render_widget(yay_text, chunks[0]);

    let message = vec![
        Line::from(""),
        Line::from(Span::styled(
                "You've made me the happiest!",
                Style::default()
                .fg(Color::LightRed)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("   ~~~ Best Valentine's Day Ever! ~~~")
            .style(Style::default().fg(Color::Magenta)),
    ];

    f.render_widget(
        Paragraph::new(message).alignment(Alignment::Center),
        chunks[1],
    );
}

fn render_accepted_maybe(f: &mut Frame, area: Rect, app: &App) {
    let dot_offset = (app.frame_count / 8) % 6;
    let dots = match dot_offset {
        0 => ".     ",
        1 => " .    ",
        2 => "  .   ",
        3 => "   .  ",
        4 => "    . ",
        _ => "     .",
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(1)])
        .split(area);

    let maybe_text = BigText::builder()
        .pixel_size(PixelSize::Quadrant)
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .lines(vec!["MAYBE?".into()])
        .alignment(Alignment::Center)
        .build();

    f.render_widget(maybe_text, chunks[0]);

    let content = vec![
        Line::from(""),
        Line::from(format!("   {} Hmm, interesting {} ", dots, dots))
            .style(Style::default().fg(Color::Yellow)),
            Line::from(""),
            Line::from("   I'll take that as a 'work in progress'!")
                .style(Style::default().fg(Color::Cyan)),
                Line::from(""),
                Line::from("        Still holding out hope!")
                    .style(Style::default().fg(Color::LightMagenta)),
    ];

    f.render_widget(
        Paragraph::new(content).alignment(Alignment::Center),
        chunks[1],
    );
}

fn render_trying_no(f: &mut Frame, area: Rect, app: &App) {
    let messages = [
        "ERROR!", "NOPE!", "TRY AGAIN", "INVALID", "DENIED", "NO WAY",
        "BLOCKED", "FAILED", "REJECTED", "HAHA NO",
    ];
    let message = messages[app.attempt_count.min(9) as usize];

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(1),
            Constraint::Length(5),
        ])
        .split(area);

    let shake_offset = if (app.frame_count / 3) % 2 == 0 { 1 } else { 0 };

    let error_text = BigText::builder()
        .pixel_size(PixelSize::Quadrant)
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .lines(vec![message.into()])
        .alignment(Alignment::Center)
        .build();

    let error_area = Rect::new(
        chunks[0].x + shake_offset,
        chunks[0].y,
        chunks[0].width.saturating_sub(shake_offset),
        chunks[0].height,
    );
    f.render_widget(error_text, error_area);

    let info = vec![
        Line::from(""),
        Line::from(Span::styled(
                "The 'No' button is currently unavailable",
                Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("        Please select YES or MAYBE")
            .style(Style::default().fg(Color::Cyan)),
            Line::from(""),
            Line::from(format!("    Failed attempts: {}", app.attempt_count))
                .style(Style::default().fg(Color::DarkGray)),
    ];

    f.render_widget(
        Paragraph::new(info).alignment(Alignment::Center),
        chunks[1],
    );

    let button_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(chunks[2]);

    f.render_widget(
        Paragraph::new(">>> YES! <<<\n  (Press Y)")
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(
            Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Green)),
        ),
        button_chunks[0],
    );

    f.render_widget(
        Paragraph::new("  Maybe?\n (Press M)")
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(
            Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Yellow)),
        ),
        button_chunks[1],
    );

    let glitch_chars = ["X", "?", "#", "@", "!"];
    let glitch = glitch_chars[(app.frame_count / 5) as usize % 5];
    f.render_widget(
        Paragraph::new(format!(" [{}] ERROR\n [{}] {}", glitch, glitch, glitch))
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(
            Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::DarkGray)),
        ),
        button_chunks[2],
    );
}
