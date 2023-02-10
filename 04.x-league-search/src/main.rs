use std::{fs, io::{self, BufRead, BufReader}, collections::HashMap};

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
	let input_chars: Vec<char> = input.trim().to_ascii_lowercase().chars().collect();



	// starting to realize this might not be that feasible until i learn how to make my own data structures
	// either i can turn these into tuples with (champ, index of matched character)
	// or just... give up
	// might need to make an implementation in typescript to get a better grasp of what i should be doing



	let mut filtered: HashMap<String, i32> = HashMap::new();

	// https://stackoverflow.com/a/66289009
	let binding = champs_vec.clone();
	let _champions_iterable = binding.iter().enumerate();
	let characters_iterable = input_chars.iter().enumerate();


	for (search_index, character) in characters_iterable {
		// because of variable moving and whatnot i have to clone this
		let champions_iterable = _champions_iterable.clone();

		for (champion_index, champion) in champions_iterable {
			// let character_index = champion.to_ascii_lowercase().find(character);
		}
	}



	// let champs_output = output_vec.join(", ");

	// let champs_output = filtered.keys()

	// println!("Champions matching your search: {}", champs_output);

	println!("{}", find_from_position("hello", 'h', 1));
}



fn find_from_position(string: &str, character: char, position: usize) -> isize {
	// this is certainly one the functions i will write
	let len = string.len();
	let mut characters = string.chars();

	let mut i = position;
	let mut found = false;

	while i < len {
		// let

		let char = match characters.nth(i).unwrap() {
			Some(n) => {

			},
			None => {

			}
		}

		i += 1;
	}

	return if found == true {i.try_into().unwrap()} else {-1};
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
