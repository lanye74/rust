fn main() {
	// slice: a reference to a section of a whole collection
	let string = String::from("hello world");

	let bytes = string.as_bytes();

	for (_i, &item) in bytes.iter().enumerate() {
		// if item == ' ' {

		// }
	}
}
