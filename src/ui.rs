use super::event::Event;
use super::tmux::*;
use super::session::Session;

use termion::event::Key;

use tui::backend::Backend;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListState, Paragraph, Text};
use tui::Frame;

pub struct UI {
    sessions: Vec<Session>,
    filtered_sessions: Vec<Session>,
    session_state: ListState,
    current_search: String,
}

enum Action {
    Select,
    Delete,
}

impl UI {
    pub fn new() -> UI {
        let mut state = ListState::default();
        state.select(Some(0));
        let sessions = get_sessions();

        UI {
            sessions: sessions.clone(),
            filtered_sessions: sessions.clone(),
            session_state: state,
            current_search: String::from(""),
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .constraints(
                [
                    Constraint::Length(2),
                    Constraint::Min(0),
                    Constraint::Length(2),
                ]
                .as_ref(),
            )
            .split(f.size());

        let text = [
            Text::styled("<C-j>", Style::new().fg(Color::Red)),
            Text::raw(" Switch/Create, "),
            Text::styled("<C-d>", Style::new().fg(Color::Red)),
            Text::raw(" Delete"),
        ];

        let block = Block::default();
        let paragraph = Paragraph::new(text.iter()).block(block).wrap(true);
        f.render_widget(paragraph, chunks[0]);

        let sessions = self.filtered_sessions.iter().map(|x| {
            Text::raw(format!("{}", x))
        });

        let sessions = List::new(sessions)
            .block(Block::default().borders(Borders::ALL).title("Sessions"))
            .highlight_style(Style::default().fg(Color::Yellow).modifier(Modifier::BOLD))
            .highlight_symbol(">");

        f.render_stateful_widget(sessions, chunks[1], &mut self.session_state);

        let search_text = [
            Text::styled("Prompt: ", Style::default().fg(Color::Cyan)),
            Text::raw(self.current_search.as_str()),
        ];
        let search_text = Paragraph::new(search_text.iter()).block(block).wrap(true);
        f.render_widget(search_text, chunks[2]);
    }

    pub fn handle_event(&mut self, event: Event<Key>) -> bool {
        match event {
            Event::Input(key) => match key {
                Key::Ctrl('p') => self.session_state.select(Some({
                    let index = self.session_state.selected().unwrap();
                    if index == 0 {
                        self.filtered_sessions.len() - 1
                    } else {
                        index - 1
                    }
                })),
                Key::Ctrl('n') => self.session_state.select(Some(
                    (self.session_state.selected().unwrap() + 1) % self.filtered_sessions.len(),
                )),
                Key::Ctrl('g') => return true,
                Key::Ctrl('d') => self.do_selection(Action::Delete),

                Key::Char('\n') => {
                    self.do_selection(Action::Select);
                    return true;
                }
                Key::Char(a) => {
                    self.current_search.push(a);
                    self.filter_sessions();
                }
                Key::Backspace => {
                    if self.current_search.len() > 0 {
                        self.current_search.remove(self.current_search.len() - 1);
                    }
                    self.filter_sessions();
                    ()
                }
                _ => (),
            },
        };
        false
    }

    fn filter_sessions(&mut self) {
        if self.current_search.len() > 0 {
            self.filtered_sessions = self
                .sessions
                .iter()
                .filter_map(|x| {
                    if x.name.contains(&self.current_search) {
                        Some(x.clone())
                    } else {
                        None
                    }
                })
                .collect();

            self.filtered_sessions
                .push(Session::new(String::from(&self.current_search), true));
        } else {
            self.filtered_sessions = self.sessions.clone()
        }

        if self.filtered_sessions.len() > 0 {
            self.session_state.select(Some(0));
        } else {
            self.session_state.select(None);
        }
    }

    fn do_selection(&mut self, action: Action) {
        if self.session_state.selected().is_some() {
            let selected = self
                .filtered_sessions
                .get(self.session_state.selected().unwrap())
                .unwrap();

            match action {
                Action::Select => {
                    if selected.new {
                        new_session(&selected.name, true);
                    } else {
                        attach_session(&selected.name);
                    }
                }
                Action::Delete => {
                    delete_session(&selected.name);
                    self.sessions = get_sessions();
                    self.filter_sessions();
                }
            }
        }
    }
}
