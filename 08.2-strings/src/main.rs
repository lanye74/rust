#![allow(unused_variables)]



fn main() {
	// strings are implemented as a wrapper of a vector
	let string1 = String::new();
	let string1 = "".to_string();
	let mut string1 = String::from("");

	// uses a string slice; doesn't take ownership of parameter
	string1.push_str("Hello");
	string1.push(' ');

	let string2 = String::from("world");

	// add function takes type String and &str; &String is coerced into &str here
	// &string2 --> &string2[..]

	// ownership of string1 is taken
	let string3 = string1 + &string2;

	// more concise concatenation (plus no ownership taking)
	let string4 = format!("{string3}!");

	// try to access character 'w' from world. throws error
	// let w = string4[0];

	// unicode encoding is complicated. so indexing 0 would access the first byte which may not necessarily be the entire first character;
	// it may be only part of a character. the only value at position 0 is the byte encoding of w
	// with a more complex character this doesn't work out

	// instead, you can access using string slices, which go by bytes:
	let w = &string4[0..1];

	// note that with this method you will get an error if you index a multi-byte character


	// can also iterate strings like so:
	let string4_str = &string4[..];

	for char in string4_str.chars() {

	}

	for byte in string4_str.bytes() {

	}
}
