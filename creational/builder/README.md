# 建造者模式（Builder Pattern）

## 设计逻辑说明

本示例通过建造者模式实现了一个电脑（Computer）对象的灵活构建。

- **Product（产品）**：`Computer` 结构体，包含 CPU、内存、存储等属性。
- **Builder（建造者接口）**：`ComputerBuilder` trait，定义了设置各属性和构建产品的方法。
- **Concrete Builder（具体建造者）**：`MyComputerBuilder` 结构体，实现了 `ComputerBuilder` trait，负责具体属性的赋值和产品的创建。
- **Director（指挥者）**：`Director` 结构体，负责组织建造过程，调用建造者的方法一步步构建出特定类型的产品（如高端游戏电脑）。

## 运行效果

main 函数通过 Director 构建了一台高端游戏电脑，并输出其配置。

## 适用场景

建造者模式适用于需要分步骤构建复杂对象，且每一步可以灵活变化的场景。例如：
- 构建不同配置的电脑、汽车等复杂对象
- 需要保证构建过程有序、可控
