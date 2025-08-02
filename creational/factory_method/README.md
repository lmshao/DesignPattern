# 工厂方法模式（Factory Method Pattern）

## 设计逻辑说明

本示例通过工厂方法模式实现了一个车辆制造系统，支持通过不同的工厂创建不同类型的车辆。

- **Product（产品接口）**：`Vehicle` trait，定义了车辆的基本接口，包含 `start_engine()`、`stop_engine()`、`get_info()`、`get_type()` 等方法。
- **Concrete Product（具体产品）**：`Car`、`Motorcycle`、`Truck` 结构体，实现了 `Vehicle` trait，分别表示汽车、摩托车和卡车。
- **Creator（创建者接口）**：`VehicleFactory` trait，定义了工厂的基本接口，包含 `create_vehicle()` 和 `get_factory_name()` 方法。
- **Concrete Creator（具体创建者）**：`CarFactory`、`MotorcycleFactory`、`TruckFactory` 结构体，实现了 `VehicleFactory` trait，分别负责创建汽车、摩托车和卡车。
- **Client（客户端）**：`VehicleManufacturer` 结构体，负责管理不同类型的工厂，通过工厂方法模式创建相应的车辆对象。

## 运行效果

main 函数创建了车辆制造商，通过不同的工厂制造汽车、摩托车和卡车，展示了如何通过工厂方法模式将对象创建的责任委托给子类，实现了创建逻辑与使用逻辑的分离。

## 适用场景

工厂方法模式适用于需要根据条件创建不同类型对象，且希望将创建逻辑与使用逻辑分离的场景。例如：
- 车辆制造系统：根据类型创建不同的车辆
- 文档编辑器：根据文件类型创建不同的编辑器
- 游戏对象创建：根据游戏类型创建不同的游戏对象
- 数据库连接：根据数据库类型创建不同的连接器 