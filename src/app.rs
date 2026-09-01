use crossterm::event::KeyCode;
use std::path::PathBuf;
use ratatui::widgets::ListState;
use std::thread;
use std::time::Duration;

use crate::scanner;

#[derive(PartialEq)]
pub enum AppMode {
    Browse,
    ConfirmDelete,
    InputPath,
    Help,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SortMode {
    Size,
    Name,
}

pub struct App {
    pub current_path: PathBuf,
    pub nodes: Vec<String>,           // строки для отображения (с размерами, процентами, иконками)
    pub list_state: ListState,
    pub mode: AppMode,
    pub input_buffer: String,
    pub loading: bool,
    pub delete_target: Option<String>,
    pub total_size: u64,              // общий размер всех элементов в текущей папке
    pub sort_mode: SortMode,
    pub raw_entries: Vec<scanner::FileEntry>, // сырые данные для пересортировки
}

impl App {
    pub fn new(path: PathBuf) -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        let mut app = App {
            current_path: path,
            nodes: Vec::new(),
            list_state: state,
            mode: AppMode::Browse,
            input_buffer: String::new(),
            loading: false,
            delete_target: None,
            total_size: 0,
            sort_mode: SortMode::Size,
            raw_entries: Vec::new(),
        };
        app.load_directory();
        app
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        match self.mode {
            AppMode::Browse => self.handle_browse(key),
            AppMode::ConfirmDelete => self.handle_confirm_delete(key),
            AppMode::InputPath => self.handle_input_path(key),
            AppMode::Help => {
                // По любой клавише выходим из справки
                self.mode = AppMode::Browse;
            }
        }
    }

    fn handle_browse(&mut self, key: KeyCode) {
        match key {
            KeyCode::Down => {
                let i = self.list_state.selected().unwrap_or(0);
                let max = self.nodes.len().saturating_sub(1);
                self.list_state.select(Some(if i == max { 0 } else { i + 1 }));
            }
            KeyCode::Up => {
                let i = self.list_state.selected().unwrap_or(0);
                let max = self.nodes.len().saturating_sub(1);
                self.list_state.select(Some(if i == 0 { max } else { i - 1 }));
            }
            KeyCode::Enter => {
                if let Some(selected) = self.list_state.selected() {
                    if let Some(entry_str) = self.nodes.get(selected) {
                        let name = entry_str.split_whitespace().last().unwrap_or(entry_str);
                        let new_path = self.current_path.join(name);
                        if new_path.is_dir() {
                            self.current_path = new_path;
                            self.load_directory();
                        }
                    }
                }
            }
            KeyCode::Backspace => {
                if let Some(parent) = self.current_path.parent() {
                    self.current_path = parent.to_path_buf();
                    self.load_directory();
                }
            }
            KeyCode::Char('d') | KeyCode::Char('в') => {
                if let Some(selected) = self.list_state.selected() {
                    if let Some(entry_str) = self.nodes.get(selected) {
                        let name = entry_str.split_whitespace().last().unwrap_or(entry_str);
                        self.delete_target = Some(name.to_string());
                        self.mode = AppMode::ConfirmDelete;
                    }
                }
            }
            KeyCode::Char('g') | KeyCode::Char('п') => {
                self.input_buffer.clear();
                self.mode = AppMode::InputPath;
            }
            KeyCode::Char('s') => {
                self.sort_mode = SortMode::Size;
                self.apply_sort();
            }
            KeyCode::Char('n') => {
                self.sort_mode = SortMode::Name;
                self.apply_sort();
            }
            KeyCode::Char('r') => {
                self.load_directory();
            }
            KeyCode::Char('?') => {
                self.mode = AppMode::Help;
            }
            _ => {}
        }
    }

    fn handle_confirm_delete(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('y') | KeyCode::Char('н') => {
                if let Some(name) = self.delete_target.take() {
                    let full_path = self.current_path.join(&name);
                    match scanner::delete_entry(&full_path) {
                        Ok(_) => {
                            self.load_directory();
                            self.mode = AppMode::Browse;
                        }
                        Err(_) => {
                            self.mode = AppMode::Browse;
                        }
                    }
                } else {
                    self.mode = AppMode::Browse;
                }
            }
            KeyCode::Char('n') | KeyCode::Char('т') | KeyCode::Esc => {
                self.delete_target = None;
                self.mode = AppMode::Browse;
            }
            _ => {}
        }
    }

    fn handle_input_path(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Enter => {
                let path = PathBuf::from(&self.input_buffer);
                if path.exists() && path.is_dir() {
                    self.current_path = path;
                    self.load_directory();
                }
                self.mode = AppMode::Browse;
                self.input_buffer.clear();
            }
            KeyCode::Esc => {
                self.mode = AppMode::Browse;
                self.input_buffer.clear();
            }
            _ => {}
        }
    }

    fn apply_sort(&mut self) {
        match self.sort_mode {
            SortMode::Size => {
                self.raw_entries.sort_by(|a, b| b.size.cmp(&a.size));
            }
            SortMode::Name => {
                self.raw_entries.sort_by(|a, b| a.name.cmp(&b.name));
            }
        }
        self.build_display_strings();
    }

    fn build_display_strings(&mut self) {
        let mut display = Vec::new();
        for entry in &self.raw_entries {
            let size_str = format_size(entry.size);
            let icon = if entry.is_dir { "📁" } else { "💾" };
            let percent = if self.total_size > 0 {
                (entry.size as f64 / self.total_size as f64) * 100.0
            } else {
                0.0
            };
            display.push(format!("{:>8}  {:>5.1}%  {}  {}", size_str, percent, icon, entry.name));
        }
        self.nodes = display;
    }

    pub fn load_directory(&mut self) {
        self.loading = true;
        // Даём время отрисовать "Загрузка..."
        thread::sleep(Duration::from_millis(100));

        match scanner::scan_directory(&self.current_path) {
            Ok((entries, total_size)) => {
                self.raw_entries = entries;
                self.total_size = total_size;
                // Сортируем в соответствии с текущим режимом
                self.apply_sort();
                self.list_state.select(Some(0));
            }
            Err(_) => {
                self.raw_entries = Vec::new();
                self.total_size = 0;
                self.nodes = Vec::new();
                self.list_state.select(Some(0));
            }
        }
        self.loading = false;
    }
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