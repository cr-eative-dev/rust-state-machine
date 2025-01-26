mod balances;
mod system;

mod types {
	pub type AccountID = String;
	pub type Balance = u128;
	pub type Nonce = u32;
	pub type BlockNumber = u32;
}

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<types::AccountID, types::BlockNumber, types::Nonce>,
	balances: balances::Pallet<types::AccountID, types::Balance>,
}

impl Runtime {
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}

fn main() {
	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

	runtime.balances.set_balance(&alice, 100);

	runtime.system.inc_block_number().unwrap();
	assert_eq!(runtime.system.block_number(), 1);

	runtime.system.inc_nonce(&alice).unwrap();
	let _res = runtime
		.balances
		.transfer(alice.clone(), bob, 30)
		.map_err(|e| eprintln!("{}", e));

	runtime.system.inc_nonce(&alice).unwrap();
	let _res = runtime.balances.transfer(alice, charlie, 20).map_err(|e| eprintln!("{}", e));

	println!("{:?}", runtime);
}
