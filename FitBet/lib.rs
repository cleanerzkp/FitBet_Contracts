#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod fitness_challenge {
    use ink_env::AccountId;
    use ink_storage::collections::HashMap as StorageMap;
    use ink_prelude::string::String;
    use ink_lang as ink;

    #[ink(storage)]
    pub struct FitnessChallenge {
        challenges: StorageMap<u32, Challenge>,
        challenge_count: u32,
    }

    #[derive(Debug, Clone, PartialEq, ink_storage::traits::SpreadAllocate, ink_storage::traits::PackedLayout, ink_storage::traits::SpreadLayout)]
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