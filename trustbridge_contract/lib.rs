#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod trustbridge_contract {
    #[ink(storage)]
    pub struct TrustbridgeContract {
        funds: Balance,
        owner: AccountId,
        beneficiary: AccountId,
        arbiter: AccountId,
        is_released: bool,
        is_cancelled: bool,
    }

    #[ink(event)]
    pub struct FundsDeposited {
        #[ink(topic)]
        amount: Balance,
    }

    #[ink(event)]
    pub struct FundsReleased {
        #[ink(topic)]
        beneficiary: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct FundsRefunded {
        #[ink(topic)]
        owner: AccountId,
        amount: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        NotAuthorized,
        AlreadyReleased,
        AlreadyCancelled,
    }

    type Result<T> = core::result::Result<T, Error>;

    impl TrustbridgeContract {
        #[ink(constructor)]
        pub fn new(beneficiary: AccountId, arbiter: AccountId) -> Self {
            Self {
                funds: 0,
                owner: Self::env().caller(),
                beneficiary,
                arbiter,
                is_released: false,
                is_cancelled: false,
            }
        }

        #[ink(message, payable)]
        pub fn deposit(&mut self) -> Result<()> {
            let deposit_amount = self.env().transferred_value();
            self.funds += deposit_amount;

            self.env().emit_event(FundsDeposited {
                amount: deposit_amount,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn release_funds(&mut self) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.arbiter {
                return Err(Error::NotAuthorized);
            }
            if self.is_released || self.is_cancelled {
                return Err(Error::AlreadyReleased);
            }

            self.is_released = true;
            if self.env().transfer(self.beneficiary, self.funds).is_err() {
                return Err(Error::InsufficientBalance);
            }

            self.env().emit_event(FundsReleased {
                beneficiary: self.beneficiary,
                amount: self.funds,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn cancel_escrow(&mut self) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.arbiter {
                return Err(Error::NotAuthorized);
            }
            if self.is_released || self.is_cancelled {
                return Err(Error::AlreadyCancelled);
            }

            self.is_cancelled = true;
            if self.env().transfer(self.owner, self.funds).is_err() {
                return Err(Error::InsufficientBalance);
            }

            self.env().emit_event(FundsRefunded {
                owner: self.owner,
                amount: self.funds,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn get_balance(&self) -> Balance {
            self.funds
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn deposit_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut contract = TrustbridgeContract::new(accounts.bob, accounts.charlie);

            assert_eq!(contract.get_balance(), 0);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(100);
            assert!(contract.deposit().is_ok());
            assert_eq!(contract.get_balance(), 100);
        }
    }
}
