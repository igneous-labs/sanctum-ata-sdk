//! All checks here need to pass for a mollusk instruction sequence execution,
//! else it is an invalid transaction.

use solana_account::Account;
use solana_pubkey::Pubkey;

pub fn is_tx_balanced(pre: &[(Pubkey, Account)], post: &[(Pubkey, Account)]) -> bool {
    let [pre_lamports, post_lamports] = [pre, post].map(|slice| {
        slice
            .iter()
            .map(|(_pk, account)| account.lamports)
            .sum::<u64>()
    });
    pre_lamports == post_lamports
}
