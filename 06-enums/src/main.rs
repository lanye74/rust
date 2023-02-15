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



fn main() {
	// kayn is my favorite champion btw :)
	let kayn = Champion {
		name: String::from("Kayn"),
		damage_type: DamageType::AD,
		range_type: RangeType::Melee,
		resource: Resource::Mana,
		role: vec![Role::Jungle]
	};

	let swain = Champion {
		name: String::from("Swain"),
		damage_type: DamageType::AP,
		range_type: RangeType::Ranged,
		resource: Resource::Mana,
		role: vec![Role::Mid, Role::Support]
	};
}
