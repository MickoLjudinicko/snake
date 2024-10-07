use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

pub struct DifficultyMenu {
    items: Vec<(&'static str, u8)>, // Difficulty name and speed
    state: ListState,
}

impl DifficultyMenu {
    pub fn new() -> Self {
        Self {
            items: vec![
                ("Easy", 100),  // Easy mode with slow speed
                ("Medium", 50), // Medium mode with normal speed
                ("Hard", 25),   // Hard mode with fast speed
            ],
            state: ListState::default(),
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(2),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(f.size());

        let title = Spans::from(vec![Span::styled(
            "Select Difficulty",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )]);

        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|(name, _)| ListItem::new(Spans::from(*name)))
            .collect();

        let menu = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(title))
            .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black))
            .highlight_symbol("> ");

        f.render_stateful_widget(menu, chunks[1], &mut self.state);
    }

    pub fn get_selected_speed(&self) -> Option<u8> {
        self.state.selected().map(|i| self.items[i].1)
    }
}
