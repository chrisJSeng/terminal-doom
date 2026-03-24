mod app;
mod constants;
mod domain;
mod framebuffer;
mod render;
#[cfg(test)]
mod tests;
mod types;
mod wad;

use std::io::{self, Write};
use std::panic;

use app::bootstrap::bootstrap_app;
use app::runtime::run_game_loop;
use constants::UI_STARTUP_ERROR_PREFIX;

fn main() {
    setup_panic_handler();

    if let Err(err) = run() {
        cleanup_terminal();
        eprintln!("\n{UI_STARTUP_ERROR_PREFIX} {err}");
    }

    cleanup_terminal();
}

fn cleanup_terminal() {
    let mut stdout = std::io::stdout();

    let _ = crossterm::terminal::disable_raw_mode();

    let _ = stdout.write_all(b"\x1b[?1004l");
    let _ = stdout.write_all(b"\x1b[?1003l");
    let _ = stdout.write_all(b"\x1b[?1015l");
    let _ = stdout.write_all(b"\x1b[?1006l");
    let _ = stdout.write_all(b"\x1b[?2004l");
    let _ = stdout.write_all(b"\x1b[m");
    let _ = stdout.write_all(b"\x1b[H\x1b[J");
    let _ = stdout.flush();

    let _ = crossterm::execute!(
        stdout,
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::cursor::Show
    );
    let _ = stdout.flush();
}

fn setup_panic_handler() {
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        cleanup_terminal();
        default_hook(panic_info);
    }));
}

fn run() -> io::Result<()> {
    let app = bootstrap_app()?;

    run_game_loop(app)
}
