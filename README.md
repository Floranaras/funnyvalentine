# Valentine-Tui

 A playful terminal-based Valentine's Day proposal app built with Rust and [ratatui](https://github.com/ratatui-org/ratatui). Ask someone to be your Valentine — right from your terminal.

---

## What it does

Launches a full-screen TUI with a beating ASCII heart and a big "Be Mine?" prompt. Three buttons appear:

- **Y — YES!** — Triggers a celebration with colorful particle effects and a happy little chime
- **M — Maybe?** — Shows a hopeful waiting screen with an animated "hmm..."
- **N — No** — The No button doesn't work. It never works. It just counts your attempts and gets more insulting each time.

Press `Q` or `Esc` to exit (if you dare).

---

## Installation

### Arch Linux (AUR)

```bash
yay -S valentine-tui
# or
paru -S valentine-tui
```

### Fedora / RHEL (COPR)

```bash
sudo dnf install dnf-plugins-core
sudo dnf copr enable amanekai/valentine-tui
sudo dnf install valentine-tui
```

### Build from source

```bash
git clone https://github.com/Floranaras/funnyvalentine
cd funnyvalentine
cargo build --release
./target/release/valentine-tui
```

**Dependencies:** `alsa-lib` (Linux, for audio)

---

## Controls

| Key | Action |
|-----|--------|
| `Y` | Yes! |
| `M` | Maybe |
| `N` | (nice try) |
| `Q` / `Esc` | Exit |

---

## Built with

- [ratatui](https://github.com/ratatui-org/ratatui) — TUI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) — Terminal backend
- [tui-big-text](https://github.com/joshka/tui-big-text) — Big pixel text rendering
- [rodio](https://github.com/RustAudio/rodio) — Audio playback
- [rand](https://github.com/rust-random/rand) — Particle randomization

---

## License

MIT — Made with Love by Amane Kai / 雨音カイ
