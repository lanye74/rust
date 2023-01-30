fn main() {
	let str5 = String::from("hello");
	mine(str5);

	// str5 has now been borrowed. no more using it

	let str6 = String::from("salutations");
	let str7 = mine_but_trade_variables(str6);
	// str6 has been moved. however, str7 is valid
	// println!("{str6}");

	// this is tedious. instead, we can use references

	mine_then_yours(&str7);

	println!("can still access str7: {str7}");
}



fn mine(input: String) {
	println!("{input}");
}



fn mine_but_trade_variables(input: String) -> String {
	println!("{input}");
	return input;
}



fn mine_then_yours(input: &String) {
	// input is now a pointer
	println!("{input}");
}

// even though input goes out of scope here, it doesn't own str7; so str7 isn't dropped
