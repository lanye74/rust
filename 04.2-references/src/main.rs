fn main() {
	let str1 = String::from("hello");
	mine(str1);

	// str1 has now been borrowed. no more using it

	let str2 = String::from("salutations");
	let mut str3 = mine_but_trade_variables(str2);
	// str2 has been moved. however, str3 is valid
	// println!("{str2}");

	// this is tedious. instead, we can use references

	mine_then_yours(&str3);

	println!("can still access str3: {str3}");


	// you can't use more than one mutable reference at a time
	// declaring them is ok, but you can only use one
	let _ref1 = &mut str3;
	let _ref2 = &mut str3;

	// you also can't use both mutable and immutable references
	let _ref3 = &str3;
}



fn mine(input: String) {
	println!("{input}");
}



fn mine_but_trade_variables(input: String) -> String {
	println!("{input}");
	return input;
}



// even though input goes out of scope here, it doesn't own str3; so str3 isn't dropped
fn mine_then_yours(input: &String) {
	// input is now a pointer
	println!("{input}");
}



// this will error; you're returning a reference to de-allocated memory ("dangling reference");
// fn dangling() -> &String {
	// return &String::from("hello");
// }


