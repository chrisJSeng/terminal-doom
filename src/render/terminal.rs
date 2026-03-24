use std::io::{self, stdout, Write};

use crossterm::{
    cursor, execute,
    style::{self, Color, Print},
    terminal::{self, BeginSynchronizedUpdate, Clear, ClearType, EndSynchronizedUpdate},
};

use crate::types::{TerminalRenderer, TruncateToWidthParams};

impl TerminalRenderer {
    pub fn new() -> io::Result<Self> {
        let mut stdout = stdout();
        let (cols, rows) = terminal::size()?;
        terminal::enable_raw_mode()?;

        execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;
        Ok(Self {
            stdout,
            cols,
            rows,
            framebuffer_cache: Vec::new(),
            framebuffer_cache_cols: 0,
            framebuffer_cache_rows: 0,
            framebuffer_viewport_signature: None,
        })
    }

    pub fn begin_framebuffer_frame(&mut self) -> io::Result<()> {
        self.refresh_size()?;

        if self.framebuffer_cache.is_empty() {
            return execute!(
                self.stdout,
                BeginSynchronizedUpdate,
                cursor::MoveTo(0, 0),
                Clear(ClearType::All)
            );
        }

        execute!(self.stdout, BeginSynchronizedUpdate)
    }

    pub fn show_status(&mut self, text: &str) -> io::Result<()> {
        let max_chars = self.cols as usize;
        let status_text = truncate_to_width(TruncateToWidthParams { text, max_chars });

        execute!(
            self.stdout,
            cursor::MoveTo(0, 0),
            Clear(ClearType::CurrentLine),
            style::SetForegroundColor(Color::White),
            Print(status_text),
            EndSynchronizedUpdate
        )?;

        self.stdout.flush()
    }

    pub fn refresh_size(&mut self) -> io::Result<()> {
        let (cols, rows) = terminal::size()?;
        let resized = cols != self.cols || rows != self.rows;
        self.cols = cols;
        self.rows = rows;
        if resized {
            self.invalidate_framebuffer_cache();
        }
        Ok(())
    }

    pub fn invalidate_framebuffer_cache(&mut self) {
        self.framebuffer_cache.clear();
        self.framebuffer_cache_cols = 0;
        self.framebuffer_cache_rows = 0;
        self.framebuffer_viewport_signature = None;
    }
}

impl Drop for TerminalRenderer {
    fn drop(&mut self) {
        let _ = self.stdout.write_all(b"\x1b[?1004l");
        let _ = self.stdout.write_all(b"\x1b[?1003l");
        let _ = self.stdout.write_all(b"\x1b[?1015l");
        let _ = self.stdout.write_all(b"\x1b[?1006l");
        let _ = self.stdout.write_all(b"\x1b[?2004l");
        let _ = self.stdout.flush();

        let _ = execute!(self.stdout, terminal::LeaveAlternateScreen, cursor::Show);
        let _ = self.stdout.flush();
        let _ = terminal::disable_raw_mode();
    }
}

fn truncate_to_width(params: TruncateToWidthParams<'_>) -> String {
    let TruncateToWidthParams { text, max_chars } = params;

    text.chars().take(max_chars).collect()
}
