// Copyright © 2025 SHAO Liming <lmshao@163.com>
// Licensed under the MIT License
//
// Command Pattern Example - Simplified
//
// The command pattern encapsulates a request as an object, thereby letting you
// parameterize clients with different requests and support undoable operations.

use std::cell::RefCell;
use std::rc::Rc;

/// Command trait - defines the interface for all commands
trait Command {
    fn execute(&mut self);
    fn undo(&mut self);
    fn get_name(&self) -> &str;
}

/// Light - Receiver that performs the actual operations
struct Light {
    is_on: bool,
    brightness: u8,
}

impl Light {
    fn new() -> Self {
        Self {
            is_on: false,
            brightness: 0,
        }
    }

    fn turn_on(&mut self) {
        self.is_on = true;
        self.brightness = 100;
        println!("💡 Light is ON (brightness: {})", self.brightness);
    }

    fn turn_off(&mut self) {
        self.is_on = false;
        self.brightness = 0;
        println!("🌑 Light is OFF");
    }

    fn status(&self) {
        if self.is_on {
            println!("💡 Light Status: ON (brightness: {})", self.brightness);
        } else {
            println!("🌑 Light Status: OFF");
        }
    }
}

/// TurnOnCommand - Concrete Command
struct TurnOnCommand {
    light: Rc<RefCell<Light>>,
    executed: bool,
}

impl TurnOnCommand {
    fn new(light: Rc<RefCell<Light>>) -> Self {
        Self {
            light,
            executed: false,
        }
    }
}

impl Command for TurnOnCommand {
    fn execute(&mut self) {
        if !self.executed {
            self.light.borrow_mut().turn_on();
            self.executed = true;
        }
    }

    fn undo(&mut self) {
        if self.executed {
            self.light.borrow_mut().turn_off();
            self.executed = false;
        }
    }

    fn get_name(&self) -> &str {
        "Turn On Light"
    }
}

/// TurnOffCommand - Concrete Command
struct TurnOffCommand {
    light: Rc<RefCell<Light>>,
    executed: bool,
}

impl TurnOffCommand {
    fn new(light: Rc<RefCell<Light>>) -> Self {
        Self {
            light,
            executed: false,
        }
    }
}

impl Command for TurnOffCommand {
    fn execute(&mut self) {
        if !self.executed {
            self.light.borrow_mut().turn_off();
            self.executed = true;
        }
    }

    fn undo(&mut self) {
        if self.executed {
            self.light.borrow_mut().turn_on();
            self.executed = false;
        }
    }

    fn get_name(&self) -> &str {
        "Turn Off Light"
    }
}

/// RemoteControl - Invoker that manages commands
struct RemoteControl {
    last_command: Option<Box<dyn Command>>,
}

impl RemoteControl {
    fn new() -> Self {
        Self {
            last_command: None,
        }
    }

    fn press_button(&mut self, mut command: Box<dyn Command>) {
        println!("🔘 Pressing button: {}", command.get_name());
        command.execute();
        self.last_command = Some(command);
    }

    fn press_undo(&mut self) {
        if let Some(mut command) = self.last_command.take() {
            println!("↩️ Pressing UNDO button");
            command.undo();
        } else {
            println!("❌ No command to undo");
        }
    }
}

fn main() {
    println!("🔘 Command Pattern Example - Smart Light Remote");
    println!("{}", "=".repeat(45));

    // Create light (receiver) and remote control (invoker)
    let light = Rc::new(RefCell::new(Light::new()));
    let mut remote = RemoteControl::new();

    println!("📱 Initial state:");
    light.borrow().status();
    println!();

    // Create commands
    let turn_on_cmd = TurnOnCommand::new(light.clone());
    let turn_off_cmd = TurnOffCommand::new(light.clone());

    // Test normal operations
    println!("🔄 Testing normal operations:");
    println!("{}", "-".repeat(25));

    remote.press_button(Box::new(turn_on_cmd));
    light.borrow().status();
    println!();

    remote.press_button(Box::new(turn_off_cmd));
    light.borrow().status();
    println!();

    // Test undo functionality
    println!("🔄 Testing undo functionality:");
    println!("{}", "-".repeat(25));

    remote.press_undo(); // Should turn light back on
    light.borrow().status();
    println!();

    remote.press_undo(); // Should turn light back off
    light.borrow().status();
    println!();

    remote.press_undo(); // No command to undo
    println!();

    println!("✅ Command Pattern example completed!");
    println!();
    println!("💡 Key Points:");
    println!("  • Command trait defines execute() and undo() interface");
    println!("  • TurnOnCommand/TurnOffCommand are concrete commands");
    println!("  • Light is the receiver that performs actual operations");
    println!("  • RemoteControl is the invoker that manages commands");
    println!("  • Commands can be executed and undone independently");
}
