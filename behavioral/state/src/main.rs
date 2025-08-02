// Copyright © 2025 SHAO Liming <lmshao@163.com>
// Licensed under the MIT License
//
// State Pattern Example
//
// The state pattern allows an object to alter its behavior when its internal
// state changes. The object will appear to change its class.

/// State machine error type
#[derive(Debug)]
enum StateError {
    InvalidOperation(String),
}

impl std::fmt::Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
        }
    }
}

impl std::error::Error for StateError {}

/// PlayerState trait - defines the interface for states
trait PlayerState {
    fn play(&self) -> Result<Box<dyn PlayerState>, StateError>;
    fn pause(&self) -> Result<Box<dyn PlayerState>, StateError>;
    fn stop(&self) -> Result<Box<dyn PlayerState>, StateError>;
    fn get_state_name(&self) -> &str;
}

/// StoppedState - Concrete State
struct StoppedState;

impl PlayerState for StoppedState {
    fn play(&self) -> Result<Box<dyn PlayerState>, StateError> {
        println!("▶️  Starting music playback");
        Ok(Box::new(PlayingState))
    }

    fn pause(&self) -> Result<Box<dyn PlayerState>, StateError> {
        Err(StateError::InvalidOperation(
            "Cannot pause when stopped".to_string(),
        ))
    }

    fn stop(&self) -> Result<Box<dyn PlayerState>, StateError> {
        Err(StateError::InvalidOperation("Already stopped".to_string()))
    }

    fn get_state_name(&self) -> &str {
        "Stopped"
    }
}

/// PlayingState - Concrete State
struct PlayingState;

impl PlayerState for PlayingState {
    fn play(&self) -> Result<Box<dyn PlayerState>, StateError> {
        Err(StateError::InvalidOperation("Already playing".to_string()))
    }

    fn pause(&self) -> Result<Box<dyn PlayerState>, StateError> {
        println!("⏸️  Pausing playback");
        Ok(Box::new(PausedState))
    }

    fn stop(&self) -> Result<Box<dyn PlayerState>, StateError> {
        println!("⏹️  Stopping playback");
        Ok(Box::new(StoppedState))
    }

    fn get_state_name(&self) -> &str {
        "Playing"
    }
}

/// PausedState - Concrete State  
struct PausedState;

impl PlayerState for PausedState {
    fn play(&self) -> Result<Box<dyn PlayerState>, StateError> {
        println!("▶️  Resuming playback");
        Ok(Box::new(PlayingState))
    }

    fn pause(&self) -> Result<Box<dyn PlayerState>, StateError> {
        Err(StateError::InvalidOperation("Already paused".to_string()))
    }

    fn stop(&self) -> Result<Box<dyn PlayerState>, StateError> {
        println!("⏹️  Stopping playback");
        Ok(Box::new(StoppedState))
    }

    fn get_state_name(&self) -> &str {
        "Paused"
    }
}

/// MusicPlayer - Context that manages state
struct MusicPlayer {
    current_state: Box<dyn PlayerState>,
    song_name: String,
}

impl MusicPlayer {
    fn new(song_name: String) -> Self {
        Self {
            current_state: Box::new(StoppedState),
            song_name,
        }
    }

    fn play(&mut self) -> Result<(), StateError> {
        println!(
            "🎵 Song: {} | Current state: {}",
            self.song_name,
            self.current_state.get_state_name()
        );
        match self.current_state.play() {
            Ok(new_state) => {
                self.current_state = new_state;
                println!(
                    "   ➡️  New state: {}\n",
                    self.current_state.get_state_name()
                );
                Ok(())
            }
            Err(e) => {
                println!("   ❌ Error: {}\n", e);
                Err(e)
            }
        }
    }

    fn pause(&mut self) -> Result<(), StateError> {
        println!(
            "🎵 Song: {} | Current state: {}",
            self.song_name,
            self.current_state.get_state_name()
        );
        match self.current_state.pause() {
            Ok(new_state) => {
                self.current_state = new_state;
                println!(
                    "   ➡️  New state: {}\n",
                    self.current_state.get_state_name()
                );
                Ok(())
            }
            Err(e) => {
                println!("   ❌ Error: {}\n", e);
                Err(e)
            }
        }
    }

    fn stop(&mut self) -> Result<(), StateError> {
        println!(
            "🎵 Song: {} | Current state: {}",
            self.song_name,
            self.current_state.get_state_name()
        );
        match self.current_state.stop() {
            Ok(new_state) => {
                self.current_state = new_state;
                println!(
                    "   ➡️  New state: {}\n",
                    self.current_state.get_state_name()
                );
                Ok(())
            }
            Err(e) => {
                println!("   ❌ Error: {}\n", e);
                Err(e)
            }
        }
    }

    fn get_current_state(&self) -> &str {
        self.current_state.get_state_name()
    }
}

fn main() {
    println!("🎵 State Pattern Example - Music Player");
    println!("{}", "=".repeat(40));

    let mut player = MusicPlayer::new("Jay Chou - Blue and White Porcelain".to_string());

    println!("📱 Initial state: {}\n", player.get_current_state());

    // Test normal playback flow
    println!("🔄 Normal playback flow:");
    println!("{}", "-".repeat(20));
    player.play().unwrap(); // Stopped → Playing
    player.pause().unwrap(); // Playing → Paused
    player.play().unwrap(); // Paused → Playing
    player.stop().unwrap(); // Playing → Stopped

    // Test invalid operations
    println!("🔄 Test invalid operations:");
    println!("{}", "-".repeat(20));

    // Use if let to handle errors without panic
    if let Err(e) = player.stop() {
        println!("🚫 Caught error: {}", e);
    }

    if let Err(e) = player.pause() {
        println!("🚫 Caught error: {}", e);
    }

    // Normal flow again
    println!("🔄 Play again:");
    println!("{}", "-".repeat(20));
    player.play().unwrap(); // Stopped → Playing

    // Use match to handle duplicate play
    match player.play() {
        Ok(()) => println!("✅ Play successful"),
        Err(e) => println!("🚫 Play failed: {}", e),
    }

    player.pause().unwrap(); // Playing → Paused

    // Test duplicate pause
    if let Err(e) = player.pause() {
        println!("🚫 Duplicate pause failed: {}", e);
    }

    println!("✅ State Pattern example completed!");
    println!();
    println!("💡 Design Pattern Key Points:");
    println!("  - PlayerState trait defines the state interface");
    println!("  - StoppedState, PlayingState, PausedState are concrete states");
    println!("  - MusicPlayer is the context that manages current state");
    println!("  - Same operations have different behaviors in different states");
    println!("  - Invalid state transitions return errors instead of silent handling");
    println!("  - State transition logic is encapsulated in each state class");
}
