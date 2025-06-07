use crate::app::{App, CurrentScreen};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, List, Paragraph},
};

pub fn draw(frame: &mut Frame, app: &App) {
    match app.current_screen {
        CurrentScreen::Search => {
            // title, search bar, footer/hints
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(5),
                        Constraint::Length(1),
                        Constraint::Length(4),
                        Constraint::Min(1),
                    ]
                    .as_ref(),
                )
                .split(frame.area());

            let title = Paragraph::new(Text::styled(
                "Welcome to Repo Depot",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ));
            frame.render_widget(title, chunks[0]);

            let title = Paragraph::new(Text::styled(
                "Search for a repo or organization:",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ));
            frame.render_widget(title, chunks[1]);

            let key_block = Block::default().borders(Borders::ALL);

            let s = format!(" {}", app.search_term.clone());
            let search_bar = Paragraph::new(s).block(key_block);
            frame.render_widget(search_bar, chunks[2]);

            let footer = Paragraph::new(Text::styled(
                "Press ? for help",
                Style::default().fg(Color::White),
            ));
            frame.render_widget(footer, chunks[3]);
        }
        CurrentScreen::SearchResults => {
            // title, list, footer/hints
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(5),
                        Constraint::Min(1),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .split(frame.area());

            let title = Paragraph::new(Text::styled(
                format!("Search results for: {}", app.search_term),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::UNDERLINED),
            ));
            frame.render_widget(title, chunks[0]);

            // if the page is the last page, we need to get the currect end index
            let end = app.repos.len().min((app.current_page + 1) * app.page_size);
            let repo_page = &app.repos[app.current_page * app.page_size..end];

            // let list = List::new(vec!["1. Hello", "2. World", "3. Exit"]);
            let mut list = Vec::new();
            for (i, repo) in repo_page.iter().enumerate() {
                let mut style = Style::default().fg(Color::White);
                if app.current_index == i {
                    style = style.fg(Color::LightBlue).add_modifier(Modifier::BOLD);
                }

                let selected = app.selected.contains(&repo.html_url);

                let s = format!(
                    " {} {}. {}",
                    if selected { "*" } else { " " },
                    (i + 1) + (app.current_page * app.page_size),
                    repo.name.clone()
                );

                let text = Text::styled(s, style);
                // if let Some(description) = repo.description.clone() {
                //     text.append(Text::styled(description, Style::default().fg(Color::White)));
                // }
                list.push(text);
            }

            frame.render_widget(List::new(list), chunks[1]);

            let footer = Paragraph::new(Text::styled(
                "Press ? for help",
                Style::default().fg(Color::White),
            ));
            frame.render_widget(footer, chunks[2]);
        }
        CurrentScreen::Help => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(frame.area());
        }
        _ => {}
    }
}
