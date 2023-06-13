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

	for line in search(&config.query, &file) {
		println!("{line}");
	}

	return Ok(());
}



pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();

	// it would be so easy to write this as an iterator;;;;
	for line in contents.lines() {
		if line.contains(query) {
			results.push(line);
		}
	}

	return results;
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn one_result() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three."; // this is so cheesy

		assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	}
}
