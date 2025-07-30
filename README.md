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

This repository uses Rust workspace to manage all design patterns, with each pattern implemented as an independent crate.

### Build All Design Patterns

Run the following command in the project root directory:

```bash
cargo build
```

### Run a Specific Design Pattern

For example, to run singleton:

```bash
cargo run -p singleton
```

Replace `singleton` with the crate name you want to run.

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

本仓库采用 Rust workspace 管理所有设计模式，每个设计模式为独立 crate。

### 编译所有设计模式

在项目根目录下执行：

```bash
cargo build
```

### 运行某个设计模式

以 singleton 为例：

```bash
cargo run -p singleton
```

将 `singleton` 替换为你想运行的 crate 名称即可。