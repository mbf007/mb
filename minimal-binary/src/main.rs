use log::info;


// read the git commit version at compile time

fn main() {
    // Initialize the logger
    env_logger::init();
    println!("Hello!");

    // Log messages with different severity levels
    info!("This is an INFO level message.");

    // Print a standard output message
    println!("Goodbye");
	let h = env!("VERGEN_GIT_SHA");
	println!("{}", h);


}
