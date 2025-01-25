mod balances;
mod system;

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
pub struct Runtime {
	/* TODO:
		- Create a field `system` which is of type `system::Pallet`.
		- Create a field `balances` which is of type `balances::Pallet`.
	*/
	pub system: system::Pallet,
	pub balances: balances::Pallet,
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		/* TODO: Create a new `Runtime` by creating new instances of `system` and `balances`. */
		Runtime { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}

fn main() {
	println!("Hello, world!");
}
