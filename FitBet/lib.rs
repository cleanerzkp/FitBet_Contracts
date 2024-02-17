#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod fitness_challenge {
    use ink::{prelude::string::String, storage::Mapping};

    #[ink(storage)]
    pub struct FitnessChallenge {
        challenges: Mapping<u32, Challenge>,
        challenge_count: u32,
    }

    #[ink(event)]
    pub struct ChallengeCreated {
        #[ink(topic)]
        challenge_id: u32,
        challenger: AccountId,
        wager: Balance,
        task_type: String,
    }

    #[ink(event)]
    pub struct ChallengeAccepted {
        #[ink(topic)]
        challenge_id: u32,
        acceptor: AccountId,
    }

    #[ink(event)]
    pub struct ResultsSubmitted {
        #[ink(topic)]
        challenge_id: u32,
        submitter: AccountId,
        result: u32,
    }

    #[ink(event)]
    pub struct WinnerDeclared {
        #[ink(topic)]
        challenge_id: u32,
        winner: AccountId,
        amount_won: Balance,
    }

    #[ink(event)]
    pub struct ChallengeCanceled {
        #[ink(topic)]
        challenge_id: u32,
        challenger: AccountId,
    }

    #[derive(Debug, Clone, PartialEq, scale::Encode, scale::Decode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
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
            Self {
                challenges: Mapping::new(),
                challenge_count: 0,
            }
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

            self.challenges.insert(challenge_id, &challenge);
            self.challenge_count = challenge_id;

            self.env().emit_event(ChallengeCreated {
                challenge_id,
                challenger: caller,
                wager,
                task_type: challenge.task_type.clone(),
            });

            Ok(challenge_id)
        }

        #[ink(message)]
        pub fn accept_challenge(&mut self, challenge_id: u32) -> Result<(), String> {
            let caller = self.env().caller();
            let mut challenge = self
                .challenges
                .get(&challenge_id)
                .ok_or("Challenge not found.")?;
            if challenge.acceptor.is_some() {
                return Err("Challenge already accepted.".into());
            }

            challenge.acceptor = Some(caller);
            challenge.start_time = Some(self.env().block_timestamp());
            self.challenges.insert(challenge_id, &challenge);
            self.env().emit_event(ChallengeAccepted {
                challenge_id,
                acceptor: caller,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn submit_results(&mut self, challenge_id: u32, result: u32) -> Result<(), String> {
            let caller = self.env().caller();
            let mut challenge = self
                .challenges
                .get(&challenge_id)
                .ok_or("Challenge not found.")?;
            if Some(caller) == Some(challenge.creator) {
                challenge.creator_result = Some(result);
            } else if Some(caller) == challenge.acceptor {
                challenge.acceptor_result = Some(result);
            } else {
                return Err("Unauthorized submission.".into());
            }
            self.challenges.insert(challenge_id, &challenge);

            self.env().emit_event(ResultsSubmitted {
                challenge_id,
                submitter: caller,
                result,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn determine_winner(&mut self, challenge_id: u32) -> Result<(), String> {
            let challenge = self
                .challenges
                .get(&challenge_id)
                .ok_or("Challenge not found.")?;
            if challenge.creator_result.is_none() || challenge.acceptor_result.is_none() {
                return Err("Results not fully submitted.".into());
            }

            let winner = if challenge.creator_result > challenge.acceptor_result {
                Some(challenge.creator)
            } else if challenge.creator_result < challenge.acceptor_result {
                challenge.acceptor
            } else {
                // Handle a tie or implement your own logic here
                None
            };

            if let Some(winner_id) = winner {
                self.env().emit_event(WinnerDeclared {
                    challenge_id,
                    winner: winner_id,
                    amount_won: challenge.wager * 2,
                });

                self.env()
                    .transfer(winner_id, challenge.wager * 2)
                    .map_err(|_| String::from("Transfer failed."))?;
            }

            Ok(())
        }

        #[ink(message)]
        pub fn cancel_challenge(&mut self, challenge_id: u32) -> Result<(), String> {
            let caller = self.env().caller();
            let challenge = self
                .challenges
                .get(&challenge_id)
                .ok_or("Challenge not found.")?;
            if challenge.creator != caller {
                return Err("Only creator can cancel.".into());
            }
            if challenge.acceptor.is_some() {
                return Err("Challenge already accepted.".into());
            }

            self.env()
                .transfer(caller, challenge.wager)
                .map_err(|_| String::from("Refund failed."))?;
            self.challenges.take(&challenge_id);

            self.env().emit_event(ChallengeCanceled {
                challenge_id,
                challenger: caller,
            });

            Ok(())
        }
    }
}
