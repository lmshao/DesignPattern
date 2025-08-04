// Product struct
struct Computer {
    cpu: String,
    ram: u32,
    storage: u32,
}

// Builder trait
trait ComputerBuilder {
    fn set_cpu(&mut self, cpu: &str);
    fn set_ram(&mut self, ram: u32);
    fn set_storage(&mut self, storage: u32);
    fn build(&self) -> Computer;
}

// Concrete Builder
struct MyComputerBuilder {
    cpu: String,
    ram: u32,
    storage: u32,
}

impl MyComputerBuilder {
    fn new() -> Self {
        Self {
            cpu: String::new(),
            ram: 0,
            storage: 0,
        }
    }
}

impl ComputerBuilder for MyComputerBuilder {
    fn set_cpu(&mut self, cpu: &str) {
        self.cpu = cpu.to_string();
    }
    fn set_ram(&mut self, ram: u32) {
        self.ram = ram;
    }
    fn set_storage(&mut self, storage: u32) {
        self.storage = storage;
    }
    fn build(&self) -> Computer {
        Computer {
            cpu: self.cpu.clone(),
            ram: self.ram,
            storage: self.storage,
        }
    }
}

// Director
struct Director;

impl Director {
    fn construct_gaming_pc(builder: &mut dyn ComputerBuilder) -> Computer {
        builder.set_cpu("Intel i9");
        builder.set_ram(32);
        builder.set_storage(2000);
        builder.build()
    }
}

fn main() {
    let mut builder = MyComputerBuilder::new();
    let gaming_pc = Director::construct_gaming_pc(&mut builder);
    println!(
        "Gaming PC: CPU={}, RAM={}GB, Storage={}GB",
        gaming_pc.cpu, gaming_pc.ram, gaming_pc.storage
    );
}
