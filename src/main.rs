#[macro_use]
extern crate log;

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
mod session_list;
mod tmux;
mod ui;
mod session_manager;

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
    pretty_env_logger::init();

    let events = Events::new();

    let mut terminal = match setup() {
        Ok(t) => t,
        Err(e) => {
            error!("Failed to configure terminal {:?}", e);
            std::process::exit(1);
        }
    };

    let mut app = UI::new();
    loop {
        terminal
            .draw(|mut f| app.draw(&mut f))
            .expect("Failed to draw terminal");

        let event = match events.next() {
            Ok(e) => e,
            Err(e) => {
                error!("Failed to receive event, error: {:?}", e);
                std::process::exit(2);
            }
        };

        if app.handle_event(event) {
            break;
        }
    }
}
