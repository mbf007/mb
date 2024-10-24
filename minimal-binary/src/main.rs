use log::info;

fn main() {
    // Initialize the logger
    env_logger::init();
    println!("Hello!");

    // Log messages with different severity levels
    info!("This is an INFO level message.");

    // Print a standard output message
    println!("Goodbye");
}
