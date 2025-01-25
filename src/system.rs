/* TODO: You might need to update your imports. */
use std::collections::BTreeMap;
/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
pub struct Pallet {
	/// The current block number.
	/* TODO: Create a field `block_number` that stores a `u32`. */
	/// A map from an account to their nonce.
	/* TODO: Create a field `nonce` that is a `BTreeMap` from `String` to `u32`. */
	pub block_number: u32,
	pub nonce: BTreeMap<String, u32>,
}

#[derive(Debug)]
pub enum SystemError {
	NonceOverflow,
	BlockNumberOverflow,
}

impl Pallet {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		/* TODO: Return a new instance of the `Pallet` struct. */
		Pallet { block_number: 0, nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> u32 {
		/* TODO: Return the current block number. */
		self.block_number
	}

	/// Increment the block number by one.
	/// Returns an error if the block number would overflow.
	pub fn inc_block_number(&mut self) -> Result<(), SystemError> {
		/* TODO: Increment the current block number by one. */
		self.block_number =
			self.block_number.checked_add(1).ok_or(SystemError::BlockNumberOverflow)?;
		Ok(())
	}

	/// Increment the nonce of an account.
	/// Returns an error if the nonce would overflow.
	pub fn inc_nonce(&mut self, who: &String) -> Result<(), SystemError> {
		/* TODO: Get the current nonce of `who`, and increment it by one. */
		let nonce = self.nonce.get(who).copied().unwrap_or(0);
		let new_nonce = nonce.checked_add(1).ok_or(SystemError::NonceOverflow)?;
		self.nonce.insert(who.clone(), new_nonce);
		Ok(())
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn init_system() {
		/* TODO: Create a test which checks the following:
			- Increment the current block number.
			- Increment the nonce of `alice`.

			- Check the block number is what we expect.
			- Check the nonce of `alice` is what we expect.
			- Check the nonce of `bob` is what we expect.
		*/
		let mut system = super::Pallet::new();
		system.inc_block_number().unwrap();
		system.inc_nonce(&"alice".to_string()).unwrap();
		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce.get(&"alice".to_string()).copied().unwrap_or(0), 1);
		assert_eq!(system.nonce.get(&"bob".to_string()).copied().unwrap_or(0), 0);
	}
}
