use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    layout::Alignment,
    prelude::{CrosstermBackend, Stylize, Terminal},
    style::{Color, Style, Styled},
    text::Text,
    widgets::{Block, Borders, Paragraph, Wrap},
};
use std::{
    fmt::format,
    io::{stdout, Result},
};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // TODO main loop
    let mut counter = 0;
    loop {
        let counter_text = Text::styled(format!("{}", counter), Style::default().fg(Color::Red));
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new(Text::from(format!("Value: {}", counter_text)))
                    .block(
                        Block::new()
                            .title("Counter App Tutorial")
                            .style(Style::new().bold())
                            .borders(Borders::ALL),
                    )
                    .alignment(Alignment::Center)
                    .on_black()
                    .wrap(Wrap { trim: true }),
                area,
            );
        })?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q') {
                        break;
                    }
                    if key.code == KeyCode::Left {
                        counter = counter - 1;
                    }
                    if key.code == KeyCode::Right {
                        counter = counter + 1;
                    }
                }
            }
        }
    }
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn draw_ui() {}
