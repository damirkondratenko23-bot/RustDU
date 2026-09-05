use ratatui::{prelude::*, widgets::*};
use crate::app::{App, AppMode};

fn color_for_size(size: u64) -> Color {
    const GB: u64 = 1024 * 1024 * 1024;
    const MB: u64 = 1024 * 1024;
    const TEN_MB: u64 = 10 * MB;
    const HUNDRED_MB: u64 = 100 * MB;
    if size >= GB {
        Color::Red
    } else if size >= HUNDRED_MB {
        Color::Yellow
    } else if size >= TEN_MB {
        Color::LightBlue
    } else if size >= MB {
        Color::Green
    } else {
        Color::Gray
    }
}

fn parse_size(s: &str) -> Option<u64> {
    let s = s.trim();
    if s.is_empty() { return None; }
    let (num_part, suffix) = s.split_at(s.len() - 1);
    let num: f64 = num_part.parse().ok()?;
    let multiplier = match suffix {
        "B" => 1,
        "K" => 1024,
        "M" => 1024 * 1024,
        "G" => 1024 * 1024 * 1024,
        _ => return None,
    };
    Some((num * multiplier as f64) as u64)
}

pub fn render(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    // ---- Основной список ----
    let items: Vec<ListItem> = app
        .nodes
        .iter()
        .map(|line| {
            let size_str = line.split_whitespace().next().unwrap_or("0B");
            let size = parse_size(size_str).unwrap_or(0);
            let color = color_for_size(size);
            ListItem::new(line.clone()).style(Style::default().fg(color))
        })
        .collect();

    let title = format!(
        "{}  |  Элементов: {}  |  Общий размер: {}",
        app.current_path.display(),
        app.raw_entries.len(),
        format_size(app.total_size)
    );

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(Style::default().bg(Color::Blue))
        .highlight_symbol("> ");

    let list_area = Rect::new(area.x, area.y, area.width, area.height.saturating_sub(2));
    frame.render_stateful_widget(list, list_area, &mut app.list_state);

    // ---- Нижняя строка ----
    let bottom_area = Rect::new(area.x, area.height - 2, area.width, 2);
    let mut bottom_text = String::new();

    match app.mode {
        AppMode::Browse => {
            bottom_text.push_str(
                "[q/й] Выход  [d/в] Удалить  [g/п] Перейти  [s] Сорт.по разм.  [n] Сорт.по имени  [r] Обновить  [?] Справка",
            );
        }
        AppMode::ConfirmDelete => {
            bottom_text.push_str("Удалить выбранный элемент? (y/н - да, n/т - нет)");
        }
        AppMode::InputPath => {
            bottom_text.push_str(&format!("Введите путь: {}", app.input_buffer));
        }
        AppMode::Help => {
            bottom_text.push_str(
                "Клавиши: ↑↓ - навигация, Enter - войти, Backspace - назад, q - выход, d - удалить, g - перейти, s/n - сортировка, r - обновить",
            );
        }
    }

    if app.loading {
        let loading_text = Paragraph::new("Загрузка...")
            .block(Block::default().borders(Borders::NONE))
            .style(Style::default().fg(Color::Yellow));
        let loading_area = Rect::new(area.width / 2 - 6, area.height / 2 - 1, 12, 1);
        frame.render_widget(loading_text, loading_area);
    }

    let bottom_paragraph = Paragraph::new(bottom_text)
        .block(Block::default().borders(Borders::TOP).border_style(Style::default().fg(Color::Gray)))
        .style(Style::default().fg(Color::White));
    frame.render_widget(bottom_paragraph, bottom_area);
}

fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    if size >= GB {
        format!("{:.1}G", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.1}M", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.1}K", size as f64 / KB as f64)
    } else {
        format!("{}B", size)
    }
}