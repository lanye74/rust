use std::{fs, io::{BufRead,BufReader}};



pub fn unwrap_usize(index: Option<usize>) -> isize {
	return match index {
		Some(n) => n as isize,
		None => -1
	};
}



pub fn find_from_position(string: String, character: char, position: isize) -> isize {
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



pub fn buffer_to_vec(buffer: BufReader<fs::File>) -> Vec<String> {
	return buffer.lines()
		.map(|l| l.expect("should've been able to parse line!"))
		.collect();
}
