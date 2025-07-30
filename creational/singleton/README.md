# 单例模式（Singleton Pattern）

## 设计逻辑说明

本示例通过 Rust 的 OnceLock 实现了线程安全的单例模式。

- **核心结构体**：Logger，包含一个静态字符串前缀。
- **全局唯一实例**：使用 static LOGGER: OnceLock<Logger>，保证全局只初始化一次。
- **获取实例方法**：get_logger()，通过 get_or_init 实现惰性初始化。
- **main 函数**：多次获取 Logger 实例，打印其地址和判断是否为同一个实例。

## 运行效果

运行后会看到 logger1 和 logger2 地址相同，证明单例模式生效。

## 适用场景

需要全局唯一对象且线程安全时，推荐使用 OnceLock 实现单例。
