#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod fitness_challenge {
    use ink_env::{AccountId, Balance, Timestamp};
    use ink_storage::{
        collections::HashMap as StorageMap,
        traits::{PackedLayout, SpreadAllocate, SpreadLayout},
    };
    use ink_prelude::string::String;
    use ink_lang as ink;

    #[ink(storage)]
    pub struct FitnessChallenge {
        challenges: StorageMap<u32, Challenge>,
        challenge_count: u32,
    }

    #[derive(Debug, Clone, PartialEq, PackedLayout, SpreadLayout, SpreadAllocate)]
    #[cfg_attr(feature = "std", derive(ink_storage::traits::StorageLayout))]
    pub struct Challenge {
        creator: AccountId,
        acceptor: Option<AccountId>,
        task_type: String,
        wager: Balance,
        start_time: Option<Timestamp>,
        creator_result: Option<u32>,
        acceptor_result: Option<u32>,
    }

    impl FitnessChallenge {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message, payable)]
        pub fn create_challenge(&mut self, task_type: String) -> Result<u32, String> {
            let caller = self.env().caller();
            let wager = self.env().transferred_value();
            let challenge_id = self.challenge_count + 1;

            let challenge = Challenge {
                creator: caller,
                acceptor: None,
                task_type,
                wager,
                start_time: None,
                creator_result: None,
                acceptor_result: None,
            };

            self.challenges.insert(challenge_id, challenge);
            self.challenge_count = challenge_id;

            Ok(challenge_id)
        }
    }

    impl Default for FitnessChallenge {
        fn default() -> Self {
            Self {
                challenges: StorageMap::new(),
                challenge_count: 0,
            }
        }
    }
}