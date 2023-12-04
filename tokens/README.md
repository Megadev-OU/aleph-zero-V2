# COINSENDER TOKENS

This implements the creation of a token and sending it to any recipient on the blockchain Aleph Zero.
Added the ability to get a token balance on one or more accounts.

## Create tokens
Need to transfer the number of tokens to create. The created tokens will be linked to the balance of the account that sent the transaction. Each newly created token will have a unique ID starting from 1 and increasing by 1 each time.

## Transfer tokens
To send tokens, must transfer the sender, recipient, token ID and quantity.
The transferred amount of tokens will be debited from the sender's balance and added to the recipient's balance.
Possible errors - sendr have not enough tokens, sender is not a signer.

## Token balance
It is possible to check the balance of one account by transferring the ID account and ID token.
In order to get a token balance for several accounts, need to transfer an array of accounts and the ID of the token.

develop - cargo watch
build - cargo +nightly contract build --release