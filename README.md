
## Vinci World 🌊🌊

Vinci World goal is to be a revolutionary blockchain-based gaming platform, intertwining the power of artificial intelligence (AI) with the thrilling world of non-fungible tokens (NFTs).
Players will be able to train different characters using reinforcement learning and later on they will be able to mint them as NFTs and enter different tournaments / challenges.

## Vinci World Programs - Current state and Architecture

### Vinci Accounts 👨‍💻 - [https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-accounts](https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-accounts).
- Program that will be responsible for storing the Vinci User relavant data

In this program, all the user relevant data (amount of Vinci Tokens, general score, etc) will be stored. This information can then be accessed by other Vinci Programs through Cross Program Invocations
The Vinci Accounts programs also provides mint and burn functions for tokens and NFTs   

***************************************************************************************************************************************************************************************

### Vinci Stake 🥩 - [https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-stake](https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-stake).
- Program where the user will be able to stake their NFTs

Vinci Stake program allows user to stake and unstake their NFTs in a custodial and non custodial way (The NFT will be frozen in the user wallet).   

***************************************************************************************************************************************************************************************

### Vinci Swap 💱 - [https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-swap](https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-swap).
- Program where the user will be able to trade assets to improve his character

In this program, a Swap program is being built. This swap program will allow users to trade assets (tokens) in order to fine tune their model training. This swap program is backed by a constant product algorithm, making the price of a certain token vary according to the available liquidity. By doing this, we ensure that the gameplay remains strategic and addictive.   

***************************************************************************************************************************************************************************************

### Vinci Rewards 🥇 - [https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-rewards](https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-rewards).
- Program responsible for deploying rewards systems as needed

Vinci Rewards will the "middleware" between all the programs when rewards need to be distributed. If a match / tournament / PvP match if finished, the Rewards program will be the one responsible to distribute Cross Program Invocations depending on the use case.
The same concept applies for unstaking rewards.   

***************************************************************************************************************************************************************************************

### Vinci Quiz 🏎️ - [https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-quiz](https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-quiz).
- Program that has a tournament system impletmented in order to update player scores, update leaderboard and give rewards according to the leaderboard

## Programs Interaction

The Vinci World Programs consists of different programs that will enhance the user experience.

Vinci Accounts will be the "main" program storing user information. The only direct calls to this programs are done to create an account and to set a general score.
Vinci Stake will be responsible for the staking and unstaking of user NFTs. Any staking reward obtained will be routed to the rewards program that will, if needed, re-route that operation to the vinci accounts program (in case on Vinci Points rewards)
Vinci Swap is where the user will experience some DeFi concepts by being able to trade tokens to fine tune their model. Currently, there no CPIs between the swap program and any other program (the user token amount will fetched directly from the associated token account)
Vinci Quiz program is the tournament / seasons program. In here, competitions will be created as accounts an dusers will be able to join them. The program will reallocate the acocunt size by every entrance. The program is, as well, responsible for setting the score of each player and ordering the leaderboard. Calls to the rewards might be performed
Vinci Reward will create CPIs to almost every program. It is responsible to calculate user rewards based on program / reward factor and will process it accordingly.

## Quick game walkthrough

To Be Updated
