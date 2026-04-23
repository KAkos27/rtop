# rtop

> A lightweight, real-time system monitor for the terminal — built with Rust.

```
┌──────────────────────────────────────┬──────────────────────┐
│  CPU  [==============..........] 61% │ C0 [======......] 50%│
│                                      │ C1 [=========...] 75%│
│                                      │ C2 [====........] 33%│
│                                      │ C3 [=======.....] 58%│
├────────────────────┬─────────────────┴──────────────────────┤
│MEM   [====....] 52%│  Filter: _                             │
│disk1 [=====...] 74%├────────────────────────────────────────┤
│disk2 [===.....] 41%│  PID   Name       CPU%    MEM          │
│                    │> 812  firefox    4.2%   1.0 GB         │
│                    │  391  kernel     0.5%   4.0 MB         │
│                    │  204  sshd       0.0%   2.0 MB         │
└────────────────────┴────────────────────────────────────────┘
```

---

## Features

- **Live CPU monitoring** — global usage gauge + per-core breakdown
- **Memory & disk usage** — at-a-glance gauges for all mounted volumes
- **Process table** — sortable by CPU or memory, updated in real time
- **Process search** — inline filter bar to find processes by name
- **Kill processes** — send SIGKILL to any selected process
- **Vim-style navigation** — `j`/`k`, `g`/`G` to move through the list

---

## Keybindings

### Normal Mode

| Key               | Action                            |
| ----------------- | --------------------------------- |
| `q` / `Esc`       | Quit                              |
| `j` / `↓`         | Select next process               |
| `k` / `↑`         | Select previous process           |
| `g`               | Jump to first process             |
| `G`               | Jump to last process              |
| `s`               | Toggle sort: CPU ↔ Memory         |
| `f`               | Enter filter mode                 |
| `c`               | Clear filter, resume live updates |
| `x` / `Backspace` | Kill selected process (SIGKILL)   |

### Filter Mode

| Key         | Action                  |
| ----------- | ----------------------- |
| `Enter`     | Apply filter            |
| `Backspace` | Delete character        |
| `←` / `→`   | Move cursor             |
| `Esc`       | Cancel and clear filter |

---

## Installation

**Prerequisites:** [Rust toolchain](https://rustup.rs/)

```bash
git clone https://github.com/KAkos27/rtop
cd rtop
cargo build --release
./target/release/rtop
```

---

## Tech Stack

| Crate                                                    | Purpose                          |
| -------------------------------------------------------- | -------------------------------- |
| [`ratatui`](https://github.com/ratatui-org/ratatui)      | TUI rendering & widget framework |
| [`crossterm`](https://github.com/crossterm-rs/crossterm) | Cross-platform terminal I/O      |
| [`sysinfo`](https://github.com/GuillaumeGomez/sysinfo)   | CPU, memory, disk & process data |
| [`color-eyre`](https://github.com/eyre-rs/color-eyre)    | Error handling & reporting       |

---

## License

MIT
