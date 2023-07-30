
## Vinci World

Vinci World goal is to be a revolutionary blockchain-based gaming platform, intertwining the power of artificial intelligence (AI) with the thrilling world of non-fungible tokens (NFTs).
Players will be able to train different characters using reinforcement learning and later on they will be able to mint them as NFTs and enter different tournaments / challenges.

## Vinci World Quiz - Made for Speedrun Hackhaton

For the Speedrun Hachaton we created a Quiz Game to be used as a first launching event for the Vinci World Project.

In this Quiz, users will have the chance to answer questions about the Vinci Quiz lore, Solana and Reinforcement Learning. Every correct answer will award 30 points to the user and for every non correct answer, the user will be deducted a random amount of points (between 0 and 20).

For the hackhaton, the user will be able to upgrade his level by 1 for every 30 points. When the user reaches level 3 or higher, he will be able to mint a NFT that will allow him to have a whitelist for the project (note that the upgrade points and mega upgrade level will be higher upon mainnet launch. These values are only for the hackhaton on devnet).

Additionaly, 0.4 bSOL will be awarded every day to the top3 users. 

## Architecture

The Vinci World Quiz game is implemented on-chain and can be found here [https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-quiz](https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-quiz).

The on chain program consists of a Quiz Account that is composed by a vector os user entries structures. Those structures will save the user pubkey, their level, their score, and a boolean stating if they claimed their whitelist NFT).

The program will receive request from the client (Create Quiz, Add user to Quiz, Update Score, Get Scores, Upgrade, Mega Upgrade, Distribute Rewards, Close Quiz) and will perform all the logic:
- Upon receiving a new player, that player struct will be initialized and added to the Quiz. After that, the quiz leaderboard (scores) will be sorted from high score to lowest score.
- Upon receiving an update score, the program will update the player score (If it is a correct answer, the program will had 30 points to the player score. If the answer is not crrect, it will deduct a random amount between 0 and 20). After that, the quiz leaderboard (scores) will be sorted from high score to lowest score.
- Upon receiving an upgrade request, the program will chack the user score. If the score is equal or bigger than 30, the player level will be incremented (player starting level will be 1) and 30 points will be deducted.
- Upon receiving a mega upgrade request, the program will check the user level. If the user level is 3 or higher, a NFT will be minted to his wallet. That NFT will be later on used as a whitelist for the project.
- Upon reaching max level (3 for the hackhaton), the player can continue to play to increase his score. The program will receive a daily request to distribute 0.4 bSOL from the program vault to the top3 players.

All this client / program interaction will be done through our Vinci dApp ([https://github.com/VinciWorld/vinci-dapp](https://github.com/VinciWorld/vinci-dapp)).
- The user can login with the available social oAuth providers (Discord, Twitter and Twitch)
- After logging in, the user will be redirected to the Quiz Page, specially made for this hackathon
- After connecting his wallet, the user can start playing the Quiz
    - Quiz Questions are stored in our database
    - The answer backend method will store the user answer off and on-chain
- The Leaderboard is fetched from our on-chain program

## Quick game walkthrough

A quick game walkthrough can be found here [https://www.youtube.com/watch?v=tcni-8_ceF8](https://www.youtube.com/watch?v=tcni-8_ceF8).
The rewards button, which will distribute bSOL, will not be available (only in the video) as it will be called daily and does not allow interaction with the user.


## Other Vinci World Programs (Out of Speedrun scope)

Vinci Accounts - [https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-accounts](https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-accounts).
- Program that will be responsible for storing the Vinci User relavant data


Vinci Stake - [https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-stake](https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-stake).
- Program where the user will be able to stake their NFTs


Vinci Swap - [https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-swap](https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-swap).
- Program where the user will be able to trade assets to improve his character


Vinci Rewards - [https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-rewards](https://github.com/VinciWorld/vinci-stake/tree/main/programs/vinci-rewards).
- Program responsible for deploying rewards systems as needed
