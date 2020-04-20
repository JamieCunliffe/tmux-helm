use super::event::Event;
use super::session_list::SessionList;
use crate::config::Config;

use termion::event::Key;

use std::error::Error;
use tui::backend::Backend;
use tui::layout::{Constraint, Layout};
use tui::style::Style;
use tui::widgets::{Block, Paragraph, Text};
use tui::Frame;

pub struct UI<'a> {
    session_list: SessionList<'a>,
    current_search: String,
    config: &'a Config,
}

impl UI<'_> {
    pub fn new(config: &Config) -> Result<UI, Box<dyn Error>> {
        Ok(UI {
            session_list: SessionList::new(config)?,
            current_search: String::from(""),
            config,
        })
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
            Text::styled("<C-j>", Style::new().fg(self.config.theme.get_help_binding_foreground())),
            Text::raw(" Switch/Create, "),
            Text::styled("<C-d>", Style::new().fg(self.config.theme.get_help_binding_foreground())),
            Text::raw(" Delete"),
        ];

        let block = Block::default();
        let paragraph = Paragraph::new(text.iter()).block(block).wrap(true);
        f.render_widget(paragraph, chunks[0]);

        self.session_list.draw(chunks[1], f);

        let search_text = [
            Text::styled("Prompt: ", Style::default().fg(self.config.theme.get_prompt_foreground())),
            Text::styled(self.current_search.as_str(), Style::default().fg(self.config.theme.get_prompt_input_foreground())),
        ];
        let search_text = Paragraph::new(search_text.iter()).block(block).wrap(true);
        f.render_widget(search_text, chunks[2]);
    }

    pub fn handle_event(&mut self, event: Event<Key>) -> Result<bool, Box<dyn Error>> {
        match event {
            Event::Input(key) => match key {
                Key::Ctrl('p') => self.session_list.previous(),
                Key::Ctrl('n') => self.session_list.next(),
                Key::Ctrl('g') => return Ok(true),
                Key::Ctrl('d') => self.session_list.delete_session()?,

                Key::Char('\n') => {
                    self.session_list.select_session()?;
                    return Ok(true);
                }
                Key::Char(a) => {
                    self.current_search.push(a);
                    self.session_list.filter_sessions(&self.current_search);
                }
                Key::Backspace => {
                    if self.current_search.len() > 0 {
                        self.current_search.remove(self.current_search.len() - 1);
                    }
                    self.session_list.filter_sessions(&self.current_search);
                    ()
                }
                _ => (),
            },
        };
        Ok(false)
    }
}
