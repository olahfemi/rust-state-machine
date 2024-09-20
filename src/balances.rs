use std::collections::BTreeMap;
use crate::types::AccountId;
use crate::types::Balance;
use num::Zero;
use num::CheckedAdd;
use num::CheckedSub;

// This is the Balances Module.
// It is a simple module which keeps track of how much balance each account has in this state
// machine.
/* TODO: Add the derive macro to implement the `Debug` trait for `Pallet`. */
#[derive(Debug)]
pub struct Pallet<AccountId, Balance> {
	// A simple storage mapping from accounts (`String`) to their balances (`u128`).
	balances: BTreeMap<AccountId, Balance>,
}

impl<AccountId, Balance> Pallet<AccountId, Balance>
where
    AccountId: Ord + Clone,
    Balance: Zero + CheckedSub + CheckedAdd + Copy, {
	// Create a new instance of the balances module.
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	// Set the balance of an account `who` to some `amount`.
	pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
		self.balances.insert(who.clone(), amount);
	}

	// Get the balance of an account `who`.
	// If the account has no stored balance, we return zero.
	pub fn balance(&self, who: &AccountId) -> Balance {
		*self.balances.get(who).unwrap_or(&Balance::zero())
	}

	pub fn transfer(
		&mut self,
		caller: AccountId,
		to: AccountId,
		amount: Balance,
	) -> Result<(), &'static str> {
		// Get the balance of account `caller`.
		let caller_balance = self.balance(&caller);
		// Get the balance of account `to`.
		let to_balance = self.balance(&to);

		// Use safe math to calculate a `new_caller_balance`.
		let new_caller_balance = caller_balance.checked_sub(&amount).ok_or("Not enough funds.")?;
		// Use safe math to calculate a `new_to_balance`.
		let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

		// Insert the new balance of `caller`.
		self.balances.insert(caller, new_caller_balance);
		// Insert the new balance of `to`.
		self.balances.insert(to, new_to_balance);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::<&'static str, u128>::new();


		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}

	#[test]
	fn transfer_balance() {
		let mut balances = super::Pallet::new();

		assert_eq!(
			balances.transfer("alice".to_string(), "bob".to_string(), 10),
			Err("Not enough funds.")
		);

		balances.set_balance(&"alice".to_string(), 10);
		assert_eq!(balances.transfer("alice".to_string(), "bob".to_string(), 10), Ok(()));
		assert_eq!(balances.balance(&"alice".to_string()), 0);
		assert_eq!(balances.balance(&"bob".to_string()), 10);
		assert_eq!(
			balances.transfer("alice".to_string(), "bob".to_string(), 10),
			Err("Not enough funds.")
		);
	}
}
