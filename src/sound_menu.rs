use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

pub struct SoundMenu {
    items: Vec<&'static str>,
    state: ListState,
    pub sound_enabled: bool,
    pub music_enabled: bool,
}

impl SoundMenu {
    pub fn new(sound_enabled: bool, music_enabled: bool) -> Self {
        Self {
            items: vec![
                if sound_enabled {
                    "Sound: On"
                } else {
                    "Sound: Off"
                },
                if music_enabled {
                    "Music: On"
                } else {
                    "Music: Off"
                },
            ],
            state: ListState::default(),
            sound_enabled,
            music_enabled,
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

    pub fn toggle_selection(&mut self) {
        if let Some(i) = self.state.selected() {
            match i {
                0 => {
                    self.sound_enabled = !self.sound_enabled;
                    self.items[0] = if self.sound_enabled {
                        "Sound: On"
                    } else {
                        "Sound: Off"
                    };
                }
                1 => {
                    self.music_enabled = !self.music_enabled;
                    self.items[1] = if self.music_enabled {
                        "Music: On"
                    } else {
                        "Music: Off"
                    };
                }
                _ => {}
            }
        }
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
            "Sound & Music Settings",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )]);

        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|&item| ListItem::new(Spans::from(item)))
            .collect();

        let menu = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(title))
            .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black))
            .highlight_symbol("> ");

        f.render_stateful_widget(menu, chunks[1], &mut self.state);
    }
}
