# FitBet_Contracts

## Overview

FitBet_Contracts is a blockchain-based project developed for DegenHack, featuring an ink! smart contract designed to power the FitBet application. This application facilitates fitness challenges on the blockchain, allowing users to commit to fitness goals, wager cryptocurrency on their achievements, and earn rewards based on their success. The contract ensures a transparent, fair, and secure process for managing these challenges and wagers.

### Key Features

- **Challenge Creation and Acceptance:** Enables users to create and accept challenges, specifying goals and wagers.
- **Dynamic Participation:** Supports updating the participants of a challenge, with the ability to dynamically manage challengers and acceptors.
- **Result Submission:** Allows participants to submit their results, ensuring an immutable record on the blockchain.
- **Winner Determination:** Implements logic to impartially determine the winner and distribute the wager accordingly.
- **Challenge Lifecycle Management:** Manages the entire lifecycle of each challenge, including cancellation under specific conditions.

## Contract Structure

- **Challenges Mapping:** Utilizes a dynamic mapping to associate unique IDs with `Challenge` structs, containing detailed information about each challenge.
- **Events:** Defines custom events such as `ChallengeCreated`, `ChallengeAccepted`, `ResultsSubmitted`, and `WinnerDeclared` to facilitate tracking of contract activities.
- **Challenge Struct:** Stores details about each challenge, including information about the creator, acceptor, task type, wager, and results.

## Usage

Designed for interaction through blockchain transactions, enabling users to:
- Create new fitness challenges with specific goals and wager amounts.
- Accept challenges by matching the wager and committing to the goal.
- Submit their results for evaluation by the contract.
- Query the status of challenges, including active challenges and end conditions.


## License

FitBet_Contracts is open-source software licensed under the MIT License.
