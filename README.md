<p align="center">
<pre align="center">
 ▗▄▄▖ ▗▄▖ ▗▖ ▗▖▗▖  ▗▖▗▄▄▄   ▗▄▄▖
▐▌   ▐▌ ▐▌▐▌ ▐▌▐▛▚▖▐▌▐▌  █ ▐▌   
 ▝▀▚▖▐▌ ▐▌▐▌ ▐▌▐▌ ▝▜▌▐▌  █  ▝▀▚▖
▗▄▄▞▘▝▚▄▞▘▝▚▄▞▘▐▌  ▐▌▐▙▄▄▀ ▗▄▄▞▘
</pre>
</p>

<p align="center">
  <em>A filesystem-first audio player for the terminal</em>
</p>

<p align="center">
  <a href="https://github.com/alhaymex/sounds/releases/latest"><img src="https://img.shields.io/github/v/release/alhaymex/sounds?style=flat-square&color=cyan" alt="Release"></a>
  <a href="https://github.com/alhaymex/sounds/blob/main/LICENSE"><img src="https://img.shields.io/github/license/alhaymex/sounds?style=flat-square" alt="License"></a>
  <a href="https://github.com/alhaymex/sounds/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/alhaymex/sounds/ci.yml?style=flat-square&label=CI" alt="CI"></a>
</p>

---

**Sounds** is a lightweight TUI music player that lets you browse, manage, and play audio directly from your local files. No databases, no daemons — just point it at a directory and listen.

## Features

- 🎵 Play MP3, FLAC, WAV, OGG, and AAC files
- 📁 Browse your music library as a file tree
- 🎛️ Playback controls (play, pause, skip, volume)
- ⌨️ Fully keyboard-driven interface
- ⚡ Fast and lightweight

## Install

### Linux & MacOS

```bash
curl -sSL https://raw.githubusercontent.com/alhaymex/sounds/main/install.sh | bash
```

### Windows (PowerShell)

```bash
irm https://raw.githubusercontent.com/alhaymex/sounds/main/install.ps1 | iex
```

### From source

```bash
git clone https://github.com/alhaymex/sounds.git
cd sounds
cargo install --path .
```

> **Note:** On Linux, you need `libasound2-dev` and `pkg-config` installed.

## Update

```bash
sounds update
```

## Usage

```bash
sounds
```

This opens the TUI where you can browse and play your music. Press `?` for keybindings.

## Roadmap

- [x] Support nested directories in the file browser
- [x] Navigate screen history by changing `prev_screen: Screen` to a `Vec<Screen>`
- [x] Auto-play next song
- [x] Add simple, scoped file manipulation (e.g. rename a playlist or a song)
- [x] Audio playack speed controls
- [ ] Search
- [ ] YouTube integration via yt-dlp
- [ ] macOS support

## License

[MIT](LICENSE)
