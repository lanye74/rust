fn main() {
	let string = String::from("hello world this is multiple words");

	let bytes = string.as_bytes();

	for (_i, &item) in bytes.iter().enumerate() {
		// if item == ' ' {

		// }
	}
}
