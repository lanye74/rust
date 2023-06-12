use std::{error::Error, fs};



// i would much rather type this with lifetimes and it would be very easy but i'll just follow along for now
pub struct Config {
	pub query: String,
	pub file_path: String
}



impl Config {
	pub fn new(args: &[String]) -> Result<Config, &'static str> {
		if args.len() < 3 {
			return Err("Not enough arguments!");
		}

		let query = args[1].to_string();
		let file_path = args[2].to_string();

		return Ok(Config {
			query,
			file_path
		});
	}
}



// Box<dyn Error>: a type that implements the Error trait
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	// ? operator returns Err if it is Err, continues otherwise
	let file = fs::read_to_string(config.file_path)?;

	println!("File contents:\n{}", file);

	return Ok(());
}
