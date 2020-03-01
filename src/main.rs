use std::io;

use std::error::Error;

use tui::backend::TermionBackend;
use tui::Terminal;

use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;

mod event;
mod session;
mod tmux;
mod ui;

use self::event::*;
use self::ui::UI;

type TermType = tui::terminal::Terminal<
    TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<std::io::Stdout>>>>,
>;

fn setup() -> Result<TermType, Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    Ok(terminal)
}

fn main() {
    let events = Events::new();

    let mut terminal = match setup() {
        Ok(t) => t,
        Err(_) => panic!("Failed to configure terminal"),
    };

    let mut app = UI::new();
    loop {
        terminal
            .draw(|mut f| app.draw(&mut f))
            .expect("Failed to draw terminal");
        if app.handle_event(events.next().unwrap()) {
            break;
        }
    }
}
