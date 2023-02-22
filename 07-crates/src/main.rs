mod hello_world;

mod world {
	pub mod hello {
		pub fn say() {
			println!("World, hello!");
		}
	}
}



fn main() {
	hello_world::say();

	world::hello::say();
}
