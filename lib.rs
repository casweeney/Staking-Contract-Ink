#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod staking {
    use ink::storage::Mapping;


    #[ink(storage)]
    pub struct Staking {
        deadline: u64,
        balances: ink::storage::Mapping<AccountId, Balance>,
    }

    impl Staking {
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            // Get the current timestamp from the environment
            let current_timestamp: u64 = Self::env().block_timestamp();
            let five_minutes: u64 = 5 * 60; // 5 minutes in seconds
            let deadline: u64 = current_timestamp + five_minutes;

            let balances = Mapping::default();

            Self { deadline: deadline, balances: balances }
        }

        #[ink(message, payable)]
        pub fn stake(&mut self) {
            let caller = self.env().caller();

            let balance = self.balances.get(caller).unwrap_or(0);

            let value = self.env().transferred_value();

            assert!(value > 0, "Insuficient funds");

            self.balances.insert(caller, &(balance + value));
        }

        
        #[ink(message)]
        pub fn withdraw(&mut self) {
            assert!(self.deadline < Self::env().block_timestamp(), "Deadline not reached");

            let caller = self.env().caller();
            
            let balance = self.balances.get(caller).unwrap();

            assert!(balance > 0, "No stake");

            self.balances.remove(caller);

            self.env().transfer(caller, balance).unwrap();
        }

        #[ink(message)]
        pub fn change_deadline(&mut self, dead_line: u64) {
            self.deadline = dead_line;
        }

        #[ink(message)]
        pub fn show_deadline(&self) -> u64 {
            self.deadline
        }

        #[ink(message)]
        pub fn show_user_balance(&self, user: AccountId) -> Balance {
            let balance = self.balances.get(user).unwrap();
            balance
        }
    }
}
