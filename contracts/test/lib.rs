#[cfg(test)]
mod tests {
    use super::*;
    use ink::env::test;

    #[ink::test]
    fn create_escrow_works() {
        let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
        let mut contract = TrustbridgeContract::new();

        test::set_value_transferred::<ink::env::DefaultEnvironment>(100);
        assert!(contract
            .create_escrow(accounts.bob, accounts.charlie)
            .is_ok());

        let escrow = contract.get_escrow(0).unwrap();
        assert_eq!(escrow.amount, 100);
        assert_eq!(escrow.beneficiary, accounts.bob);
        assert_eq!(escrow.arbiter, accounts.charlie);
    }
}
