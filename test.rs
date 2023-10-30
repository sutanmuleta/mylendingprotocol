use ink_lang as ink;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit() {
        let mut contract = LendingProtocol::new(vec![b"TOKEN"]);
        contract.deposit(AccountId::from([0x0; 32]), b"TOKEN", 100);

        // Assuming a get_balance() function is available in the contract
        assert_eq!(contract.get_balance(AccountId::from([0x0; 32]), b"TOKEN"), Some(100));
    }

    #[test]
    fn test_borrow() {
        let mut contract = LendingProtocol::new(vec![b"TOKEN"]);
        contract.deposit(AccountId::from([0x0; 32]), b"TOKEN", 100);
        assert!(contract.borrow(AccountId::from([0x0; 32]), b"TOKEN", 50));

        // Assuming a get_borrowing_balance() function is available in the contract
        assert_eq!(contract.get_borrowing_balance(AccountId::from([0x0; 32]), b"TOKEN"), Some(50));
    }

    #[test]
    fn test_repay() {
        let mut contract = LendingProtocol::new(vec![b"TOKEN"]);
        contract.deposit(AccountId::from([0x0; 32]), b"TOKEN", 100);
        contract.borrow(AccountId::from([0x0; 32]), b"TOKEN", 50);
        contract.repay(AccountId::from([0x0; 32]), b"TOKEN", 25);

        // Assuming a get_borrowing_balance() function is available in the contract
        assert_eq!(contract.get_borrowing_balance(AccountId::from([0x0; 32]), b"TOKEN"), Some(25));
    }

    #[test]
    fn test_calculate_interest() {
        let mut contract = LendingProtocol::new(vec![b"TOKEN"]);
        contract.deposit(AccountId::from([0x0; 32]), b"TOKEN", 100);

        // Simulating the passage of time, perhaps using a helper in the contract
        contract.simulate_time_passage(5 * 24 * 60 * 60);

        contract.calculate_interest(AccountId::from([0x0; 32]), b"TOKEN");

        // Assuming the deposit balance gets updated with the earned interest
        assert_eq!(contract.get_balance(AccountId::from([0x0; 32]), b"TOKEN"), Some(105));
    }
}
