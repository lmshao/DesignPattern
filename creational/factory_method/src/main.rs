// Copyright © 2025 SHAO Liming <lmshao@163.com>
// Licensed under the MIT License
//
// Factory Method Pattern Example
//
// The factory method pattern defines an interface for creating objects,
// but lets subclasses decide which class to instantiate. This pattern
// delegates the responsibility of object creation to subclasses.

use std::collections::HashMap;

/// Vehicle trait - Product interface
trait Vehicle {
    fn start_engine(&self);
    fn stop_engine(&self);
    fn get_info(&self) -> String;
}

/// Car - Concrete Product
struct Car {
    brand: String,
    model: String,
    year: u32,
}

impl Car {
    fn new(brand: String, model: String, year: u32) -> Self {
        Self {
            brand,
            model,
            year,
        }
    }
}

impl Vehicle for Car {
    fn start_engine(&self) {
        println!("🚗 {} {} engine started", self.brand, self.model);
    }

    fn stop_engine(&self) {
        println!("🚗 {} {} engine stopped", self.brand, self.model);
    }

    fn get_info(&self) -> String {
        format!("Car: {} {} ({})", self.brand, self.model, self.year)
    }
}

/// Motorcycle - Concrete Product
struct Motorcycle {
    brand: String,
    model: String,
    year: u32,
}

impl Motorcycle {
    fn new(brand: String, model: String, year: u32) -> Self {
        Self {
            brand,
            model,
            year,
        }
    }
}

impl Vehicle for Motorcycle {
    fn start_engine(&self) {
        println!("🏍️ {} {} engine started", self.brand, self.model);
    }

    fn stop_engine(&self) {
        println!("🏍️ {} {} engine stopped", self.brand, self.model);
    }

    fn get_info(&self) -> String {
        format!("Motorcycle: {} {} ({})", self.brand, self.model, self.year)
    }
}

/// Truck - Concrete Product
struct Truck {
    brand: String,
    model: String,
    year: u32,
    capacity: f32, // Load capacity (tons)
}

impl Truck {
    fn new(brand: String, model: String, year: u32, capacity: f32) -> Self {
        Self {
            brand,
            model,
            year,
            capacity,
        }
    }
}

impl Vehicle for Truck {
    fn start_engine(&self) {
        println!("🚛 {} {} engine started", self.brand, self.model);
    }

    fn stop_engine(&self) {
        println!("🚛 {} {} engine stopped", self.brand, self.model);
    }

    fn get_info(&self) -> String {
        format!("Truck: {} {} ({}) - Capacity: {} tons", self.brand, self.model, self.year, self.capacity)
    }
}

/// VehicleFactory trait - Creator interface
trait VehicleFactory {
    fn create_vehicle(&self, brand: String, model: String, year: u32) -> Box<dyn Vehicle>;
    fn get_factory_name(&self) -> &str;
}

/// CarFactory - Concrete Creator
struct CarFactory;

impl VehicleFactory for CarFactory {
    fn create_vehicle(&self, brand: String, model: String, year: u32) -> Box<dyn Vehicle> {
        println!("🏭 Car factory manufacturing: {} {}", brand, model);
        Box::new(Car::new(brand, model, year))
    }

    fn get_factory_name(&self) -> &str {
        "Car Factory"
    }
}

/// MotorcycleFactory - Concrete Creator
struct MotorcycleFactory;

impl VehicleFactory for MotorcycleFactory {
    fn create_vehicle(&self, brand: String, model: String, year: u32) -> Box<dyn Vehicle> {
        println!("🏭 Motorcycle factory manufacturing: {} {}", brand, model);
        Box::new(Motorcycle::new(brand, model, year))
    }

    fn get_factory_name(&self) -> &str {
        "Motorcycle Factory"
    }
}

/// TruckFactory - Concrete Creator
struct TruckFactory;

impl VehicleFactory for TruckFactory {
    fn create_vehicle(&self, brand: String, model: String, year: u32) -> Box<dyn Vehicle> {
        println!("🏭 Truck factory manufacturing: {} {}", brand, model);
        // Truck needs additional capacity parameter
        Box::new(Truck::new(brand, model, year, 10.0))
    }

    fn get_factory_name(&self) -> &str {
        "Truck Factory"
    }
}

/// VehicleManufacturer - Client class that uses factories
struct VehicleManufacturer {
    factories: HashMap<String, Box<dyn VehicleFactory>>,
}

impl VehicleManufacturer {
    fn new() -> Self {
        let mut factories: HashMap<String, Box<dyn VehicleFactory>> = HashMap::new();
        factories.insert("car".to_string(), Box::new(CarFactory) as Box<dyn VehicleFactory>);
        factories.insert("motorcycle".to_string(), Box::new(MotorcycleFactory) as Box<dyn VehicleFactory>);
        factories.insert("truck".to_string(), Box::new(TruckFactory) as Box<dyn VehicleFactory>);
        
        Self { factories }
    }

    fn manufacture_vehicle(&self, vehicle_type: &str, brand: String, model: String, year: u32) -> Option<Box<dyn Vehicle>> {
        if let Some(factory) = self.factories.get(vehicle_type) {
            println!("🏭 Using {} to manufacture vehicle", factory.get_factory_name());
            Some(factory.create_vehicle(brand, model, year))
        } else {
            println!("❌ Unknown vehicle type: {}", vehicle_type);
            None
        }
    }

    fn list_available_types(&self) {
        println!("📋 Available vehicle types:");
        for (vehicle_type, factory) in &self.factories {
            println!("  - {}: {}", vehicle_type, factory.get_factory_name());
        }
    }
}

fn main() {
    println!("🏭 Factory Method Pattern Example - Vehicle Manufacturing System");
    println!("{}", "=".repeat(50));

    // Create vehicle manufacturer
    let manufacturer = VehicleManufacturer::new();
    
    // Display available vehicle types
    manufacturer.list_available_types();
    println!();

    // Manufacture different types of vehicles
    let vehicles = vec![
        ("car", "Volkswagen", "Golf", 2024),
        ("motorcycle", "BMW", "R1200GS", 2024),
        ("truck", "Volvo", "FH16", 2024),
    ];

    let mut manufactured_vehicles: Vec<Box<dyn Vehicle>> = Vec::new();

    for (vehicle_type, brand, model, year) in vehicles {
        if let Some(vehicle) = manufacturer.manufacture_vehicle(vehicle_type, brand.to_string(), model.to_string(), year) {
            manufactured_vehicles.push(vehicle);
        }
        println!();
    }

    // Test manufactured vehicles
    println!("🚗 Testing manufactured vehicles:");
    println!("{}", "=".repeat(30));
    
    for vehicle in &manufactured_vehicles {
        println!("📋 {}", vehicle.get_info());
        vehicle.start_engine();
        vehicle.stop_engine();
        println!();
    }

    println!("✅ Factory Method Pattern example completed!");
    println!();
    println!("💡 Design Pattern Key Points:");
    println!("  - Vehicle trait defines the product interface");
    println!("  - Car, Motorcycle, Truck are concrete products");
    println!("  - VehicleFactory trait defines the factory interface");
    println!("  - CarFactory, MotorcycleFactory, TruckFactory are concrete factories");
    println!("  - VehicleManufacturer is the client that uses factories to create products");
}
