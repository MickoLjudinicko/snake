mod constants;
mod coordinate;
mod difficulty_menu;
mod direction;
mod food;
mod game;
mod input;
mod menu;
mod music;
mod snake;
mod sound;
mod sound_menu;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use difficulty_menu::DifficultyMenu;
use game::{Game, GameDifficulty};
use menu::{Menu, MenuItem};
use sound_menu::SoundMenu;
use std::{error::Error, io};
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut menu = Menu::new();
    let mut game_difficulty = GameDifficulty::MEDIUM;
    let mut sound_enabled = true;
    let mut music_enabled = true;
    let autopilot = false;

    loop {
        terminal.draw(|f| menu.render(f))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Up => menu.previous(),
                KeyCode::Down => menu.next(),
                KeyCode::Enter => {
                    match menu.get_selected() {
                        Some(MenuItem::Play) => {
                            let mut game =
                                Game::new(game_difficulty, autopilot, sound_enabled, music_enabled);

                            disable_raw_mode()?;
                            execute!(
                                terminal.backend_mut(),
                                LeaveAlternateScreen,
                                DisableMouseCapture
                            )?;

                            let is_game_over = game.run();

                            enable_raw_mode()?;
                            execute!(
                                terminal.backend_mut(),
                                EnterAlternateScreen,
                                EnableMouseCapture
                            )?;

                            if is_game_over {
                                break;
                            }
                        }
                        Some(MenuItem::Difficulty) => {
                            // Implement difficulty selection logic
                            game_difficulty = select_difficulty(&mut terminal)?;
                        }
                        Some(MenuItem::Sound) => {
                            // Implement sound toggle logic
                            toggle_sound(&mut terminal, &mut sound_enabled, &mut music_enabled)?;
                        }
                        Some(MenuItem::Quit) => break,
                        None => {}
                    }
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(())
}

fn select_difficulty(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<GameDifficulty, Box<dyn Error>> {
    let mut difficulty_menu = DifficultyMenu::new();

    loop {
        terminal.draw(|f| difficulty_menu.render(f))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up => difficulty_menu.previous(),
                KeyCode::Down => difficulty_menu.next(),
                KeyCode::Enter => {
                    if let Some(selected_difficulty) = difficulty_menu.get_selected_speed() {
                        return Ok(selected_difficulty);
                    }
                }
                KeyCode::Char('q') => break, // Exit if 'q' is pressed
                _ => {}
            }
        }
    }

    Ok(GameDifficulty::MEDIUM) // Default to medium difficulty
}

fn toggle_sound(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    sound_enabled: &mut bool,
    music_enabled: &mut bool,
) -> Result<(), Box<dyn Error>> {
    let mut sound_menu = SoundMenu::new(*sound_enabled, *music_enabled);

    loop {
        terminal.draw(|f| sound_menu.render(f))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up => sound_menu.previous(),
                KeyCode::Down => sound_menu.next(),
                KeyCode::Enter => {
                    sound_menu.toggle_selection();
                    *sound_enabled = sound_menu.sound_enabled;
                    *music_enabled = sound_menu.music_enabled;
                }
                KeyCode::Char('q') => break, // Exit if 'q' is pressed
                _ => {}
            }
        }
    }

    Ok(())
}
