use std::sync::OnceLock;

struct Logger {
    prefix: &'static str,
}

static LOGGER: OnceLock<Logger> = OnceLock::new();

fn get_logger() -> &'static Logger {
    LOGGER.get_or_init(|| Logger { prefix: "[Singleton]" })
}

fn main() {
    let logger1 = get_logger();
    let logger2 = get_logger();
    println!("{} Hello, world!", logger1.prefix);
    println!("logger1 address: {:p}", logger1);
    println!("logger2 address: {:p}", logger2);
    println!("Is same instance: {}", std::ptr::eq(logger1, logger2));
}
