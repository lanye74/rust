use std::{env, error::Error, fs};



// i would much rather type this with lifetimes and it would be very easy but i'll just follow along for now
pub struct Config {
	pub query: String,
	pub file_path: String,

	pub ignore_case: bool
}



impl Config {
	pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
		// skip file path
		args.next();

		// step through the iterator
		// if one isn't present, then error on it
		let query = match args.next() {
			Some(arg) => arg,
			None => return Err("no query string received!")
		};

		let file_path = match args.next() {
			Some(arg) => arg,
			None => return Err("no file path received!")
		};

		let ignore_case = env::var("IGNORE_CASE").is_ok();

		return Ok(Config {
			query,
			file_path,
			ignore_case
		});
	}
}



// Box<dyn Error>: a type that implements the Error trait
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	// ? operator returns Err if it is Err, continues otherwise
	let file_contents = fs::read_to_string(config.file_path)?;

	let results = if config.ignore_case {
		search_case_insensitive(&config.query, &file_contents)
	} else {
		search(&config.query, &file_contents)
	};


	for line in results {
		println!("{line}");
	}

	return Ok(());
}



pub fn search<'a>(query: &str, source: &'a str) -> Vec<&'a str> {
	return source.lines()
		.filter(|&line| line.contains(query))
		.collect::<Vec<&'a str>>();
}



// snooork mimimimimi....
pub fn search_case_insensitive<'a>(query: &str, source: &'a str) -> Vec<&'a str> {
	return source.lines()
		.filter(|&line| line.to_lowercase().contains(query))
		.collect::<Vec<&'a str>>();
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn case_sensitive() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape."; // this is so cheesy

		assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	}

	#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

		assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
	}
}
