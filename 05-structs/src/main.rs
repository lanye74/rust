struct Champion {
	name: String,
	resource: String,
	attack_type: String,
	role: Vec<String>
}



fn main() {
    println!("Hello, world!");

	let kayn = Champion {
		name: String::from("Kayn"),
		resource: String::from("Mana"),
		attack_type: String::from("Melee"),
		role: vec![String::from("Jungle")]
	};

	// constructing structs can use the x: x, --> x, shorthand

	let diana = Champion {
		name: String::from("Diana"),
		role: vec![String::from("Jungle"), String::from("Mid")],
		// "struct update syntax"
		..kayn
	};

	// valid
	println!("{}", kayn.name);

	// not valid; value was moved
	// println!("{}", kayn.attack_type);
}
