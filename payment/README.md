# COINSENDER PAYMENTS

This implements the group sending of native Alephzero tokens.
It is possible to instantly send tokens to an array of recipient wallets.
Added the function of sending tokens with temporary blocking, to withdraw funds, the recipient must complete the transaction after the blocking date. By transferring account and the end date of the block, can check the available amount.

## AZERO sending
Send an arrays of the amounts of tokens and account addresses for sending native tokens.
The index of the account address array must match the index of the array with the amount sent to this account.

## AZERO lock sending
Send an arrays of the amounts of tokens, account addresses for sending native tokens, lockdown end dates.
The index of the account address array must match the index of the array with the amount sent to this account and lockdown end date.

## Withdraw AZERO locked tokens
Send accountId and lockdown date for withdraw. If the account does not have the right to withdraw funds, 0 will be transferred and the program will return success.

## Get AZERO locked amount
Send accountId and lockdown date (timestamp) for showing locked ammount.

develop - cargo watch
build - cargo +nightly contract build --release