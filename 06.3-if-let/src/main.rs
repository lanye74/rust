fn main() {
	let my_value = Some(7);

	// this is cringe and too verbose
	match my_value {
		Some(7) => println!("value is 7"),
		// matches must be exhaustive thus this is necessary
		_ => ()
	};

	// syntactic sugar for match basically
	if let Some(7) = my_value {
		println!("value is 7");
	}
}
