use crate::ui::draw;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::Rect,
    widgets::{List, Widget},
};
use std::collections::HashSet;
use std::io;

pub enum CurrentScreen {
    Search,
    SearchResults,
    Help,
}

pub struct App {
    pub repos: Vec<crate::search::Repo>,
    pub search_term: String,
    pub page_size: usize,
    pub total_pages: usize,
    pub current_page: usize,
    pub current_screen: CurrentScreen,
    pub current_index: usize,
    pub exit: bool,

    pub selected: HashSet<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            // repos: vec![],
            repos: vec![
                crate::search::Repo {
                    html_url: "https://github.com/rust-lang/rust".to_string(),
                    name: "Rust".to_string(),
                    description: Some("A safe, concurrent, practical language".to_string()),
                    stargazers_count: 100,
                    forks_count: 100,
                },
                crate::search::Repo {
                    html_url: "https://github.com/bevyengine/bevy".to_string(),
                    name: "Bevy".to_string(),
                    description: Some("A data-oriented game engine".to_string()),
                    stargazers_count: 100,
                    forks_count: 100,
                },
                crate::search::Repo {
                    html_url: "https://github.com/flippinberger/repo-depot".to_string(),
                    name: "Repo Depot".to_string(),
                    description: Some(
                        "A terminal UI for searching and selecting GitHub repositories".to_string(),
                    ),
                    stargazers_count: 100,
                    forks_count: 100,
                },
            ],
            search_term: String::new(),
            page_size: 10,
            current_page: 0,
            total_pages: 1,
            current_screen: CurrentScreen::Search,
            current_index: 0,
            exit: false,
            selected: HashSet::new(),
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| draw(frame, &self))?;
            self.handle_events().await?;
        }
        Ok(())
    }

    async fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(());
            }

            match self.current_screen {
                CurrentScreen::Search => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => self.exit(),
                    KeyCode::Char('?') => self.help(),
                    KeyCode::Char(c) => self.search_term.push(c),
                    KeyCode::Backspace => {
                        if !self.search_term.is_empty() {
                            self.search_term.pop();
                        }
                    }
                    KeyCode::Enter => {
                        self.repos = crate::search::search(&self.search_term).await.unwrap();
                        self.current_screen = CurrentScreen::SearchResults;
                    }
                    _ => {}
                },
                CurrentScreen::SearchResults => match key.code {
                    KeyCode::Char('q') => self.exit(),
                    KeyCode::Esc => self.current_screen = CurrentScreen::Search,
                    KeyCode::Char('?') => self.help(),
                    KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('a') => {
                        let last = self.current_page;
                        self.current_page = 0.max(self.current_page - 1);

                        if last != self.current_page {
                            self.current_index = 0;
                        }
                    }
                    KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('d') => {
                        let last = self.current_page;
                        self.current_page = self.total_pages.min(self.current_page + 1);

                        if last != self.current_page {
                            self.current_index = 0;
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('w') => {
                        let end = self
                            .repos
                            .len()
                            .min((self.current_page + 1) * self.page_size);
                        if self.current_index > 0 {
                            self.current_index -= 1;
                        } else {
                            self.current_index = end - 1;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('s') => {
                        let end = self
                            .repos
                            .len()
                            .min((self.current_page + 1) * self.page_size);
                        if self.current_index < end - 1 {
                            self.current_index += 1;
                        } else {
                            self.current_index = 0;
                        }
                    }
                    KeyCode::Tab => {
                        let index = self.current_index + (self.current_page * self.page_size);
                        let url = self.repos[index].html_url.clone();
                        if let Some(_) = self.selected.get(&url) {
                            self.selected.remove(&url);
                        } else {
                            self.selected.insert(url);
                        }
                    }
                    _ => {}
                },
                CurrentScreen::Help => match key.code {
                    KeyCode::Char('q') => self.exit(),
                    KeyCode::Char('?') => self.help(),
                    _ => {}
                },
            }
        }

        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn help(&mut self) {
        self.current_screen = CurrentScreen::Help;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        List::new(vec!["1. Hello", "2. World", "3. Exit"]).render(area, buf);
    }
}
