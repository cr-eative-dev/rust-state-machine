use core::ops::AddAssign;
use num::traits::{One, Zero, CheckedAdd};
use std::collections::BTreeMap;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.

#[derive(Debug)]
pub struct Pallet<AccountID, BlockNumber, Nonce> {
	/// The current block number.
	/* TODO: Create a field `block_number` that stores a `u32`. */
	/// A map from an account to their nonce.
	/* TODO: Create a field `nonce` that is a `BTreeMap` from `String` to `u32`. */
	pub block_number: BlockNumber,
	pub nonce: BTreeMap<AccountID, Nonce>,
}

#[derive(Debug)]
pub enum SystemError {
	NonceOverflow,
	BlockNumberOverflow,
}

impl<AccountID, BlockNumber, Nonce> Pallet<AccountID, BlockNumber, Nonce>
where
	AccountID: Ord + Clone,
	BlockNumber: Zero + One + AddAssign + Copy + CheckedAdd,
	Nonce: Zero + One + Copy + CheckedAdd,
{
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		/* TODO: Return a new instance of the `Pallet` struct. */
		Pallet { block_number: BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> BlockNumber {
		/* TODO: Return the current block number. */
		self.block_number
	}

	/// Increment the block number by one.
	/// Returns an error if the block number would overflow.
	pub fn inc_block_number(&mut self) -> Result<(), SystemError> {
		/* TODO: Increment the current block number by one. */
		self.block_number = self.block_number.checked_add(&BlockNumber::one())
			.ok_or(SystemError::BlockNumberOverflow)?;
		Ok(())
	}

	/// Increment the nonce of an account.
	/// Returns an error if the nonce would overflow.
	pub fn inc_nonce(&mut self, who: &AccountID) -> Result<(), SystemError> {
		/* TODO: Get the current nonce of `who`, and increment it by one. */
		let nonce = *self.nonce.get(who).unwrap_or(&Nonce::zero());
		let new_nonce = nonce.checked_add(&Nonce::one())
			.ok_or(SystemError::NonceOverflow)?;
		self.nonce.insert(who.clone(), new_nonce);
		Ok(())
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn init_system() {
		let mut system = super::Pallet::<String, u32, u32>::new();
		system.inc_block_number().unwrap();
		system.inc_nonce(&"alice".to_string()).unwrap();
		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce.get(&"alice".to_string()).copied().unwrap_or(0), 1);
		assert_eq!(system.nonce.get(&"bob".to_string()).copied().unwrap_or(0), 0);
	}
}
