use solana_pubkey::Pubkey;
use spl_token::solana_program::instruction::AccountMeta;

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
