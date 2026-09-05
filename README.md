# 🦀 RustDU

**RustDU** is a modern, interactive terminal-based disk usage analyzer written in Rust.  
Inspired by `ncdu`, it provides a colorful TUI with **real-time** sorting, file/folder icons, percentage bars, and intuitive navigation — all with **Russian keyboard shortcut support** (no need to switch layouts).

## 📌 Feedback from you:
RustDU🦀 is actively maintained as a pet project alongside my university studies🎓. Updates🔄 and new features✨ might occasionally land a little slower during semesters📚, but I'm fully committed
 to fixing bugs🐛 and improving the tool🛠️.

Contributions🤝 and issues🐞 are always welcome! 😄 I'd love to read your ideas💡, requests📩, and suggestions💭—though I do ask that you refrain from posting nasty comments💬🚫.

📢 My channels:
📱 https://t.me/MyRustDU
📥 https://t.me/MyRustDU_input

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
- ⚡ **Fast navigation** – sizes are cached to avoid re‑scanning
- 💻 **Cross‑platform** – works on Linux, macOS, Windows (with proper terminal support)

---

## 📦 Installation

### Via `cargo` (requires Rust)
```bash
cargo install rustdu
```
