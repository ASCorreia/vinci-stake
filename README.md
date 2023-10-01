
## Vinci World

Vinci World goal is to be a revolutionary blockchain-based gaming platform, intertwining the power of artificial intelligence (AI) with the thrilling world of non-fungible tokens (NFTs).
Players will be able to train different characters using reinforcement learning and later on they will be able to mint them as NFTs and enter different tournaments / challenges.

## Vinci World Programs - Current state

Vinci Accounts - [https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-accounts](https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-accounts).
- Program that will be responsible for storing the Vinci User relavant data


Vinci Stake - [https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-stake](https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-stake).
- Program where the user will be able to stake their NFTs


Vinci Swap - [https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-swap](https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-swap).
- Program where the user will be able to trade assets to improve his character


Vinci Rewards - [https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-rewards](https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-rewards).
- Program responsible for deploying rewards systems as needed


Vinci Quiz - [https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-rewards](https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-quiz).
- Program that has a tournament system impletmented in order to update player scores, update leaderboard and give rewards according to the leaderboard

## Architecture

The Vinci World Programs consists of different programs that will enhance the user experience.

- There will user accounts created and maintained by the Vinci Accounts program. In this program, "sensitive" information about the user will be stored.
- The user will be able to stake their NFTs (Trained models / agents / characters) in a custodial and non-custodial way in order to collect rewards (from tokens, to training time..) through the staking program. This program has, as well, methos that will be calle din order to update the staking details and to call the Vinci Rewards Program through CPIs
- The Vinci Rewards program will be updated regularly (according to different seasons), in order to distribute the appropriate rewards. To do so, it might interact with different Vinci World programs.
- In the Vinci Swap program, the user will be able to swap different tokens that will affect the behavior of their trained model. It can and should be used to fine tune the model without the need to re-train the model.

The Vinci Quiz program is a game to be used as  alaunching event, where players will compete in a Quiz game that has a tournament system impletmented in order to update player scores, update leaderboard and give rewards according to the leaderboard

## Quick game walkthrough

To Be Updated
