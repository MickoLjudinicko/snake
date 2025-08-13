# 🐍 Snake Game in Rust

A classic Snake game implementation written in Rust, featuring a terminal-based user interface with sound effects, background music, and multiple difficulty levels.

## ✨ Features

- **Classic Snake Gameplay**: Navigate your snake to eat food and grow longer while avoiding walls and self-collision
- **Multiple Difficulty Levels**: Choose from Easy, Medium, or Hard difficulty settings
- **Audio Experience**:
  - Background music during gameplay
  - Sound effects for eating food and game over
  - Configurable sound settings (music and effects can be toggled independently)
- **Intuitive Menu System**: Easy-to-navigate menus for game options
- **Cross-platform Terminal Interface**: Runs in any terminal that supports crossterm
- **Autopilot Mode**: AI-controlled snake for demonstration purposes

## 🎮 Controls

### In Game

- **Arrow Keys** or **WASD**: Move the snake
  - `↑` or `W`: Move Up
  - `↓` or `S`: Move Down  
  - `←` or `A`: Move Left
  - `→` or `D`: Move Right

### In Menus

- `↑/↓ Arrow Keys`: Navigate menu options
- `Enter`: Select menu item
- `Q`: Quit game or return to previous menu

## 🚀 Installation

### Prerequisites

- Rust (latest stable version)
- Cargo package manager

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd snake

# Build the project
cargo build --release

# Run the game
cargo run
```

## 🎯 How to Play

1. **Start the Game**: Run the executable and select "Play" from the main menu
2. **Choose Difficulty**: Select your preferred difficulty level:
   - **Easy**: Slower snake movement (150ms delay)
   - **Medium**: Normal snake movement (100ms delay)  
   - **Hard**: Faster snake movement (50ms delay)
3. **Configure Audio**: Toggle background music and sound effects on/off
4. **Gameplay**:
   - Use arrow keys or WASD to control your snake
   - Eat the food (`*`) to grow and increase your score
   - Avoid hitting the walls (blue border) or your own body (`@`)
   - Try to achieve the highest score possible!

## 🏗️ Project Structure

```text
src/
├── main.rs           # Application entry point and menu handling
├── game.rs           # Core game logic and loop
├── snake.rs          # Snake entity and movement logic
├── food.rs           # Food generation and positioning
├── input.rs          # Input handling and controls
├── direction.rs      # Direction enum and logic
├── coordinate.rs     # 2D coordinate system
├── menu.rs           # Main menu implementation
├── difficulty_menu.rs # Difficulty selection menu
├── sound_menu.rs     # Audio settings menu
├── sound.rs          # Sound effects system
├── music.rs          # Background music system
├── constants.rs      # Game constants (board size, etc.)
└── lib.rs           # Library configuration
```

## 🔧 Dependencies

- **`tui`**: Terminal user interface library for menus
- **`crossterm`**: Cross-platform terminal manipulation
- **`rand`**: Random number generation for food placement
- **`rodio`**: Audio playback for sound effects and music

## 🎨 Game Display

The game features a colorful terminal display:

- 🟦 **Blue borders**: Game boundaries
- 🟨 **Yellow '@' symbols**: Snake body segments
- 🔴 **Red '*' symbol**: Food items
- ⚫ **Black background**: Empty game area

## 🤖 Technical Features

- **Collision Detection**: Advanced collision detection for walls and self-collision
- **Smooth Animation**: Consistent frame rate with configurable difficulty-based timing
- **Memory Safe**: Written in Rust with no unsafe code blocks
- **Modular Design**: Well-structured codebase with separated concerns
- **Cross-platform Audio**: Works on Windows, macOS, and Linux

## 🎵 Audio System

The game includes a complete audio system:

- **Background Music**: Themed music that loops during gameplay
- **Sound Effects**:
  - Food consumption sound (440Hz tone, 200ms)
  - Game over sound (220Hz tone, 500ms)
- **Audio Controls**: Independent toggles for music and sound effects

## 🏆 Scoring

- Each piece of food consumed increases your score by 1
- Your final score is displayed when the game ends
- Challenge yourself to beat your high score across different difficulty levels!

## 🐛 Contributing

Feel free to contribute to this project by:

- Reporting bugs
- Suggesting new features
- Submitting pull requests
- Improving documentation

## 📝 License

This project is open source. Please check the license file for more details.

## 🙏 Acknowledgments

- Built with the amazing Rust ecosystem
- Inspired by the classic Snake game
- Uses modern terminal UI libraries for a great user experience

---

Enjoy the game! 🐍✨
