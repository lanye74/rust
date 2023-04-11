#![allow(dead_code)]
enum DamageType {
	AP,
	AD
}



enum RangeType {
	Melee,
	Ranged
}



enum Resource {
	Mana,
	Energy,
	Resourceless
}



enum Role {
	Top,
	Jungle,
	Mid,
	ADC,
	Support
}



struct Champion {
	name: String,
	damage_type: DamageType,
	range_type: RangeType,
	resource: Resource,
	role: Vec<Role>
}



// not sure why you would ever use this? but you could
// you can also put structs inside enum variants if you wanted..? and other enums

// i guess the eaxmple they gave is nice. you have a Request type of sorts
// that can either be a request to do X, a request to do Y, etc.
enum Point {
	TwoD(f32, f32),
	ThreeD(f32, f32, f32)
}

// you can also use impl on enums



fn main() {
	// kayn is my favorite champion btw :)
	let _kayn = Champion {
		name: String::from("Kayn"),
		damage_type: DamageType::AD,
		range_type: RangeType::Melee,
		resource: Resource::Mana,
		role: vec![Role::Jungle]
	};

	let _swain = Champion {
		name: String::from("Swain"),
		damage_type: DamageType::AP,
		range_type: RangeType::Ranged,
		resource: Resource::Mana,
		role: vec![Role::Support, Role::Mid]
	};


	let _point_one = Point::TwoD(-4.0, 5.0);
	let _point_two = Point::ThreeD(1.0, -7.0, 2.0);


	let some_x = Some(5);
	let none_y: Option<i32> = None;

	// 5
	some_x.expect("yippee");

	// error
	// y.expect("grahhh")

	// unwrapping x
	match some_x {
		Some(n) => n,
		None => 6
	};

	// unwrap w default
	some_x.unwrap_or(6); // 5
	none_y.unwrap_or(6); // 6

	// this is good for constants/slices/whatever but if you need a function call then use unwrap_or_else
	some_x.unwrap_or_else(|| 5 * 3); // 5
	none_y.unwrap_or_else(|| 5 * 3); // 15
}
