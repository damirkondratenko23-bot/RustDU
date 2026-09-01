use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use once_cell::sync::Lazy;

// Кэш размеров папок (чтобы не пересчитывать повторно)
static SIZE_CACHE: Lazy<Mutex<HashMap<PathBuf, u64>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    #[allow(dead_code)]
    pub path: PathBuf,
    pub size: u64,
    pub is_dir: bool,
}

/// Рекурсивно вычисляет размер папки с использованием кэша
fn dir_size(path: &PathBuf) -> u64 {
    // Проверяем кэш
    if let Some(cached) = SIZE_CACHE.lock().unwrap().get(path) {
        return *cached;
    }

    let mut total = 0;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let meta = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };
            if meta.is_file() {
                total += meta.len();
            } else if meta.is_dir() {
                let sub_size = dir_size(&entry.path());
                total += sub_size;
            }
        }
    }
    // Сохраняем в кэш
    SIZE_CACHE.lock().unwrap().insert(path.clone(), total);
    total
}

/// Сканирует директорию, возвращает список и общий размер всех элементов в ней
pub fn scan_directory(path: &PathBuf) -> anyhow::Result<(Vec<FileEntry>, u64)> {
    let mut entries = Vec::new();
    let read_dir = fs::read_dir(path)?;
    let mut total_size = 0;

    for entry in read_dir {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let name = entry.file_name().to_string_lossy().to_string();
        let path = entry.path();
        let is_dir = metadata.is_dir();
        let size = if is_dir {
            dir_size(&path)
        } else {
            metadata.len()
        };
        total_size += size;
        entries.push(FileEntry { name, path, size, is_dir });
    }

    // Сортируем по размеру (убывание) — как в ncdu
    entries.sort_by(|a, b| b.size.cmp(&a.size));
    Ok((entries, total_size))
}

/// Удаляет файл или папку (рекурсивно)
pub fn delete_entry(path: &PathBuf) -> anyhow::Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)?;
    } else {
        fs::remove_file(path)?;
    }
    // Инвалидируем кэш для родительской папки
    if let Some(parent) = path.parent() {
        SIZE_CACHE.lock().unwrap().remove(parent);
    }
    Ok(())
}

/// Очистка кэша (например, при выходе)
#[allow(dead_code)]
pub fn clear_cache() {
    SIZE_CACHE.lock().unwrap().clear();
}