fn main() {
	yawn_i_can_do_this_already(true);

	// statement: doesn't result in a value (variable assignments)
	// expression: does result in a value (5 + 6, all function/macro calls)

	// note: this weird behavior with blocks

	// X = 2
	const _X: i8 = {
		let y = 4;
		// note the lack of semicolon
		// if there was one here then it would be a statement (no return)
		y / 2
	};

	let _y = returnless_function();
}



fn yawn_i_can_do_this_already(real: bool) {
	println!("{real}");
}



fn returnless_function() -> i8 {
	// personally i hate this pattern and i would much rather use return always. but it's something you can do
	return 73;
}
