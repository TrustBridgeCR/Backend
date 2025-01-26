#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod trustbridge_contract {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct TrustbridgeContract {
        escrows: Mapping<u32, EscrowDetails>,
        next_escrow_id: u32,
        admin: AccountId,
    }

    #[derive(scale::Decode, scale::Encode, Clone)]
    #[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq, scale_info::TypeInfo))]
    pub struct EscrowDetails {
        amount: Balance,
        owner: AccountId,
        beneficiary: AccountId,
        arbiter: AccountId,
        is_active: bool,
    }

    #[ink(event)]
    pub struct EscrowCreated {
        #[ink(topic)]
        escrow_id: u32,
        amount: Balance,
    }

    #[ink(event)]
    pub struct FundsReleased {
        #[ink(topic)]
        escrow_id: u32,
        amount: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientFunds,
        NotAuthorized,
        EscrowNotFound,
        EscrowNotActive,
    }

    impl TrustbridgeContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                escrows: Mapping::new(),
                next_escrow_id: 0,
                admin: Self::env().caller(),
            }
        }

        #[ink(message, payable)]
        pub fn create_escrow(
            &mut self,
            beneficiary: AccountId,
            arbiter: AccountId,
        ) -> Result<(), Error> {
            let caller = self.env().caller();
            let amount = self.env().transferred_value();
            let escrow_id = self.next_escrow_id;

            let escrow = EscrowDetails {
                amount,
                owner: caller,
                beneficiary,
                arbiter,
                is_active: true,
            };

            self.escrows.insert(escrow_id, &escrow);
            self.next_escrow_id += 1;

            self.env().emit_event(EscrowCreated { escrow_id, amount });

            Ok(())
        }

        #[ink(message)]
        pub fn release_funds(&mut self, escrow_id: u32) -> Result<(), Error> {
            let escrow = self.escrows.get(&escrow_id).ok_or(Error::EscrowNotFound)?;

            if !escrow.is_active {
                return Err(Error::EscrowNotActive);
            }

            if self.env().caller() != escrow.arbiter {
                return Err(Error::NotAuthorized);
            }

            if !escrow.is_active {
                return Err(Error::EscrowNotActive);
            }

            self.env()
                .transfer(escrow.beneficiary, escrow.amount)
                .map_err(|_| Error::InsufficientFunds)?;

            let mut updated_escrow = escrow.clone();
            updated_escrow.is_active = false;
            self.escrows.insert(escrow_id, &updated_escrow);

            self.env().emit_event(FundsReleased {
                escrow_id,
                amount: escrow.amount,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn get_escrow(&self, escrow_id: u32) -> Option<EscrowDetails> {
            self.escrows.get(&escrow_id)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn create_escrow_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut contract = TrustbridgeContract::new();

            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(100);
            assert!(contract
                .create_escrow(accounts.bob, accounts.charlie)
                .is_ok());

            let escrow = contract.get_escrow(0).unwrap();
            assert_eq!(escrow.amount, 100);
            assert_eq!(escrow.beneficiary, accounts.bob);
            assert_eq!(escrow.arbiter, accounts.charlie);
        }
    }
}
