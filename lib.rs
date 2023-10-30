use ink_lang as ink;

#[ink::contract]
mod lending_protocol {
    use ink_prelude::collections::BTreeMap as StorageHashMap;
    use ink_storage::traits::SpreadLayout;
    use ink_env::call::FromAccountId;

    #[derive(Debug, Clone, SpreadLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Account {
        deposits: StorageHashMap<[u8; 5], Balance>,
        borrowings: StorageHashMap<[u8; 5], Balance>,
        last_updated: Timestamp,
    }

    #[derive(Default)]
    #[ink(storage)]
    pub struct LendingProtocol {
        accounts: StorageHashMap<AccountId, Account>,
        supported_tokens: StorageHashMap<[u8; 5], bool>,
    }

    type Balance = u128;
    type Timestamp = u64;
    type AccountId = AccountHash; 

    impl LendingProtocol {
        #[ink(constructor)]
        pub fn new(supported_tokens: ink_prelude::vec::Vec<[u8; 5]>) -> Self {
            let mut instance = Self::default();
            for token in supported_tokens {
                instance.supported_tokens.insert(token, true);
            }
            instance
        }

        #[ink(message)]
        pub fn add_token(&mut self, symbol: [u8; 5]) {
            self.supported_tokens.insert(symbol, true);
        }

        #[ink(message)]
        pub fn deposit(&mut self, account_id: AccountId, symbol: [u8; 5], amount: Balance) {
            if !self.supported_tokens.contains_key(&symbol) {
                ink_env::debug_println!("Token not supported");
                return;
            }

            let account = self.accounts.entry(account_id).or_insert(Account::default());
            account.deposit(symbol, amount);
        }

        #[ink(message)]
        pub fn borrow(&mut self, account_id: AccountId, symbol: [u8; 5], amount: Balance) -> bool {
            if !self.supported_tokens.contains_key(&symbol) {
                ink_env::debug_println!("Token not supported");
                return false;
            }

            if let Some(account) = self.accounts.get_mut(&account_id) {
                account.borrow(symbol, amount)
            } else {
                false
            }
        }

        #[ink(message)]
        pub fn repay(&mut self, account_id: AccountId, symbol: [u8; 5], amount: Balance) {
            if let Some(account) = self.accounts.get_mut(&account_id) {
                account.repay(symbol, amount);
            }
        }

        #[ink(message)]
        pub fn calculate_interest(&mut self, account_id: AccountId, symbol: [u8; 5]) {
            if let Some(account) = self.accounts.get_mut(&account_id) {
                account.calculate_interest(symbol);
            }
        }
    }

    impl Account {
        pub fn deposit(&mut self, symbol: [u8; 5], amount: Balance) {
            let current_balance = self.deposits.entry(symbol).or_insert(0);
            *current_balance += amount;
        }

        pub fn borrow(&mut self, symbol: [u8; 5], amount: Balance) -> bool {
            if let Some(deposit) = self.deposits.get(&symbol) {
                if *deposit >= amount {
                    let current_borrowing = self.borrowings.entry(symbol).or_insert(0);
                    *current_borrowing += amount;
                    return true;
                }
            }
            false
        }

        pub fn repay(&mut self, symbol: [u8; 5], amount: Balance) {
            if let Some(borrowing) = self.borrowings.get_mut(&symbol) {
                if *borrowing >= amount {
                    *borrowing -= amount;
                }
            }
        }

        pub fn calculate_interest(&mut self, symbol: [u8; 5]) {
            let now = Self::current_time();
            let duration = now - self.last_updated;

            const INTEREST_RATE: u128 = 5;
            const DURATION: Timestamp = 5 * 24 * 60 * 60; // 5 days in seconds

            if let Some(deposit) = self.deposits.get_mut(&symbol) {
                if duration >= DURATION {
                    let periods = duration / DURATION;
                    *deposit += *deposit * INTEREST_RATE / 100 * periods as u128;
                    self.last_updated = now;
                }
            }
        }

        pub fn current_time() -> Timestamp {
            // Implementation here to get the current time
            // You might use the `block_timestamp` function from the `ink_env` crate
            0 // Placeholder, replace this
        }
    }
}
