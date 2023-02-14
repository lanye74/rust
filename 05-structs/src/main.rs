struct Champion {
	name: String,
	resource: String,
	attack_type: String,
	roles: Vec<String>
}



// ??? println was whining at me to use it
// apparently it allows me to use the Debug format {:?}
#[derive(Debug)]
struct Point(f32, f32, f32);



fn main() {
	   println!("Hello, world!");

	let kayn = Champion {
		name: String::from("Kayn"),
		resource: String::from("Mana"),
		attack_type: String::from("Melee"),
		roles: vec![String::from("Jungle")]
	};

	// constructing structs can use the x: x, --> x, shorthand

	let diana = Champion {
		name: String::from("Diana"),
		roles: vec![String::from("Jungle"), String::from("Mid")],
		// "struct update syntax"
		..kayn
	};

	// valid
	println!("{}", kayn.name);

	// not valid; value was moved
	// println!("{}", kayn.attack_type);


	// tuple structs
	// it's basically a way to give a tuple type a name
	let my_point = Point(-3.0, 5.0, 2.0);
	let my_other_point = Point(-1.0, 7.0, 6.0);


	println!("average of points: {:?}", midpoint(&my_point, &my_other_point));
}



fn midpoint(n: &Point, m: &Point) -> Point {
	return Point(mean(n.0, m.0), mean(n.1, m.1), mean(n.2, m.2));
}



fn mean(x: f32, y: f32) -> f32 {
	return (x + y) / 2.0;
}
