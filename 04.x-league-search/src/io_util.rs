use std::{fs, io::{self, BufReader}};



pub fn read_line() -> String {
	let mut input = String::new();

	let stdin = io::stdin();

	stdin.read_line(&mut input).expect("should've been able to read line!");

	// there's probably a better way to do this but I don't think I want a slice here (?)
	// also because slices are references and i can't have a dangling reference ig this is best
	let trimmed = input.trim().to_owned();

	return trimmed;
}


pub fn read_file(path: &str) -> BufReader<fs::File> {
	// https://stackoverflow.com/questions/30801031/
	let file = fs::File::open(path).expect("file should exist!");
	// bufreader is beyond the realm of what i know but i just want to get this done tbh
	let buf = BufReader::new(file);

	return buf;
}
