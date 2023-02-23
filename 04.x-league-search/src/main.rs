use std::collections::HashMap;
// use std::time::SystemTime;

mod tools;
mod io_util;



fn main() {
	let path = "./champs.txt";


	let file_buffer = io_util::read_file(&path);

	let champs_vec = tools::buffer_to_vec(file_buffer);


	println!("Enter your query...");
	let input = io_util::read_line();

	let input_chars: Vec<char> = input.trim().to_ascii_lowercase().chars().collect();


	let mut filtered: HashMap<usize, isize> = HashMap::new();

	// https://stackoverflow.com/a/66289009
	let binding = champs_vec.clone();
	// more efficient to do this out here rather than in the loop
	let _champions_iterable = binding.iter().map(|champ| champ.to_ascii_lowercase()).enumerate();
	let characters_iterable = input_chars.iter().enumerate();


	for (search_index, character) in characters_iterable {
		// because of variable moving and whatnot i have to clone this
		let champions_iterable = _champions_iterable.clone();

		for (champion_index, champion) in champions_iterable {
			// location of character in string
			let character_index = tools::unwrap_usize_to_isize(champion.find(*character));

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
			let character_index = tools::find_from_position(champion, *character, old_position + 1);

			if character_index == -1 {
				// character exists in the champion name, but it comes before the most recent found character
				filtered.remove(&champion_index);
				continue;
			}

			// champion has the desired character present after the last matched one. we're good!
			filtered.insert(champion_index, character_index);
		}
	}



	// let t1 = SystemTime::now();
	let champs_output = filtered.keys();

	// because of moving and whatnot this has to be cloned
	// let champs_output = champs_output.map(|champ| champs_vec[*champ].clone());
	// let mut champs_output = champs_output.collect::<Vec<String>>();

	// did some very unofficial benchmarking and it looks like slices are slightly faster for both smaller and larger datasets
	// ionknow if it is. ionknow what best practice is. don't ask me
	let champs_output = champs_output.map(|champ| &champs_vec[*champ][..]);
	// turbofish :)
	let mut champs_output = champs_output.collect::<Vec<&str>>();


	champs_output.sort();
	println!("Matched champions include: {}", champs_output.join(", "));

	// let t2 = SystemTime::now();
	// let elapsed = t2.duration_since(t1).unwrap();

	// println!("Time taken: {:?}", elapsed);
}
