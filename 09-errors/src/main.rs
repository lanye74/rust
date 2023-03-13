use std::fs::File;
use std::io::ErrorKind;



fn main() {
	// abort program
	// panic!("bad things happened!!");

	let result: Result<(), ()> = Ok(());

	match result {
		Ok(()) => {},
		Err(()) => {}
	};


	let file_result = File::open("doesntexist.txt");

	match file_result {
		// no idea what ref is but i need it apparently
		Ok(ref _file) => {},
		Err(ref error) => match error.kind() {
			ErrorKind::NotFound => {}, // possibly create file here or whatever with another match handlign it
			_ => {} // whatever other errors
		}
	}

	// alternatively
	file_result.unwrap_or_else(|error| {
		// code that returns a file here
		panic!("issue!")
	});
}
