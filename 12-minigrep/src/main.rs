use std::{env, fs};



fn main() {
	// the book wants me to annotate the type. i will use turbofish instead. no one can stop me.
	let args = env::args().collect::<Vec<String>>();

	let query = &args[1];
	let file_path = &args[2];

	println!("Searching for {} in file {}", query, file_path);


	let file = fs::read_to_string(file_path)
		.expect("should've been able to read file!");

	println!("File contents:\n{}", file);
}
