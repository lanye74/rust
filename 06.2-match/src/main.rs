#[allow(dead_code)]
enum RubiksColors {
	White,
	Blue,
	Red,
	Yellow,
	Green,
	Orange
}




fn main() {
	let sticker_color = RubiksColors::Red;

	// this code is completely redundant but it shows how to exhaust match arms
	map_to_number(sticker_color);

	let option = Some(5);

	let _option_plus_one = match option {
		// wrapping i + 1 in a some allows you to return None as opposed to like... -1 or something
		Some(i) => Some(i + 1),
		None => None
	};


	let _unwrap = match option {
		Some(5) => {
			println!("value is 5");
		},
		// can also be a variable name
		_ => ()
	};
}



fn map_to_number(color: RubiksColors) -> i32 {
	return match color {
		RubiksColors::White => 0,
		RubiksColors::Blue => 1,
		RubiksColors::Red => 2,
		RubiksColors::Yellow => 3,
		RubiksColors::Green => 4,
		RubiksColors::Orange => 5
	};
}
