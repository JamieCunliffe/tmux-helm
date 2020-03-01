use super::tmux::*;

use super::event::Event;

use termion::event::Key;

use tui::backend::Backend;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListState, Text};
use tui::Frame;

pub struct UI {
    sessions: Vec<Session>,
    session_state: ListState,
}

impl UI {
    pub fn new() -> UI {
        let mut state = ListState::default();
        state.select(Some(0));

        UI {
            sessions: get_sessions(),
            session_state: state,
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());

        let tasks = self.sessions.iter().map(|x| Text::raw(x.name.clone()));

        let tasks = List::new(tasks)
            .block(Block::default().borders(Borders::ALL).title("Sessions"))
            .highlight_style(Style::default().fg(Color::Yellow).modifier(Modifier::BOLD))
            .highlight_symbol(">");

        f.render_stateful_widget(tasks, chunks[0], &mut self.session_state);
    }

    pub fn handle_event(&mut self, event: Event<Key>) -> bool {
        match event {
            Event::Input(key) => match key {
                Key::Up => self.session_state.select(Some({
                    let index = self.session_state.selected().unwrap();
                    if index == 0 {
                        self.sessions.len() - 1
                    } else {
                        index - 1
                    }
                })),
                Key::Down => self.session_state.select(Some(
                    (self.session_state.selected().unwrap() + 1) % self.sessions.len(),
                )),
                Key::Char('q') => return true,
                _ => (),
            },
        };
        false
    }
}
