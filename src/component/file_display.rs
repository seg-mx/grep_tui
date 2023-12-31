use std::any::Any;

use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{self, Block, Borders, ListItem, ListState},
    Frame,
};

use crate::app::App;

use super::Component;

pub struct FileDisplay {
    state: ListState,
    items: Vec<(u32, String)>,
    title: String,
    focus_key: KeyCode,
}

impl FileDisplay {
    pub fn new(title: String, focus_key: KeyCode) -> Self {
        Self {
            state: ListState::default(),
            items: Vec::new(),
            title,
            focus_key,
        }
    }

    pub fn set_items(&mut self, items: Vec<(u32, String)>) {
        self.items = items;
    }

    fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i == self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }

            None => 0,
        };

        self.state.select(Some(i));
    }

    fn prev(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.items.len() - 1,
        };

        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

impl<B: Backend> Component<B> for FileDisplay {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn render(&mut self, f: &mut Frame<B>, area: Rect, is_focused: bool) {
        let KeyCode::Char(focus_key) = self.focus_key else {
            return;
        };

        let block = Block::default()
            .title(Spans::from(vec![
                Span::styled(
                    format!(" ({}) {}", self.items.len(), self.title),
                    Style::default().fg(Color::Cyan),
                ),
                Span::styled(
                    format!(" [{focus_key}] "),
                    Style::default().fg(Color::Yellow),
                ),
            ]))
            .borders(Borders::ALL)
            .style(Style::default().fg(if is_focused {
                Color::Blue
            } else {
                Color::White
            }));

        let numbers_len = self
            .items
            .last()
            .map(|(n, _)| n.to_string().len())
            .unwrap_or(0);

        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|i| {
                ListItem::new(Spans::from(vec![
                    Span::styled(
                        format!("{: >numbers_len$} | ", i.0),
                        Style::default().fg(Color::DarkGray),
                    ),
                    Span::raw(i.1.clone()),
                ]))
            })
            .collect();

        let items = widgets::List::new(items)
            .block(block)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().bg(Color::Gray).fg(Color::Black));

        f.render_stateful_widget(items, area, &mut self.state);
    }

    fn focus_key(&self) -> KeyCode {
        KeyCode::Char('r')
    }

    fn handle_key(&mut self, key: KeyCode, _: &mut App<B>) {
        match key {
            KeyCode::Esc => self.unselect(),
            KeyCode::Up | KeyCode::Char('k') => self.prev(),
            KeyCode::Down | KeyCode::Char('j') => self.next(),
            _ => (),
        }
    }
}
