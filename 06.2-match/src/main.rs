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

	// this code is completley redundant but it shows how to exhaust match arms
	map_to_number(sticker_color);
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
