use std::env;



fn main() {
	// the book wants me to annotate the type. i will use turbofish instead. no one can stop me.
	let args = env::args().collect::<Vec<String>>();

	dbg!(args);
}
