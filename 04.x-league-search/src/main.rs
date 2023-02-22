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

	let input_chars: Vec<char> = input.trim().to_ascii_lowercase().chars().collect();


	let mut filtered: HashMap<usize, isize> = HashMap::new();

	// https://stackoverflow.com/a/66289009
	let binding = champs_vec.clone();
	let _champions_iterable = binding.iter().enumerate();
	let characters_iterable = input_chars.iter().enumerate();


	for (search_index, character) in characters_iterable {
		// because of variable moving and whatnot i have to clone this
		let champions_iterable = _champions_iterable.clone();

		for (champion_index, champion) in champions_iterable {
			let champion_lower = champion.to_ascii_lowercase();

			// location of character in string
			let character_index = unwrap_usize(champion_lower.find(*character));

			let key_exists = filtered.contains_key(&champion_index);


			// character is not in string. don't bother
			if character_index == -1 {
				filtered.remove(&champion_index);
				continue;
			}


			// character is in string, but it's the first pass and the champion isn't in the hashmap. set it
			if !key_exists && search_index == 0 {
				filtered.insert(champion_index, character_index);
				continue;
			}

			// the champion isn't in the hashmap and contains the target character;
			// but it isn't the first pass, so it was removed for a reason. ignore it
			if !key_exists && search_index != 0 {
				continue;
			}


			// champion exists in the map and contains this character
			let old_position = filtered.get(&champion_index);
			let old_position = old_position.unwrap_or(&-1);

			// find the location of the character and whether or not it came after the most recent found character
			let character_index = find_from_position(champion_lower, *character, old_position + 1);

			if character_index == -1 {
				// character exists in the champion name, but it comes before the most recent found character
				filtered.remove(&champion_index);
				continue;
			}

			// champion has the desired character present after the last matched one. we're good!
			filtered.insert(champion_index, character_index);
		}
	}


	let champs_output = filtered.keys();
	let champs_output = champs_output.map(|champ| champs_vec[*champ].clone());

	let mut champs_output = champs_output.collect::<Vec<String>>();

	champs_output.sort();

	println!("Matched champions include: {}", champs_output.join(", "));
}



fn find_from_position(string: String, character: char, position: isize) -> isize {
	// this is certainly one the functions i will write
	// update: having rewritten it, it isn't really that scary

	for (index, char) in string.chars().enumerate() {
		if index < usize::try_from(position).ok().unwrap() {
			continue;
		}

		if char == character {
			return index as isize;
		}
	}

	return -1;
}



fn unwrap_usize(index: Option<usize>) -> isize {
	return match index {
		Some(n) => n as isize,
		None => -1
	};
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
