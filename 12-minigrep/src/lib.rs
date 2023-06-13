use std::{env, error::Error, fs};



// i would much rather type this with lifetimes and it would be very easy but i'll just follow along for now
pub struct Config {
	pub query: String,
	pub file_path: String,

	pub ignore_case: bool
}



impl Config {
	pub fn new(args: &[String]) -> Result<Config, &'static str> {
		if args.len() < 3 {
			return Err("Not enough arguments!");
		}

		let query = args[1].to_string();
		let file_path = args[2].to_string();

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



pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();

	// it would be so easy to write this as an iterator;;;;
	for line in contents.lines() {
		if line.contains(&query) {
			results.push(line);
		}
	}

	return results;
}



// snooork mimimimimi....
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	let mut results = Vec::new();

	for line in contents.lines() {
		if line.to_lowercase().contains(&query) {
			results.push(line);
		}
	}

	return results;
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
