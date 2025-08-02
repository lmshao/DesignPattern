// Copyright © 2025 SHAO Liming <lmshao@163.com>
// Licensed under the MIT License
//
// Abstract Factory Pattern Example
//
// The abstract factory pattern provides an interface for creating families
// of related objects without specifying their concrete classes. This pattern
// ensures that the created objects are compatible with each other.

use std::collections::HashMap;

/// Chair trait - Abstract Product A
trait Chair {
    fn sit_on(&self);
    fn get_info(&self) -> String;
}

/// Table trait - Abstract Product B
trait Table {
    fn put_on(&self);
    fn get_info(&self) -> String;
}

/// Sofa trait - Abstract Product C
trait Sofa {
    fn lie_on(&self);
    fn get_info(&self) -> String;
}

/// Modern Chair - Concrete Product A1
struct ModernChair {
    material: String,
    color: String,
}

impl ModernChair {
    fn new(material: String, color: String) -> Self {
        Self { material, color }
    }
}

impl Chair for ModernChair {
    fn sit_on(&self) {
        println!("🪑 Sitting on modern {} {} chair", self.color, self.material);
    }

    fn get_info(&self) -> String {
        format!("Modern Chair - Material: {}, Color: {}", self.material, self.color)
    }
}

/// Modern Table - Concrete Product B1
struct ModernTable {
    material: String,
    color: String,
    size: String,
}

impl ModernTable {
    fn new(material: String, color: String, size: String) -> Self {
        Self { material, color, size }
    }
}

impl Table for ModernTable {
    fn put_on(&self) {
        println!("🪑 Putting items on modern {} {} {} table", self.color, self.material, self.size);
    }

    fn get_info(&self) -> String {
        format!("Modern Table - Material: {}, Color: {}, Size: {}", self.material, self.color, self.size)
    }
}

/// Modern Sofa - Concrete Product C1
struct ModernSofa {
    material: String,
    color: String,
    seats: u32,
}

impl ModernSofa {
    fn new(material: String, color: String, seats: u32) -> Self {
        Self { material, color, seats }
    }
}

impl Sofa for ModernSofa {
    fn lie_on(&self) {
        println!("🛋️ Lying on modern {} {} sofa with {} seats", self.color, self.material, self.seats);
    }

    fn get_info(&self) -> String {
        format!("Modern Sofa - Material: {}, Color: {}, Seats: {}", self.material, self.color, self.seats)
    }
}

/// Victorian Chair - Concrete Product A2
struct VictorianChair {
    material: String,
    color: String,
    carvings: bool,
}

impl VictorianChair {
    fn new(material: String, color: String, carvings: bool) -> Self {
        Self { material, color, carvings }
    }
}

impl Chair for VictorianChair {
    fn sit_on(&self) {
        let carving_desc = if self.carvings { "with beautiful carvings" } else { "without carvings" };
        println!("🪑 Sitting on victorian {} {} chair {}", self.color, self.material, carving_desc);
    }

    fn get_info(&self) -> String {
        let carving_desc = if self.carvings { "with carvings" } else { "without carvings" };
        format!("Victorian Chair - Material: {}, Color: {}, {}", self.material, self.color, carving_desc)
    }
}

/// Victorian Table - Concrete Product B2
struct VictorianTable {
    material: String,
    color: String,
    size: String,
    carvings: bool,
}

impl VictorianTable {
    fn new(material: String, color: String, size: String, carvings: bool) -> Self {
        Self { material, color, size, carvings }
    }
}

impl Table for VictorianTable {
    fn put_on(&self) {
        let carving_desc = if self.carvings { "with beautiful carvings" } else { "without carvings" };
        println!("🪑 Putting items on victorian {} {} {} table {}", self.color, self.material, self.size, carving_desc);
    }

    fn get_info(&self) -> String {
        let carving_desc = if self.carvings { "with carvings" } else { "without carvings" };
        format!("Victorian Table - Material: {}, Color: {}, Size: {}, {}", self.material, self.color, self.size, carving_desc)
    }
}

/// Victorian Sofa - Concrete Product C2
struct VictorianSofa {
    material: String,
    color: String,
    seats: u32,
    carvings: bool,
}

impl VictorianSofa {
    fn new(material: String, color: String, seats: u32, carvings: bool) -> Self {
        Self { material, color, seats, carvings }
    }
}

impl Sofa for VictorianSofa {
    fn lie_on(&self) {
        let carving_desc = if self.carvings { "with beautiful carvings" } else { "without carvings" };
        println!("🛋️ Lying on victorian {} {} sofa with {} seats {}", self.color, self.material, self.seats, carving_desc);
    }

    fn get_info(&self) -> String {
        let carving_desc = if self.carvings { "with carvings" } else { "without carvings" };
        format!("Victorian Sofa - Material: {}, Color: {}, Seats: {}, {}", self.material, self.color, self.seats, carving_desc)
    }
}

/// FurnitureFactory trait - Abstract Factory
trait FurnitureFactory {
    fn create_chair(&self, material: String, color: String) -> Box<dyn Chair>;
    fn create_table(&self, material: String, color: String, size: String) -> Box<dyn Table>;
    fn create_sofa(&self, material: String, color: String, seats: u32) -> Box<dyn Sofa>;
    fn get_factory_name(&self) -> &str;
}

/// ModernFurnitureFactory - Concrete Factory 1
struct ModernFurnitureFactory;

impl FurnitureFactory for ModernFurnitureFactory {
    fn create_chair(&self, material: String, color: String) -> Box<dyn Chair> {
        println!("🏭 Modern factory creating chair: {} {}", material, color);
        Box::new(ModernChair::new(material, color))
    }

    fn create_table(&self, material: String, color: String, size: String) -> Box<dyn Table> {
        println!("🏭 Modern factory creating table: {} {} {}", material, color, size);
        Box::new(ModernTable::new(material, color, size))
    }

    fn create_sofa(&self, material: String, color: String, seats: u32) -> Box<dyn Sofa> {
        println!("🏭 Modern factory creating sofa: {} {} with {} seats", material, color, seats);
        Box::new(ModernSofa::new(material, color, seats))
    }

    fn get_factory_name(&self) -> &str {
        "Modern Furniture Factory"
    }
}

/// VictorianFurnitureFactory - Concrete Factory 2
struct VictorianFurnitureFactory;

impl FurnitureFactory for VictorianFurnitureFactory {
    fn create_chair(&self, material: String, color: String) -> Box<dyn Chair> {
        println!("🏭 Victorian factory creating chair: {} {}", material, color);
        Box::new(VictorianChair::new(material, color, true))
    }

    fn create_table(&self, material: String, color: String, size: String) -> Box<dyn Table> {
        println!("🏭 Victorian factory creating table: {} {} {}", material, color, size);
        Box::new(VictorianTable::new(material, color, size, true))
    }

    fn create_sofa(&self, material: String, color: String, seats: u32) -> Box<dyn Sofa> {
        println!("🏭 Victorian factory creating sofa: {} {} with {} seats", material, color, seats);
        Box::new(VictorianSofa::new(material, color, seats, true))
    }

    fn get_factory_name(&self) -> &str {
        "Victorian Furniture Factory"
    }
}

/// FurnitureManufacturer - Client class that uses abstract factories
struct FurnitureManufacturer {
    factories: HashMap<String, Box<dyn FurnitureFactory>>,
}

impl FurnitureManufacturer {
    fn new() -> Self {
        let mut factories: HashMap<String, Box<dyn FurnitureFactory>> = HashMap::new();
        factories.insert("modern".to_string(), Box::new(ModernFurnitureFactory) as Box<dyn FurnitureFactory>);
        factories.insert("victorian".to_string(), Box::new(VictorianFurnitureFactory) as Box<dyn FurnitureFactory>);
        
        Self { factories }
    }

    fn create_furniture_set(&self, style: &str, material: String, color: String) -> Option<(Box<dyn Chair>, Box<dyn Table>, Box<dyn Sofa>)> {
        if let Some(factory) = self.factories.get(style) {
            println!("🏭 Using {} to create furniture set", factory.get_factory_name());
            
            let chair = factory.create_chair(material.clone(), color.clone());
            let table = factory.create_table(material.clone(), color.clone(), "Medium".to_string());
            let sofa = factory.create_sofa(material, color, 3);
            
            Some((chair, table, sofa))
        } else {
            println!("❌ Unknown furniture style: {}", style);
            None
        }
    }

    fn list_available_styles(&self) {
        println!("📋 Available furniture styles:");
        for (style, factory) in &self.factories {
            println!("  - {}: {}", style, factory.get_factory_name());
        }
    }
}

fn main() {
    println!("🏭 Abstract Factory Pattern Example - Furniture Manufacturing System");
    println!("{}", "=".repeat(60));

    // Create furniture manufacturer
    let manufacturer = FurnitureManufacturer::new();
    
    // Display available styles
    manufacturer.list_available_styles();
    println!();

    // Create furniture sets in different styles
    let furniture_orders = vec![
        ("modern", "Leather", "Black"),
        ("victorian", "Wood", "Brown"),
    ];

    for (style, material, color) in furniture_orders {
        println!("🏭 Creating {} furniture set...", style);
        
        if let Some((chair, table, sofa)) = manufacturer.create_furniture_set(style, material.to_string(), color.to_string()) {
            println!();
            println!("📋 {} Furniture Set Details:", style.to_uppercase());
            println!("{}", "=".repeat(40));
            
            // Test chair
            println!("📋 {}", chair.get_info());
            chair.sit_on();
            
            // Test table
            println!("📋 {}", table.get_info());
            table.put_on();
            
            // Test sofa
            println!("📋 {}", sofa.get_info());
            sofa.lie_on();
            
            println!();
        }
        println!();
    }

    println!("✅ Abstract Factory Pattern example completed!");
    println!();
    println!("💡 Design Pattern Key Points:");
    println!("  - Abstract Factory creates families of related objects");
    println!("  - Chair, Table, Sofa are abstract products");
    println!("  - Modern/Victorian variants are concrete products");
    println!("  - FurnitureFactory is the abstract factory interface");
    println!("  - ModernFurnitureFactory/VictorianFurnitureFactory are concrete factories");
    println!("  - All products from same factory are guaranteed to be compatible");
}
