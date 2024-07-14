# dao

a program that let anyone create one or multiple polling dao's, users stake their tokens for voting power and get points for partipating.

## initialize 

if you deploy this program yourself, you'll need to initialize the `analytics` pda before creating a dao.

## create dao

when creating a new dao, there are few parameters to consider :

- `threshold` : required threshold to consider a poll approved when being executed, from 51% to 100% (u8 type)
- `mint` : the mint required to stake for users to get voting power (pda)
- `min_poll_tokens` : the minimum number of tokens required to start a new poll, accounting for decimals (u64)
- `name` : dao name, max 50 characters (string)
- `time` : enum with only 4 possible values : `fiveSeconds` (for testing purposes), `twentyFourHours`, `fourtyEightHours` and `oneWeek` (i64).

## stake/unstake

you can stake any amount of tokens to get voting power 1:1 (amount x 1 x 10 ^ decimals).

to unstake your tokens, you need to deactivate your staked tokens, unstaking takes 30 days (5 seconds for testing purposes) before claiming.

users can only unstake their own deposits.

## new poll

before creating a new poll, you need to 1st have the required minimum tokens for your dao.

a poll have a title (max 50 characters), and a content body (280 characters max) and can't be empty.

## new vote

to vote, need voting power.

you have only 2 choices when voting : approve or reject.

users cannot vote twice on the same poll, disregarding of added deposits for more voting power.

users cannot vote on already executed polls, or polls whose voting period have expired (current_time > (created_at + voting_period))

## poll execute

anyone can execute a poll for any dao.

to successfully execute a poll, the voting period have to be expired.

during poll execution, we sum each approvals and rejections votes to define the winning side.

if the winning side equals or exceed the threshold set a dao creation, the poll is marked "approved".

