use solana_account::Account;
use solana_pubkey::Pubkey;
use spl_token::{
    solana_program::{instruction::AccountMeta, program_option::COption, program_pack::Pack},
    state::Mint,
};

use crate::MINT_RENT_EXEMPT_LAMPORTS;

// Mint

pub fn init_mint_acc(
    mint_authority: Option<Pubkey>,
    supply: u64,
    decimals: u8,
    freeze_authority: Option<Pubkey>,
) -> Mint {
    Mint {
        mint_authority: mint_authority.map_or_else(|| COption::None, COption::Some),
        supply,
        decimals,
        is_initialized: true,
        freeze_authority: freeze_authority.map_or_else(|| COption::None, COption::Some),
    }
}

pub fn account_from_mint(mint: Mint) -> Account {
    let mut data = vec![0u8; 82];
    Mint::pack(mint, data.as_mut_slice()).unwrap();
    Account {
        data,
        lamports: MINT_RENT_EXEMPT_LAMPORTS,
        owner: spl_token::ID,
        executable: false,
        rent_epoch: u64::MAX,
    }
}

// instructions

pub const fn key_signer_writable_to_metas<const N: usize>(
    keys: &[Pubkey; N],
    is_signer: &[bool; N],
    is_writable: &[bool; N],
) -> [AccountMeta; N] {
    const UNINIT: AccountMeta = AccountMeta {
        pubkey: Pubkey::new_from_array([0; 32]),
        is_signer: false,
        is_writable: false,
    };
    let mut res = [UNINIT; N];
    let mut i = 0;
    while i < N {
        res[i] = AccountMeta {
            pubkey: keys[i],
            is_signer: is_signer[i],
            is_writable: is_writable[i],
        };
        i += 1;
    }
    res
}
