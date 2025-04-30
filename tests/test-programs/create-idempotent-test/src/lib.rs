//! This program
//! - Takes in the same account arguments as create idempotent ATA instruction
//! - Runs try_find_ata to verify that the provided ATA is of the correct address
//! - Runs create_ata with returned bump to verify the results of try_find_ata
//! - CPIs create idempotent ATA instruction

#![allow(unexpected_cfgs)]

use std::mem::MaybeUninit;

use jiminy_cpi::program_error::BuiltInProgramError;
use jiminy_entrypoint::program_error::ProgramError;
use sanctum_ata_jiminy::{
    instructions::create::create_idempotent_ix,
    pda::{create_ata_to, try_find_ata_to},
    sanctum_ata_core::{instructions::create::CreateIxAccs, pda::NewAtaPdaArgsBuilder},
};

const MAX_ACCOUNTS: usize = 7;

type Accounts<'a> = jiminy_entrypoint::account::Accounts<'a, MAX_ACCOUNTS>;
type Cpi = jiminy_cpi::Cpi<MAX_ACCOUNTS>;

jiminy_entrypoint::entrypoint!(process_ix, MAX_ACCOUNTS);

fn process_ix(
    accounts: &mut Accounts,
    _data: &[u8],
    _prog_id: &[u8; 32],
) -> Result<(), ProgramError> {
    // WTF changing this to ata_prog at the end and split_first_chunk()
    // instead of ata_prog at start and split_last_chunk() adds 1.5kb to program size
    let (ata_prog, create_accs) = match accounts.as_slice().split_last_chunk() {
        Some((&[ata_prog], a)) => (ata_prog, CreateIxAccs(*a)),
        _ => {
            return Err(ProgramError::from_builtin(
                BuiltInProgramError::NotEnoughAccountKeys,
            ));
        }
    };

    let [ata_prog_key, mint_key, token_prog_key, wallet_key, ata_key] = [
        &ata_prog,
        create_accs.mint(),
        create_accs.token_prog(),
        create_accs.wallet(),
        create_accs.ata(),
    ]
    .map(|h| accounts.get(*h).key());

    let seeds = NewAtaPdaArgsBuilder::start()
        .with_mint(mint_key)
        .with_token_prog(token_prog_key)
        .with_wallet(wallet_key)
        .build();

    // using MaybeUninit out pointers here:
    // - shaves off ~130 bytes from .so file
    // - saves 12 CUs

    let mut found_ata = MaybeUninit::uninit();
    let mut bump = MaybeUninit::uninit();
    let Some((found_ata, bump)) = try_find_ata_to(&seeds, ata_prog_key, &mut found_ata, &mut bump)
    else {
        return Err(ProgramError::custom(0));
    };

    let mut created_ata = MaybeUninit::uninit();
    let Some(created_ata) = create_ata_to(&seeds, bump, ata_prog_key, &mut created_ata) else {
        return Err(ProgramError::custom(1));
    };

    if found_ata != created_ata {
        return Err(ProgramError::custom(2));
    }

    if found_ata != ata_key {
        return Err(ProgramError::custom(3));
    }

    Cpi::new().invoke_signed(accounts, create_idempotent_ix(ata_prog, create_accs), &[])
}
