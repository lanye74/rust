mod hello_world;

mod world {
	pub mod hello {
		pub fn say() {
			println!("World, hello!");

			super::outside();
		}
	}

	fn outside() {
		println!("Hello from outside!");
	}
}




fn main() {
	hello_world::say();

	world::hello::say();


	// use world::hello;
	// hello::say();
}
