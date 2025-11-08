# Persian Subtitle Fixer

Modern Rust/libadwaita rewrite of the original Qt utility for converting Persian SRT subtitles from Windows-1256 (Windows Arabic) to UTF-8 so they render correctly on Linux, macOS, and open-source media players.

## Personal Story

This was my first serious C++/Qt project from 10 years ago when I began using Qt seriously and became skilled in C++. However, this year I switched to Rust and now use GTK, Tauri, Iced, and other frameworks instead of Qt. I decided to rewrite my first project in Rust both for learning purposes and for fun! It's interesting to see how much development practices and tooling have evolved over the past decade.

## Branch layout

- `main` – the new Rust implementation (this branch).
- `old-cpp` – frozen copy of the original Qt/C++ application for reference.

## Highlights

- Auto-detects UTF-8 input and falls back to Windows-1256 when needed.
- Native libadwaita UI with GNOME-style header bar, shortcuts, and theme integration.
- Drag-and-drop support, improved error dialogs, and better validation of command-line arguments.
- Consistent UTF-8 saving workflow with overwrite confirmation.
- Font customization via the GTK font dialog (applied to the preview only).

## Prerequisites

You need the GTK4/libadwaita development stack plus a recent Rust toolchain (1.74+). On Ubuntu/Debian:

```bash
sudo apt install libadwaita-1-dev libgtk-4-dev pkg-config build-essential
curl https://sh.rustup.rs -sSf | sh
```

Other distributions provide similar packages (`libadwaita`/`gtk4` headers).

## Building & running

```bash
# Format, lint, test
make fmt
make lint

# Debug build
cargo run -- path/to/subtitle.srt

# Release binary
make build
```

Drag an `.srt` file onto the window or use **Open**. Choose the source encoding (Auto, UTF-8, Windows-1256) before opening. Use **Save** to write a UTF-8 copy. The font button opens a GTK font picker that only affects the preview widget.

## Packaging notes

- The app ID is `ir.ehsan.PersianSubtitleFixer`.
- `cargo build --release` outputs `target/release/persian-subtitle-fixer`.
- Ship the GTK/libadwaita runtime for non-GNOME environments if you package this upstream.

## License

GPL-3.0-or-later (see `LICENSE`). Original author: Ehsan Tork (journalehsan@gmail.com). The Rust port keeps the same license.
