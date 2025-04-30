use proptest::{prelude::*, strategy::Union};
use spl_token::{
    solana_program::{program_option::COption, pubkey::Pubkey},
    state::Mint,
};

pub fn any_init_mint() -> impl Strategy<Value = Mint> {
    (
        any_coption::<[u8; 32]>(),
        any::<u64>(),
        any::<u8>(),
        any_coption::<[u8; 32]>(),
    )
        .prop_map(
            |(mint_authority, supply, decimals, freeze_authority)| Mint {
                mint_authority: mint_authority.map(Pubkey::new_from_array),
                supply,
                decimals,
                is_initialized: true,
                freeze_authority: freeze_authority.map(Pubkey::new_from_array),
            },
        )
}

fn any_coption<T: Arbitrary + Clone + 'static>() -> impl Strategy<Value = COption<T>> {
    Union::new([
        Just(COption::None).boxed(),
        any::<T>().prop_map(|val| COption::Some(val)).boxed(),
    ])
}
