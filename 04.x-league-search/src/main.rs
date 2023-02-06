use std::{fs, io};


fn main() {
	let path = "../champs.txt";

	// https://doc.rust-lang.org/std/error/index.html#common-message-styles
	let champs = fs::read_to_string(path).expect("file should've been read!");
}
