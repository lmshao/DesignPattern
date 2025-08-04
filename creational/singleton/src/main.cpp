/**
 * @file main.cpp
 * @brief Singleton Pattern Example - C++ Implementation
 * @author SHAO Liming <lmshao@163.com>
 * @copyright Copyright © 2025 SHAO Liming <lmshao@163.com>
 * @license MIT
 *
 * The singleton pattern ensures that a class has only one instance and provides
 * a global point of access to that instance. This is useful for coordinating
 * actions across the system.
 *
 * SPDX-License-Identifier: MIT
 */

#include <iostream>
#include <string>

class Logger {

public:
    static Logger &getInstance()
    {
        // C++11 guarantees thread-safe initialization of static local variables
        static Logger instance;
        return instance;
    }

    const std::string &getPrefix() const { return prefix_; }

private:
    Logger() : prefix_("[Singleton]") {}

    Logger(const Logger &) = delete;
    Logger &operator=(const Logger &) = delete;

    std::string prefix_;
};

int main()
{
    auto &logger1 = Logger::getInstance();
    auto &logger2 = Logger::getInstance();

    std::cout << logger1.getPrefix() << " Hello, world!" << std::endl;
    std::cout << "logger1 address: " << &logger1 << std::endl;
    std::cout << "logger2 address: " << &logger2 << std::endl;
    std::cout << "Is same instance: " << (&logger1 == &logger2 ? "true" : "false") << std::endl;
    return 0;
}