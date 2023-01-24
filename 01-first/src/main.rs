fn main() {
	// let is actually a const
	let my_integer: u16 = 320;
	println!("The value of my_integer is {my_integer}");

	// let mut is actually a let
	let integer_2: u8 = 74;
	println!("{integer_2}");

	let sum = add_numbers(3904603, 3040431);

	println!("Generic add numbers function: Sum of 3904603 and 3040431: {sum}");
}



fn add_numbers(a: i32, b: i32) -> i64 {
	return (a + b) as i64;
}
