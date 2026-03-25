<div align="center">
<pre>

▓▓▒░  [ TERMINAL - DOOM ]  ░▒▓▓

Doom running in the terminal. No window, no GPU, just ANSI escape sequences and Unicode characters on stdout.

[![Download latest release](https://img.shields.io/badge/Download-Latest%20Release-2ea44f?style=for-the-badge)](https://github.com/chrisJSeng/terminal-doom/releases/latest)
</pre>
</div>

---

## What it is

Integrates the original Doom engine (via [doomgeneric](https://github.com/ozkl/doomgeneric)) compiled as a static C library linked into a Rust frontend. Rust owns everything the user sees and interacts with: terminal rendering, input handling, HUD, WAD parsing and world logic.

The result is Doom running in any terminal with 24-bit True Color support.

---

## Requirements

**OS:** Linux (tested on Ubuntu 22.04+).

**System dependencies:**

```bash
# Rust stable toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# C compiler (to build the doomgeneric engine)
sudo apt install build-essential

# Optional: enables in-game sound and music
sudo apt install libsdl2-dev libsdl2-mixer-dev
```

Without SDL2, the game compiles and runs fine — just without audio. The build script detects it automatically.

---

## Running

### Standalone (pre-built binary)

For `v0.1.0`, download the release artifact `terminal-doom-v0.1.0-linux-x86_64.tar.gz`, extract it, and run:

```bash
tar -xzf terminal-doom-v0.1.0-linux-x86_64.tar.gz
cd terminal-doom-v0.1.0-linux-x86_64
./doom-terminal
```

### From source

```bash
git clone https://github.com/chrisJSeng/terminal-doom
cd doom-terminal
./run.sh
```

`run.sh` builds the release binary on first run (requires Rust + `build-essential`) and executes it directly. Subsequent runs skip the build step.

---

The terminal must support **True Color (24-bit)**. GNOME Terminal, Alacritty, Kitty, iTerm2, and the VS Code integrated terminal all work.

---

## Controls

| Key | Action |
|---|---|
| `W` / `↑` | Move forward |
| `S` / `↓` | Move backward |
| `A` / `←` | Turn left |
| `D` / `→` | Turn right |
| `Z` | Strafe left |
| `C` | Strafe right |
| `Space` | Fire |
| `Ctrl` | Use / open doors |
| `Q` | Previous weapon |
| `E` | Next weapon |
| `Tab` | Automap |
| `P` | Menu |
| `Enter` | Confirm |
| `Backspace` | Back / cancel |
| `Esc` | Quit |

---

## Architecture

```
src/
├── main.rs               # Orchestration and game loop
├── domain.rs             # World logic (collision, pickups)
├── app/
│   ├── bootstrap.rs      # Application state initialization
│   ├── input/            # Polling, mapping and input state
│   └── runtime/          # Game loop and movement
├── render/
│   ├── terminal.rs       # Terminal control (crossterm)
│   └── framebuffer_draw.rs # RGB → Unicode character conversion
├── framebuffer/
│   ├── c_backend.rs      # FFI bridge to doomgeneric C engine
│   └── providers.rs      # World geometry upload to the engine
├── wad/
│   ├── reader.rs         # WAD file reading and validation
│   └── parser.rs         # WAD binary structure parsing
├── types/                # Typed domain structs
└── constants/            # Constants organized by domain
```

---

## Implementation challenges

### 1. FFI with a C engine across the thread boundary

doomgeneric expects to own the main loop and calls C callbacks exported by the host. Integrating that without a display window — keeping Doom ticking in the background while Rust controls the interface — requires careful isolation via `OnceLock<Mutex<...>>` in the backend and explicit sequencing of player/frame state to avoid data races.

The Rust compiler cannot verify safety across `unsafe extern "C"` boundaries, so the contract is enforced manually: the engine's screen buffer (`DG_ScreenBuffer`) is only read by Rust after each `doomgeneric_Tick()` completes, enforced through explicit sequencing in the game loop.

### 2. Rendering RGB pixels as terminal characters

Each "terminal pixel" spans two rows of the original image: the upper and lower halves are encoded as the foreground/background of a `▀` (UPPER HALF BLOCK) character. This doubles vertical resolution without using two characters per cell.

The challenge isn't just color conversion — it's making the human eye perceive a fluid image within ~16ms per frame. The solution combines:

- **Tone mapping** with gamma and adaptive saturation to compensate for Doom's original 256-color palette
- **Unsharp masking** applied regionally (lower screen areas, where the HUD lives, receive stronger sharpening)
- **Cell cache**: each terminal cell stores a 64-bit signature derived from its color samples; unchanged cells are never rewritten, drastically reducing ANSI sequences emitted per frame
- **Adjacent span merging**: consecutive cells with the same style are batched into a single `Print()` instead of one command per cell

### 3. Non-blocking input in raw terminal mode

In raw mode (`crossterm::terminal::enable_raw_mode`), every keypress must be read without blocking the 60fps game loop. *Held* actions (movement) and *discrete* actions (menu open) need different handling. The input system drains all pending events each frame via non-blocking polling, resolves opposing pairs (forward/backward, strafe L/R), and applies a stale timeout to prevent ghost inputs — which can occur when some terminals don't emit key-up events.

### 4. WAD binary format parsing

The WAD format is a 1993 binary with no schema. The parser handles:

- Patch columns in tall-patch format (textures > 256px tall via accumulated `top_delta`)
- Multi-patch texture composition (multiple patches overlaid at arbitrary offsets)
- Flats (floor/ceiling textures) at a fixed 4096-byte size
- BSP ordering to determine correct wall rendering sequence

### 5. Adaptive viewport and aspect ratio

A terminal cell is not square — typical proportions are roughly 1:2 (width:height). Rendering the 320×200 Doom framebuffer directly would produce a squashed image. The viewport is calculated dynamically from the terminal dimensions, preserving the original aspect ratio via letterboxing, and recalculated on live terminal resize.

---

## Tests

```bash
cargo test
```

45 tests covering: WAD parsing, collision, pickups, input state, game loop, framebuffer rendering and bootstrap.

---

## Notes

- `run.sh` always uses the release binary — significantly faster than a dev build for rendering
- The Doom shareware (`doom1.wad`) includes the full Episode 1 (9 maps)
- The original Doom engine runs at 35 ticks/s internally; the Rust frontend targets ~60fps and synchronizes
