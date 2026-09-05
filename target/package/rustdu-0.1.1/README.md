# 🦀 RustDU

**RustDU** is a modern, interactive terminal-based disk usage analyzer written in Rust.  
Inspired by `ncdu`, it provides a colorful TUI with **real-time** sorting, file/folder icons, percentage bars, and intuitive navigation — all with **Russian keyboard shortcut support** (no need to switch layouts).

[![Crates.io](https://img.shields.io/crates/v/rustdu.svg)](https://crates.io/crates/rustdu)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/yourusername/rustdu/actions)

---

## ✨ Features

- 📊 **Real directory sizes** (recursive, cached for speed)
- 🎨 **Color‑coded files** – red for huge, yellow for large, blue for medium, green for small
- 📁 **Icons** – `📁` for folders, `💾` for files
- 📈 **Percentages** – each item shows its share of the total size
- 🔄 **Sorting** – by size (default) or by name (`s` / `n` keys)
- 🗑️ **Delete files/folders** (with confirmation)
- 🧭 **Navigate** – jump to any path (`g` key), go back (`Backspace`), enter folders (`Enter`)
- 🔄 **Refresh** current directory (`r` key)
- ❓ **Help screen** (`?` key)
- 🇷🇺 **Russian keyboard support** – use `й/в/п/н/т` without switching layout
- ⚡ **Fast navigation** – sizes are cached to avoid re‑scanning
- 💻 **Cross‑platform** – works on Linux, macOS, Windows (with proper terminal support)

---

## 📦 Installation

### Via `cargo` (requires Rust)
cargo install rustdu
