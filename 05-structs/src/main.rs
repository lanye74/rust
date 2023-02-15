struct Champion {
	name: String,
	resource: String,
	attack_type: String,
	roles: Vec<String>,
	cs: i32
}



impl Champion {
	// shorthand for self: &[mut] Self
	// rust allows you to have properties and methods named the same thing,
	// which is nice in cases like this were cs is both a noun and verb
	fn cs(&mut self) {
		self.cs += 4;
	}

	// functions implemented without a self ref are called associated functions
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
		roles: vec![String::from("Jungle")],
		cs: 0
	};

	// constructing structs can use the x: x, --> x, shorthand

	let mut diana = Champion {
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

	let mid = midpoint(&my_point, &my_other_point);

	// :? = debug print
	// :#? = debug pretty-print
	println!("average of points: {:?}", &mid);

	// see also: dbg! macro which prints more advanced info about line
	// dbg! macro takes ownership and returns a new variable which allows you to do this
	let my_other_other_point = Point(4.0, dbg!(mid.1 * 7.0), -5.0);


	// methods on structs
	diana.cs();

	println!("diana cs: {}", diana.cs);
}



fn midpoint(n: &Point, m: &Point) -> Point {
	return Point(mean(n.0, m.0), mean(n.1, m.1), mean(n.2, m.2));
}



fn mean(x: f32, y: f32) -> f32 {
	return (x + y) / 2.0;
}
