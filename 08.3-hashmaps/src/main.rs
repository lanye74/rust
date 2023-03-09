use std::collections::HashMap;



fn main() {
	// already figured out how to use these but it's nice to formally learn it ig

	let mut map: HashMap<&str, &str> = HashMap::new();

	map.insert("key", "value");

	let value = map.get("key").unwrap();

	// hashmaps take ownership of owned values; copies values with the Copy trait

	// overwrite
	map.insert("key", "hello");

	// insert if doesn't exist; or_insert returns mutable reference to value in map
	map.entry("key").or_insert("world");
}
