
/**
 * @file main.cpp
 * @brief Command Pattern Example - C++ Implementation
 * @author SHAO Liming <lmshao@163.com>
 * @copyright Copyright © 2025 SHAO Liming
 * @license MIT
 *
 * The command pattern encapsulates a request as an object, thereby letting you
 * parameterize clients with different requests and support undoable operations.
 *
 * SPDX-License-Identifier: MIT
 */

#include <iostream>
#include <memory>
#include <string>
#include <vector>

// Receiver
class Light {
public:
    Light() : is_on_(false), brightness_(0) {}

    void turnOn()
    {
        is_on_ = true;
        brightness_ = 100;
        std::cout << "💡 Light is ON (brightness: " << brightness_ << ")" << std::endl;
    }

    void turnOff()
    {
        is_on_ = false;
        brightness_ = 0;
        std::cout << "🌑 Light is OFF" << std::endl;
    }

    void status() const
    {
        if (is_on_) {
            std::cout << "💡 Light Status: ON (brightness: " << brightness_ << ")" << std::endl;
        } else {
            std::cout << "🌑 Light Status: OFF" << std::endl;
        }
    }

private:
    bool is_on_;
    int brightness_;
};

// Command interface
class Command {
public:
    virtual ~Command() = default;
    virtual void execute() = 0;
    virtual void undo() = 0;
    virtual std::string getName() const = 0;
};

// Concrete Command: TurnOn
class TurnOnCommand : public Command {
public:
    TurnOnCommand(std::shared_ptr<Light> light) : light_(light), executed_(false) {}
    void execute() override
    {
        if (!executed_) {
            light_->turnOn();
            executed_ = true;
        }
    }
    void undo() override
    {
        if (executed_) {
            light_->turnOff();
            executed_ = false;
        }
    }
    std::string getName() const override { return "Turn On Light"; }

private:
    std::shared_ptr<Light> light_;
    bool executed_;
};

// Concrete Command: TurnOff
class TurnOffCommand : public Command {
public:
    TurnOffCommand(std::shared_ptr<Light> light) : light_(light), executed_(false) {}

    void execute() override
    {
        if (!executed_) {
            light_->turnOff();
            executed_ = true;
        }
    }

    void undo() override
    {
        if (executed_) {
            light_->turnOn();
            executed_ = false;
        }
    }
    std::string getName() const override { return "Turn Off Light"; }

private:
    std::shared_ptr<Light> light_;
    bool executed_;
};

// Invoker
class RemoteControl {
public:
    void pressButton(std::unique_ptr<Command> command)
    {
        std::cout << "🔘 Pressing button: " << command->getName() << std::endl;
        command->execute();
        last_command_ = std::move(command);
    }

    void pressUndo()
    {
        if (last_command_) {
            std::cout << "↩️ Pressing UNDO button" << std::endl;
            last_command_->undo();
            last_command_.reset();
        } else {
            std::cout << "❌ No command to undo" << std::endl;
        }
    }

private:
    std::unique_ptr<Command> last_command_;
};

int main()
{
    std::cout << "🔘 Command Pattern Example - Smart Light Remote" << std::endl;
    std::cout << std::string(45, '=') << std::endl;

    auto light = std::make_shared<Light>();
    RemoteControl remote;

    std::cout << "📱 Initial state:" << std::endl;
    light->status();
    std::cout << std::endl;

    // Create commands
    auto turn_on_cmd = std::make_unique<TurnOnCommand>(light);
    auto turn_off_cmd = std::make_unique<TurnOffCommand>(light);

    // Test normal operations
    std::cout << "🔄 Testing normal operations:" << std::endl;
    std::cout << std::string(25, '-') << std::endl;

    remote.pressButton(std::move(turn_on_cmd));
    light->status();
    std::cout << std::endl;

    turn_off_cmd = std::make_unique<TurnOffCommand>(light);
    remote.pressButton(std::move(turn_off_cmd));
    light->status();
    std::cout << std::endl;

    // Test undo functionality
    std::cout << "🔄 Testing undo functionality:" << std::endl;
    std::cout << std::string(25, '-') << std::endl;

    remote.pressUndo(); // Should turn light back on
    light->status();
    std::cout << std::endl;

    remote.pressUndo(); // Should turn light back off
    light->status();
    std::cout << std::endl;

    remote.pressUndo(); // No command to undo
    std::cout << std::endl;

    std::cout << "✅ Command Pattern example completed!" << std::endl;
    std::cout << std::endl;
    std::cout << "💡 Key Points:" << std::endl;
    std::cout << "  • Command interface defines execute() and undo() interface" << std::endl;
    std::cout << "  • TurnOnCommand/TurnOffCommand are concrete commands" << std::endl;
    std::cout << "  • Light is the receiver that performs actual operations" << std::endl;
    std::cout << "  • RemoteControl is the invoker that manages commands" << std::endl;
    std::cout << "  • Commands can be executed and undone independently" << std::endl;
}