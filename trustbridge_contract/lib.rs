#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod trustbridge_contract {
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    /// Define the contract's storage structure.
    #[ink(storage)]
    pub struct EscrowContract {
        /// The owner of the contract (the one who deposits funds).
        owner: AccountId,
        /// The beneficiary of the contract (the one who receives funds).
        beneficiary: AccountId,
        /// The amount of funds deposited in the contract.
        funds: Balance,
        /// Indicates whether the funds have been released.
        is_released: bool,
        /// Indicates whether the contract has been canceled.
        is_cancelled: bool,
    }

    /// Define custom errors for the contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Only the owner can perform this action.
        NotOwner,
        /// Only the beneficiary can perform this action.
        NotBeneficiary,
        /// The contract has already been finalized (released or canceled).
        AlreadyFinalized,
        /// There are insufficient funds in the contract.
        InsufficientFunds,
    }

    impl EscrowContract {
        /// Constructor that initializes the contract with an owner and a beneficiary.
        #[ink(constructor)]
        pub fn new(owner: AccountId, beneficiary: AccountId) -> Self {
            Self {
                owner,
                beneficiary,
                funds: 0,
                is_released: false,
                is_cancelled: false,
            }
        }

        /// Allows the owner to deposit funds into the contract.
        #[ink(message, payable)]
        pub fn deposit(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            // Add the transferred funds to the contract.
            self.funds += self.env().transferred_value();
            Ok(())
        }

        /// Allows the beneficiary to release the funds.
        #[ink(message)]
        pub fn release_funds(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            if caller != self.beneficiary {
                return Err(Error::NotBeneficiary);
            }

            if self.is_released || self.is_cancelled {
                return Err(Error::AlreadyFinalized);
            }

            if self.funds == 0 {
                return Err(Error::InsufficientFunds);
            }

            // Mark the funds as released.
            self.is_released = true;

            // Transfer the funds to the beneficiary.
            self.env()
                .transfer(self.beneficiary, self.funds)
                .expect("Transfer failed");

            Ok(())
        }

        /// Allows the owner to cancel the contract and refund the funds.
        #[ink(message)]
        pub fn cancel(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            if self.is_released || self.is_cancelled {
                return Err(Error::AlreadyFinalized);
            }

            if self.funds == 0 {
                return Err(Error::InsufficientFunds);
            }

            // Mark the contract as canceled.
            self.is_cancelled = true;

            // Refund the funds to the owner.
            self.env()
                .transfer(self.owner, self.funds)
                .expect("Transfer failed");

            Ok(())
        }

        /// Returns the amount of funds deposited in the contract.
        #[ink(message)]
        pub fn get_funds(&self) -> Balance {
            self.funds
        }

        /// Returns the status of the contract (released or canceled).
        #[ink(message)]
        pub fn get_status(&self) -> (bool, bool) {
            (self.is_released, self.is_cancelled)
        }
    }

    /// Unit tests for the contract.
    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_contract_works() {
            let owner = AccountId::from([0x1; 32]);
            let beneficiary = AccountId::from([0x2; 32]);
            let contract = EscrowContract::new(owner, beneficiary);

            assert_eq!(contract.get_funds(), 0);
            assert_eq!(contract.get_status(), (false, false));
        }

        #[ink::test]
        fn deposit_works() {
            let owner = AccountId::from([0x1; 32]);
            let beneficiary = AccountId::from([0x2; 32]);
            let mut contract = EscrowContract::new(owner, beneficiary);

            // Simulate a deposit of 100 units.
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(100);
            assert_eq!(contract.deposit(), Ok(()));
            assert_eq!(contract.get_funds(), 100);
        }

        #[ink::test]
        fn release_funds_works() {
            let owner = AccountId::from([0x1; 32]);
            let beneficiary = AccountId::from([0x2; 32]);
            let mut contract = EscrowContract::new(owner, beneficiary);

            // Simulate a deposit of 100 units.
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(100);
            assert_eq!(contract.deposit(), Ok(()));

            // Simulate the release of funds by the beneficiary.
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(beneficiary);
            assert_eq!(contract.release_funds(), Ok(()));
            assert_eq!(contract.get_status(), (true, false));
        }

        #[ink::test]
        fn cancel_works() {
            let owner = AccountId::from([0x1; 32]);
            let beneficiary = AccountId::from([0x2; 32]);
            let mut contract = EscrowContract::new(owner, beneficiary);

            // Simulate a deposit of 100 units.
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(100);
            assert_eq!(contract.deposit(), Ok(()));

            // Simulate the cancellation of the contract by the owner.
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
            assert_eq!(contract.cancel(), Ok(()));
            assert_eq!(contract.get_status(), (false, true));
        }
    }
}
