fn main() {
	// constants are in screaming snake case and always have a type signature
	// they should be used for known values. js const is more like rust let than a rust const
	const MY_CONST: u8 = 1;

	let my_value = 5;

	// you can create scopes anywhere you feel like in rust
	// i think you can in js too but uhhh don't

	{
		let my_value = 10;
		// shadowing, my value is 10
	}

	// my value is 5
	// variable shadowing can change a variable's type, but not re-assignment via mut

	let char = 'a';

	// typing a tuple isn't necessary
	// tuples have fixed length
	let tuple: (i8, f32, bool) = (75, 285.96, false);

	// destructuring a tuple
	let (t1, t2, t3) = tuple;

	// indexing a tuple
	let t1_index = tuple.0;

	// expressions implicitly return the unit value as an empty value (but not a None value)
	let unit = ();

	// arrays in rust are fixed length, lmao
	// on the stack, rather than the heap
	// arrays in rust are kinda like rust consts... generally you don't want them to change
	let array: [u8; 4] = [1, 5, 7, 8];

	// [74, 74, 74, 74, 74]
	let filled_array = [74; 5];

	// can also destructure arrays, index them like you would expect
	let [a1, a2, a3, a4] = array;
}
