mod app;
mod scanner;
mod ui;
mod tui;

use clap::Parser;
use crossterm::event::{self, Event, KeyCode};
use std::path::PathBuf;
use ratatui::{prelude::*, widgets::*};
use std::time::{Duration, Instant};

use app::App;

#[derive(Parser)]
struct Args {
    #[arg(default_value = ".")]
    path: PathBuf,
}

/// Показывает экран загрузки с прогресс-баром в течение 5 секунд
fn show_loading_screen(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> anyhow::Result<()> {
    let start = Instant::now();
    let duration = Duration::from_secs(5);
    let bar_width = 40u16; // теперь явно u16

    loop {
        let elapsed = start.elapsed();
        if elapsed >= duration {
            break;
        }
        let progress = elapsed.as_secs_f32() / duration.as_secs_f32();
        let percent = (progress * 100.0) as u16;
        let filled = (progress * bar_width as f32) as u16;
        let empty = bar_width.saturating_sub(filled); // безопасное вычитание

        terminal.draw(|frame| {
            let area = frame.area(); // заменили .size() на .area()
            let center_x = area.width / 2;
            let center_y = area.height / 2;

            // Заголовок
            let title = Paragraph::new("Загрузка анализатора дискового пространства")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Yellow));
            frame.render_widget(title, Rect::new(0, center_y - 2, area.width, 1));

            // Проценты
            let percent_text = Paragraph::new(format!("{}%", percent))
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Cyan));
            frame.render_widget(percent_text, Rect::new(0, center_y, area.width, 1));

            // Прогресс-бар
            let bar = format!("[{}{}]", "=".repeat(filled as usize), " ".repeat(empty as usize));
            let bar_paragraph = Paragraph::new(bar)
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Green));
            let bar_x = center_x.saturating_sub(bar_width / 2); // защита от отрицательного
            let bar_area = Rect::new(bar_x, center_y + 1, bar_width, 1);
            frame.render_widget(bar_paragraph, bar_area);
        })?;

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut terminal = tui::init()?;

    // Показываем экран загрузки
    show_loading_screen(&mut terminal)?;

    // Теперь основное приложение
    let mut app = App::new(args.path);

    loop {
        terminal.draw(|frame| ui::render(frame, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('й') | KeyCode::Esc => break,
                _ => app.handle_key(key.code),
            }
        }
    }

    tui::restore()?;
    Ok(())
}