use super::session::Session;
use super::tmux::*;
use crate::config::Config;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use std::error::Error;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Modifier, Style};
use tui::widgets::{Block, Borders, List, ListState, Text};
use tui::Frame;

enum Action {
    Select,
    Delete,
}

pub struct SessionList<'a> {
    sessions: Vec<Session>,
    filtered_sessions: Vec<Session>,
    session_state: ListState,
    last_search: String,
    config: &'a Config,
}

impl SessionList<'_> {
    pub fn new<'a>(config: &'a Config) -> Result<SessionList<'a>, Box<dyn Error>> {
        let mut state = ListState::default();
        state.select(Some(0));
        let sessions = get_sessions(&config)?;

        Ok(SessionList {
            sessions: sessions.clone(),
            filtered_sessions: sessions.clone(),
            session_state: state,
            last_search: String::from(""),
            config,
        })
    }

    pub fn draw<B: Backend>(&mut self, region: Rect, f: &mut Frame<B>) {
        let sessions = self
            .filtered_sessions
            .iter()
            .map(|x| Text::raw(format!("{}", x)));

        let sessions = List::new(sessions)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Sessions")
                    .title_style(Style::default().fg(self.config.theme.get_session_list_border()))
                    .border_style(Style::default().fg(self.config.theme.get_session_list_border())),
            )
            .style(Style::default().fg(self.config.theme.get_session_foreground()))
            .highlight_style(
                Style::default()
                    .fg(self.config.theme.get_highlight_foreground())
                    .modifier(Modifier::BOLD),
            )
            .highlight_symbol(">");

        f.render_stateful_widget(sessions, region, &mut self.session_state);
    }

    pub fn filter_sessions(&mut self, search: &String) {
        if search.len() > 0 {
            let matcher = SkimMatcherV2::default();
            self.filtered_sessions = self
                .sessions
                .iter()
                .cloned()
                .filter(|x| matcher.fuzzy_match(&x.name, search).is_some())
                .collect();

            self.filtered_sessions
                .push(Session::new(String::from(search), true));
        } else {
            self.filtered_sessions = self.sessions.clone()
        }

        if self.filtered_sessions.len() > 0 {
            self.session_state.select(Some(0));
        } else {
            self.session_state.select(None);
        }

        self.last_search = search.clone();
    }

    pub fn select_session(&mut self) -> Result<(), Box<dyn Error>> {
        self.do_selection(Action::Select)?;
        Ok(())
    }

    pub fn delete_session(&mut self) -> Result<(), Box<dyn Error>> {
        self.do_selection(Action::Delete)?;
        Ok(())
    }

    pub fn next(&mut self) {
        self.session_state.select(Some(
            (self.session_state.selected().unwrap() + 1) % self.filtered_sessions.len(),
        ));
    }

    pub fn previous(&mut self) {
        let index = self.session_state.selected().unwrap();
        let selected = if index == 0 {
            self.filtered_sessions.len() - 1
        } else {
            index - 1
        };
        self.session_state.select(Some(selected));
    }

    fn do_selection(&mut self, action: Action) -> Result<(), Box<dyn Error>> {
        if self.session_state.selected().is_some() {
            let selected = self
                .filtered_sessions
                .get(self.session_state.selected().unwrap())
                .unwrap();

            match action {
                Action::Select => {
                    if selected.new {
                        new_session(&selected.name, true, self.config);
                    } else {
                        attach_session(&selected.name);
                    }
                }
                Action::Delete => {
                    delete_session(&selected.name);

                    self.sessions = get_sessions(self.config)?;
                    let last = self.last_search.clone();
                    self.filter_sessions(&last);
                }
            }
        }
        Ok(())
    }
}
