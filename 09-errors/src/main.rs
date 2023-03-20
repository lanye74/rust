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
	};

	// alternatively
	file_result.unwrap_or_else(|_error| {
		// code that returns a file here
		panic!("issue!");
	});


	// unwrap value/panic if err
	result.unwrap();

	// unwrap value/panic with message if err
	result.expect("something should've worked!");

	// propagate errors via ?
	// let file = File::open("hello.txt")?;
	// unwraps if it exists, returns Err if not
}
