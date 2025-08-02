Design Pattern
===

## Design Pattern Categories

This repository covers 23 classic design patterns, divided into three categories:

### Creational Patterns
- Singleton
- Factory Method
- Abstract Factory
- Builder
- Prototype

### Structural Patterns
- Adapter
- Bridge
- Composite
- Decorator
- Facade
- Flyweight
- Proxy

### Behavioral Patterns
- Chain of Responsibility
- Command
- Interpreter
- Iterator
- Mediator
- Memento
- Observer
- State
- Strategy
- Template Method
- Visitor

## Build and Run

This repository provides **dual-language** implementations with both **Rust** and **C++** examples. Each design pattern is implemented as an independent module.

### 🦀 Rust Examples

#### Build All Rust Examples

Run the following command in the project root directory:

```bash
cargo build
```

#### Run a Specific Rust Example

For example, to run singleton:

```bash
cargo run -p singleton
```

Replace `singleton` with the crate name you want to run.

### ⚡ C++ Examples

#### Build All C++ Examples

Run the following command in the project root directory:

```bash
make
```

#### Run a Specific C++ Example

For example, to run command pattern:

```bash
make run singleton
```

Replace `singleton` with the crate name you want to run.

#### Other C++ Commands

```bash
make list     # List all available C++ examples
make clean    # Clean C++ build files
make help     # Show help information
```

---

## 设计模式分类

本仓库涵盖了23种经典设计模式，分为三大类：

### 创建型模式
- 单例模式（Singleton）
- 工厂方法模式（Factory Method）
- 抽象工厂模式（Abstract Factory）
- 建造者模式（Builder）
- 原型模式（Prototype）

### 结构型模式
- 适配器模式（Adapter）
- 桥接模式（Bridge）
- 组合模式（Composite）
- 装饰模式（Decorator）
- 外观模式（Facade）
- 享元模式（Flyweight）
- 代理模式（Proxy）

### 行为型模式
- 责任链模式（Chain of Responsibility）
- 命令模式（Command）
- 解释器模式（Interpreter）
- 迭代器模式（Iterator）
- 中介者模式（Mediator）
- 备忘录模式（Memento）
- 观察者模式（Observer）
- 状态模式（State）
- 策略模式（Strategy）
- 模板方法模式（Template Method）
- 访问者模式（Visitor）

## 编译与运行

本仓库提供 **两种语言** 实现，包含 **Rust** 和 **C++** 示例。每个设计模式都实现为独立模块。

### 🦀 Rust 示例

#### 编译所有 Rust 示例

在项目根目录下执行：

```bash
cargo build
```

#### 运行某个 Rust 示例

以 singleton 为例：

```bash
cargo run -p singleton
```

将 `singleton` 替换为你想运行的 crate 名称即可。

### ⚡ C++ 示例

#### 编译所有 C++ 示例

在项目根目录下执行：

```bash
make
```

#### 运行某个 C++ 示例

以命令模式为例：

```bash
make run singleton
```

将 `singleton` 替换为你想运行的模式名称即可。

#### 其他 C++ 命令

```bash
make list     # 列出所有可用的 C++ 示例
make clean    # 清理 C++ 构建文件
make help     # 显示帮助信息
```