fn main() {
	// slice: a reference to a section of a whole collection
	let mut string = String::from("hello world");

	let word = first_word(&string);

	println!("\"{}\"", word);

	// causes error, because a slice is an immutable reference, and clear takes a mutable one
	// string.clear();
	// println!("\"{}\"", word);

	// this is great and all but if i were to, say, call string.clear(), then...
	// what do we do with the value 5?

	// in js this would just be expected behavior. in rust we can do better with slices


	// you can also slice arrays
	let array = [1, 2, 3, 4, 5];
	// from index 1 to element 3... idk why it's like this but it happens in a bunch of other languages so
	let slice = &array[1..3];


}



// fn first_word(input: &String) -> usize {
// fn first_word(input: &String) -> &str {
fn first_word(input: &str) -> &str {
	// string to array of bytes
	let bytes = input.as_bytes();

	// iter: returns each element in a collection
	// enumerate: wraps each element in an [index, &element] tuple
	for (i, &item) in bytes.iter().enumerate() {
		// byte literal
		if item == b' ' {
			// return i;
			return &input[..i];
		}
	}

	// return input.len();
	return &input[..];
}
