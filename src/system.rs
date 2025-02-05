use core::ops::AddAssign;
use num::traits::{One, Zero, CheckedAdd};
use std::collections::BTreeMap;

pub trait Config {
	type AccountId: Ord + Clone;
	type BlockNumber: Zero + One + AddAssign + CheckedAdd + Copy;
	type Nonce: Zero + One + Copy + CheckedAdd;
}

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.

#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// The current block number.
	/* TODO: Create a field `block_number` that stores a `u32`. */
	/// A map from an account to their nonce.
	/* TODO: Create a field `nonce` that is a `BTreeMap` from `String` to `u32`. */
	pub block_number: T::BlockNumber,
	pub nonce: BTreeMap<T::AccountId, T::Nonce>,
}

#[derive(Debug)]
pub enum SystemError {
	NonceOverflow,
	BlockNumberOverflow,
}

impl<T: Config> Pallet<T>
{
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		/* TODO: Return a new instance of the `Pallet` struct. */
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> T::BlockNumber {
		/* TODO: Return the current block number. */
		self.block_number
	}

	/// Increment the block number by one.
	/// Returns an error if the block number would overflow.
	pub fn inc_block_number(&mut self) -> Result<(), SystemError> {
		/* TODO: Increment the current block number by one. */
		self.block_number = self.block_number.checked_add(&T::BlockNumber::one())
			.ok_or(SystemError::BlockNumberOverflow)?;
		Ok(())
	}

	/// Increment the nonce of an account.
	/// Returns an error if the nonce would overflow.
	pub fn inc_nonce(&mut self, who: &T::AccountId) -> Result<(), SystemError> {
		/* TODO: Get the current nonce of `who`, and increment it by one. */
		let nonce: T::Nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
		let new_nonce = nonce.checked_add(&T::Nonce::one())
			.ok_or(SystemError::NonceOverflow)?;
		self.nonce.insert(who.clone(), new_nonce);
		Ok(())
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;
	impl super::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}
	#[test]
	fn init_system() {
		let mut system = super::Pallet::<TestConfig>::new();
		system.inc_block_number().unwrap();
		system.inc_nonce(&"alice".to_string()).unwrap();
		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce.get(&"alice".to_string()).copied().unwrap_or(0), 1);
		assert_eq!(system.nonce.get(&"bob".to_string()).copied().unwrap_or(0), 0);
	}
}
