use serde::{Deserialize, Serialize};
use serum_common::pack::*;
use solana_client_gen::prelude::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum FundType {
    /// similar to a gofundme
    FundMe,
    Raise {
        private: bool,
    },
}

/// The Owner of the fund has the right to withdraw all or some of the funds
#[derive(Debug, Serialize, Deserialize)]
pub struct Fund {
    /// check to see if a fund is ininitialized
    pub initialized: bool,
    /// open defines if a fund is open for deposits
    pub open: bool,
    /// type of fund
    pub fund_type: FundType,
    /// fund Owner
    pub owner: Pubkey,
    /// Owner authority
    pub authority: Pubkey,
    /// max size of the fund
    pub max_balance: u64,
    /// balance of the
    pub balance: u64,
    /// Nonce of the program account
    pub nonce: u8,
    /// Mint
    pub mint: Pubkey,
    /// Address of the token vault controlled by the Safe.
    pub vault: Pubkey,

    /// Params

    /// shares, shares increment with investment, but do not decrement with withdraw
    pub shares: u64,
    /// nft account
    pub nft_account: Pubkey,
    /// nft mint
    pub nft_mint: Pubkey,
    /// whitelist represents a list of pubkeys that can deposit into a fund
    pub whitelist: Pubkey,

    /// Payback info

    /// payback balance
    pub payback_total: u64,
    /// amount of token returned
    pub payback_per_share: u64,
}

impl Fund {
    pub fn deduct(&mut self, amount: u64) {
        if self.balance > 0 {
            self.balance -= amount;
        }
    }
    /// Add adds the depoist amount to the total balance and shares
    pub fn add(&mut self, amount: u64) {
        self.balance += amount;
        if self.fund_type.eq(&FundType::Raise { private: true })
            || self.fund_type.eq(&FundType::Raise { private: false })
        {
            self.shares += amount;
        }
    }
    /// close_fund is called when the owner starts the withdrawl process
    pub fn close_fund(&mut self) {
        if self.open {
            self.open = false;
        }
    }
    pub fn add_payback_bal(&mut self, amount: u64) {
        self.payback_total += amount;
    }
    pub fn add_payback_per_share(&mut self, amount: u64) {
        self.payback_per_share += amount;
    }
}

serum_common::packable!(Fund);
