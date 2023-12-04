#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::prelude::vec::Vec;
use ink::prelude::vec;

#[cfg_attr(test, allow(dead_code))]

pub type TokenId = u128;
pub type Time = u128;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    // Returned if the requested transfer failed. This can be the case if 
    // does not have sufficient free funds
    TransferFailed
}

pub type Result<T> = core::result::Result<T, Error>;

#[ink::contract]
mod coinsender {
    use super::*;

    use ink::storage::Mapping;

    #[ink(event)]
    pub struct TransferTimes {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        time: Time,
        value: Balance,
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct Contract {
        recipients: Mapping<(AccountId, Time), Balance>,
        admin: Vec<AccountId> 
    }

    impl Contract {

        // The only constructor of the contract.
        // The arguments `recipient` and `admin` are required.
        // admin - accounts for fee
        // recipients -  accounts with amounts and time of unblocking
        #[ink(constructor)]
        pub fn new(admin: AccountId) -> Self {
            Self {
                admin: vec![admin],
                recipients: Mapping::new()
            }
        }

        // Group sending of Azero tokens
        // 0.1% commission is paid from each token sending
        #[ink(message, payable)]
        pub fn sending_azero(&mut self, amounts: Vec<Balance>, recipients: Vec<AccountId>) -> Result<()> {
            let amount: u128 = amounts.iter().sum();
            self.env().transfer(self.admin[0], (amount / 1000) as u128).map_err(|_| Error::TransferFailed)?;
            for i in 0..recipients.len() {
                self.env()
                    .transfer(recipients[i], amounts[i])
                    .map_err(|_| Error::TransferFailed)?;
            }
            Ok(())
        }

        // Group sending of Azero tokens with a temporary lock
        // 0.1% commission is paid from each token sending
        // To withdraw funds, the recipient must make a transaction after the end of the blocking period
        #[ink(message, payable)]
        pub fn send_azero_lock(&mut self, amounts: Vec<Balance>, recipients: Vec<AccountId>, times: Vec<Time>) -> Result<()> {
            let amount: u128 = amounts.iter().sum();
            self.env().transfer(self.admin[0], (amount / 1000) as u128).map_err(|_| Error::TransferFailed)?;
            let caller = self.env().caller();
            for i in 0..recipients.len() {
                self.recipients.insert((recipients[i], times[i]), &amounts[i]);
                self.env().emit_event(TransferTimes {
                    from: caller,
                    to: recipients[i],
                    time: times[i],
                    value: amounts[i]
                });
            }
            Ok(())
        }

        // The recipient can withdraw the funds only after the expiration of the deadline set by the sender.
        // If the user does not have the right to withdraw funds, the program returns Ok
        // After the withdrawal of funds, the recipient's amount is deleted from the storage
        #[ink(message)]
        pub fn withdraw(&mut self, time: Time) -> Result<()> {
            let time_now = Self::env().block_timestamp() as u128;
            if time > time_now { return Err(Error::TransferFailed); }
            let recipient = self.env().caller();
            let amount = self.recipients.get((recipient, time)).unwrap_or(0);
            if amount > 0 {
                self.env().transfer(recipient, amount).map_err(|_| Error::TransferFailed)?;
                self.recipients.insert((recipient, time), &0);
            }
            Ok(())
        }

        // Checking the right to withdraw funds
        // It is also necessary timestamp
        #[ink(message)]
        pub fn balance_lock(&self, owner: AccountId, time: Time) -> Balance {
            self.recipients.get((owner, time)).unwrap_or(0)
        }

    }
}