#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::prelude::vec::Vec;
use ink::prelude::vec;

#[cfg_attr(test, allow(dead_code))]

pub type TokenId = u128;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
pub enum Error {
    /// Returned if not enough tokens
    NotEnoughTokens,
    /// Returned if caller is not the `recipient` while required to.
    CallerIsNotRecipient
}

/// Type alias for the contract's `Result` type.
pub type Result<T> = core::result::Result<T, Error>;

#[ink::contract]
mod coinsender {
    use super::*;

    use ink::storage::Mapping;

    #[ink(event)]
    pub struct TransferSingle {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        token_id: TokenId,
        value: Balance,
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct Contract {
        balances: Mapping<(AccountId, TokenId), Balance>,
        token_id_nonce: TokenId,
        admin: Vec<AccountId>
    }

    impl Contract {

        // The only constructor of the contract.
        // The argument admin is required.
        // admin - accounts for fee
        #[ink(constructor)]
        pub fn new(admin: AccountId) -> Self {
            Self { 
                balances: Mapping::new(),
                token_id_nonce: 0,
                admin: vec![admin]
            }
        }

        // Creation of tokens, linking to the balance of the creator
        // 0.1% the percentage of tokens is transferred to the administrator's balance as a commission
        #[ink(message)]
        pub fn create_tokens(&mut self, value: Balance) -> TokenId {
            let caller = self.env().caller();
            self.token_id_nonce += 1;

            self.balances.insert((caller, self.token_id_nonce), &value);
            self.env().emit_event(TransferSingle {
                from: None,
                to: if value == 0 { None } else { Some(caller) },
                token_id: self.token_id_nonce,
                value,
            });

            let fee = (value / 1000) as u128;
            let _ = self.transfer_tokens(caller, self.admin[0], self.token_id_nonce, fee);

            self.token_id_nonce
        }

        // Tokens transfer
        #[ink(message)]
        pub fn transfer_tokens(
            &mut self,
            from: AccountId,
            to: AccountId,
            token_id: TokenId,
            value: Balance,
        ) -> Result<()> {
            if self.env().caller() != from {
                return Err(Error::CallerIsNotRecipient)
            }

            let mut sender_balance = self
                .balances
                .get((from, token_id))
                .expect("Caller should have ensured that `from` holds `token_id`.");

            if sender_balance < value {
                return Err(Error::CallerIsNotRecipient)
            }

            sender_balance -= value;

            self.balances.insert((from, token_id), &sender_balance);

            let mut recipient_balance = self.balances.get((to, token_id)).unwrap_or(0);
            recipient_balance += value;
            self.balances.insert((to, token_id), &recipient_balance);

            self.env().emit_event(TransferSingle {
                from: Some(from),
                to: Some(to),
                token_id,
                value,
            });

            Ok(())
        }

        // Checking the token balance for one account
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId, token_id: TokenId) -> Balance {
            self.balances.get((owner, token_id)).unwrap_or(0)
        }

        // Checking the balance of tokens for multiple accounts
        #[ink(message)]
        pub fn balance_of_batch(
            &self,
            owners: Vec<AccountId>,
            token_ids: Vec<TokenId>,
        ) -> Vec<Balance> {
            let mut output = Vec::new();
            for o in &owners {
                for t in &token_ids {
                    let amount = self.balance_of(*o, *t);
                    output.push(amount);
                }
            }
            output
        }
    }
}