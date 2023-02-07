use std::{fs, io::{self, BufRead, BufReader}};

// the more i delve into this the more i realize that it's way out of my... league (badum-tshh)



fn main() {
	let path = "./champs.txt";

	// https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
	// https://doc.rust-lang.org/std/error/index.html#common-message-styles
	// let champs = fs::read_to_string(path).expect("file should've been read!");

	// https://doc.rust-lang.org/std/string/struct.String.html#method.split
	// let mut champs_vec: Vec<&str> = champs.split('\n').collect();
	// champs_vec.pop(); // remove newline


	// https://stackoverflow.com/questions/30801031/
	let file = fs::File::open(path).expect("file should exist!");
	// bufreader is beyond the realm of what i know but i just want to get this done tbh
	let buf = BufReader::new(file);

	let champs_vec: Vec<String> = buf.lines()
		.map(|l| l.expect("should've been able to parse line!"))
		.collect();



	println!("Enter your query...");
	let input = read_line();

	// let input_chars: Vec<char> = input.chars().map(|char| char.to_ascii_lowercase()).collect();
	let input_chars: Vec<char> = input.to_ascii_lowercase().chars().collect();
	let char = input_chars[0];

	// https://doc.rust-lang.org/book/ch08-01-vectors.html
	let mut output_vec: Vec<&str> = vec![];


	// starting to realize this might not be that feasible until i learn how to make my own data structures
	// either i can turn these into tuples with (champ, index of matched character)
	// or just... give up
	// might need to make an implementation in typescript to get a better grasp of what i should be doing
	for (_i, champ) in champs_vec.iter().enumerate() {
		if champ.to_ascii_lowercase().contains(char) {
			output_vec.push(champ);
		}
	}

	let champs_output = output_vec.join(", ");

	println!("Champions matching your search: {}", champs_output);
}



fn read_line() -> String {
	let mut input = String::new();

	let stdin = io::stdin();

	stdin.read_line(&mut input).expect("should've been able to read line!");

	// there's probably a better way to do this but I don't think I want a slice here (?)
	// also because slices are references and i can't have a dangling reference ig this is best
	let trimmed = input.trim().to_string();

	return trimmed;
}
