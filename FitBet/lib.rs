#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod fitness_challenge {
    use ink_lang as ink;

    #[ink(storage)]
    pub struct FitnessChallenge {
    }

    impl FitnessChallenge {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { }
        }
    }
}
