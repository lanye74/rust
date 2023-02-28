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



// pub use world::hello;
// re-export so that external files can now call lib::hello::say();



mod module2 {
	// use statements must be module level
	use crate::world::hello as HelloModule;

	pub fn speak() {
		HelloModule::say();
	}
}




fn main() {
	hello_world::say();

	// absolute path:
	// crate::world::hello::say();
	world::hello::say();

	// use world::hello;
	// hello::say();


	module2::speak();

}
