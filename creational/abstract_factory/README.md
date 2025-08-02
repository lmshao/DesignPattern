# 抽象工厂模式（Abstract Factory Pattern）

## 设计逻辑说明

本示例通过抽象工厂模式实现了一个家具制造系统，支持创建相互兼容的家具产品族。

- **抽象产品**：`Chair`、`Table` 和 `Sofa` trait 定义了不同类型家具的接口。
- **具体产品**：`ModernChair`、`ModernTable`、`ModernSofa` 和 `VictorianChair`、`VictorianTable`、`VictorianSofa` 结构体实现了相应的 trait，代表不同风格的家具。
- **抽象工厂**：`FurnitureFactory` trait 定义了创建相关家具产品族的接口。
- **具体工厂**：`ModernFurnitureFactory` 和 `VictorianFurnitureFactory` 实现了 `FurnitureFactory` trait，确保同一工厂创建的所有家具都遵循相同的风格并且相互兼容。
- **客户端**：`FurnitureManufacturer` 管理不同类型的工厂，通过抽象工厂模式创建完整的家具套装。

## 运行效果

main 函数创建了家具制造商，生产不同风格（现代和维多利亚）的完整家具套装，展示了抽象工厂模式如何确保同一工厂创建的所有对象都相互兼容并遵循相同的设计主题。

## 适用场景

抽象工厂模式适用于需要创建相互兼容的相关对象族的场景。例如：
- 家具制造：创建特定风格的完整家具套装
- UI组件库：创建遵循相同设计系统的UI组件
- 数据库系统：创建来自同一供应商的数据库连接、查询和结果集
- 游戏开发：创建属于同一阵营或主题的游戏对象
- 操作系统组件：创建遵循相同外观和感觉的GUI元素 