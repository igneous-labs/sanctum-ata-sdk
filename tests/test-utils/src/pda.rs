use sanctum_ata_core::pda::AtaPdaArgs;
use solana_pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address_with_program_id;

pub type SolAtaPdaSeeds = AtaPdaArgs<Pubkey>;

pub fn sol_find_ata(seeds: SolAtaPdaSeeds) -> Pubkey {
    get_associated_token_address_with_program_id(seeds.wallet(), seeds.mint(), seeds.token_prog())
}
