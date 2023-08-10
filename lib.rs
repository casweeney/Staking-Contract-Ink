#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod staking {
    use ink::storage::Mapping;


    // Setting storage for state variables
    #[ink(storage)]
    pub struct Staking {
        deadline: u64,
        balances: Mapping<AccountId, Balance>,
    }

    // Events
    #[ink(event)]
    pub struct Staked {
        #[ink(topic)] // -> indexed
        caller: AccountId,
        value: Balance,
    }

    #[ink(event)]
    pub struct Withdrawn {
        #[ink(topic)] // -> indexed
        caller: AccountId,
        value: Balance,
    }

    #[ink(event)]
    pub struct DeadlineUpdated {
        #[ink(topic)] // -> indexed
        deadline: u64
    }

    #[ink(impl)]
    impl Staking {
        fn stake_now(&mut self, caller: AccountId, value: Balance) {
            self.balances.insert(caller, &value);
        }
    }

    // Implementation of contract functions
    #[ink(impl)]
    impl Staking {
        // Constructor function: an ink contract must have at least one constructor function
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            // Get the current timestamp from the environment
            let current_timestamp: u64 = Self::env().block_timestamp();
            let five_minutes: u64 = 5 * 60; // 5 minutes in seconds
            let deadline: u64 = current_timestamp + five_minutes;

            let balances = Mapping::default();

            Self { deadline: deadline, balances: balances }
        }

        // Stake function: Called to stake a value
        #[ink(message, payable)]
        pub fn stake(&mut self) {
            let caller = self.env().caller();

            let balance = self.balances.get(caller).unwrap_or(0);

            let value = self.env().transferred_value();

            assert!(value > 0, "Insuficient funds");

            self.stake_now(caller, balance + value);

            self.env().emit_event(Staked {
                caller: caller,
                value: value
            });
        }

        // Withdraw function: called to withdraw staked value
        #[ink(message)]
        pub fn withdraw(&mut self) {
            assert!(self.deadline < Self::env().block_timestamp(), "Deadline not reached");

            let caller = self.env().caller();

            let balance = self.balances.get(caller).unwrap_or(0);

            assert!(balance > 0, "No stake");

            self.balances.remove(caller);

            self.env().transfer(caller, balance).unwrap();

            self.env().emit_event(Withdrawn {
                caller: caller,
                value: balance
            });
        }

        // Function to change the deadline
        #[ink(message)]
        pub fn change_deadline(&mut self, dead_line: u64) {
            let current_timestamp: u64 = Self::env().block_timestamp();

            self.deadline = current_timestamp + dead_line;

            self.env().emit_event(DeadlineUpdated {
                deadline: dead_line
            });
        }

        // Function to show the current deadline
        #[ink(message)]
        pub fn show_deadline(&self) -> u64 {
            self.deadline
        }

        // function to return the current staked value of the caller
        #[ink(message)]
        pub fn show_user_balance(&self, user: AccountId) -> Balance {
            let balance = self.balances.get(user).unwrap_or(0);
            balance
        }
    }
}
