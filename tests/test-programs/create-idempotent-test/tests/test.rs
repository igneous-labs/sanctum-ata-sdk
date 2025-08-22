//! .so file size 6_288

#![cfg(feature = "test-sbf")]

use mollusk_svm::{
    program::keyed_account_for_system_program,
    result::{Check, InstructionResult},
    Mollusk,
};
use proptest::{prelude::*, strategy::Union};
use sanctum_ata_jiminy::sanctum_ata_core::{
    instructions::create::{NewCreateIxAccsBuilder, CREATE_IX_IS_SIGNER, CREATE_IX_IS_WRITABLE},
    pda::AtaPdaArgsBuilder,
};
use sanctum_ata_test_utils::{
    account_from_mint, account_from_token_acc, init_mint_acc, is_tx_balanced,
    key_signer_writable_to_metas, silence_mollusk_prog_logs, sol_find_ata, token_acc_for_trf,
};
use solana_account::Account;
use solana_pubkey::Pubkey;
use spl_token::{
    solana_program::instruction::{AccountMeta, Instruction},
    state::{Account as TokenAccount, Mint},
};

const PROG_NAME: &str = "create_idempotent_test";
const PROG_ID: Pubkey = solana_pubkey::pubkey!("9ibbe3zwJq3uabkPG4XYp9BwrpeAumHSEEetJiUdGyqT");

const FUNDING: Pubkey = solana_pubkey::pubkey!("FmqrDYpnekE92iPotx8PGQed8fQ9DbeMuE7ASeA9Q72x");
const WALLET: Pubkey = solana_pubkey::pubkey!("2mQbNpB6tbF6cguY7M6NjGozGLTUwJVeUBceWqEH3gkt");
const MINT: Pubkey = solana_pubkey::pubkey!("2AHbbAHQQrQsEP7yrE9PGWpkn7Uz27PKJBByRwkurnWG");
const SUPPLY: u64 = 29_125_461_325;
const DECIMALS: u8 = 9;

thread_local! {
    static SVM: Mollusk = {
        let mut svm = Mollusk::new(&PROG_ID, PROG_NAME);
        mollusk_svm_programs_token::token::add_program(&mut svm);
        mollusk_svm_programs_token::associated_token::add_program(&mut svm);
        svm
    };
}

// CUs: 28_851
#[test]
fn create_idempotent_nonexisting_cus() {
    let accounts = ix_accounts(
        FUNDING,
        WALLET,
        MINT,
        init_mint_acc(None, SUPPLY, DECIMALS, None),
        None,
    );
    let instr = ix(FUNDING, WALLET, MINT);

    SVM.with(|svm| {
        let InstructionResult {
            compute_units_consumed,
            raw_result,
            resulting_accounts,
            ..
        } = svm.process_and_validate_instruction(&instr, &accounts, &[Check::all_rent_exempt()]);

        raw_result.unwrap();

        eprintln!("{compute_units_consumed} CUs");

        assert!(is_tx_balanced(&accounts, &resulting_accounts));

        // if program succeeded, it means ata create successfully
        // executed, so no need to check properties of created ata
    });
}

// CUs: 12_842
#[test]
fn create_idempotent_already_existing_cus() {
    let accounts = ix_accounts(
        FUNDING,
        WALLET,
        MINT,
        init_mint_acc(None, SUPPLY, DECIMALS, None),
        Some(token_acc_for_trf(MINT, 0, false, WALLET)),
    );
    let instr = ix(FUNDING, WALLET, MINT);

    SVM.with(|svm| {
        let InstructionResult {
            compute_units_consumed,
            raw_result,
            resulting_accounts,
            ..
        } = svm.process_and_validate_instruction(&instr, &accounts, &[Check::all_rent_exempt()]);

        raw_result.unwrap();

        eprintln!("{compute_units_consumed} CUs");

        assert!(is_tx_balanced(&accounts, &resulting_accounts));

        // if program succeeded, it means ata create successfully
        // executed, so no need to check properties of created ata
    })
}

proptest! {
    #[test]
    fn create_idempotent_all_cases(
        (mint, funding, wallet, token_acc) in
            any::<[u8; 32]>().prop_flat_map(
                |mint| (
                    Just(Pubkey::new_from_array(mint)),
                    any::<[u8; 32]>().prop_filter("", move |k| *k != mint).prop_map(Pubkey::new_from_array),
                    any::<[u8; 32]>().prop_filter("", move |k| *k != mint).prop_map(Pubkey::new_from_array),
                )
            ).prop_flat_map(
                |(mint, funding, wallet)| (
                    Just(mint),
                    Just(funding),
                    Just(wallet),
                    Union::new([
                        Just(None).boxed(),
                        (any::<u64>(), any::<bool>())
                            .prop_map(move |(amt, is_native)| Some(token_acc_for_trf(mint, amt, is_native, wallet)))
                            .boxed()
                    ]),
                )
            ),
        decimals: u8,
        supply: u64,
    ) {
        silence_mollusk_prog_logs();
        let accounts = ix_accounts(
            funding,
            wallet,
            mint,
            init_mint_acc(None, supply, decimals, None),
            token_acc,
        );
        let instr = ix(funding, wallet, mint);

        SVM.with(|svm| {
            let InstructionResult {
                raw_result,
                resulting_accounts,
                ..
            } = svm.process_and_validate_instruction(&instr, &accounts, &[Check::all_rent_exempt()]);

            raw_result.unwrap();

            prop_assert!(is_tx_balanced(&accounts, &resulting_accounts));

            // if program succeeded, it means ata create successfully
            // executed, so no need to check properties of created ata

            Ok(())
        }).unwrap();
    }
}

fn ix_accounts(
    funding: Pubkey,
    wallet: Pubkey,
    mint: Pubkey,
    mint_acc: Mint,
    ata_acc: Option<TokenAccount>,
) -> [(Pubkey, Account); 7] {
    let ata = sol_find_ata(
        AtaPdaArgsBuilder::start()
            .with_mint(mint)
            .with_token_prog(spl_token::ID)
            .with_wallet(wallet)
            .build(),
    );
    [
        mollusk_svm_programs_token::associated_token::keyed_account(),
        (
            funding,
            Account {
                lamports: 1_000_000_000,
                ..Default::default()
            },
        ),
        (
            ata,
            ata_acc.map_or_else(Account::default, account_from_token_acc),
        ),
        (wallet, Account::default()),
        (mint, account_from_mint(mint_acc)),
        keyed_account_for_system_program(),
        mollusk_svm_programs_token::token::keyed_account(),
    ]
}

fn ix(funding: Pubkey, wallet: Pubkey, mint: Pubkey) -> Instruction {
    let ata = sol_find_ata(
        AtaPdaArgsBuilder::start()
            .with_mint(mint)
            .with_token_prog(spl_token::ID)
            .with_wallet(wallet)
            .build(),
    );
    Instruction {
        program_id: PROG_ID,
        accounts: core::iter::once(AccountMeta {
            pubkey: mollusk_svm_programs_token::associated_token::ID,
            is_signer: false,
            is_writable: false,
        })
        .chain(key_signer_writable_to_metas(
            &NewCreateIxAccsBuilder::start()
                .with_ata(ata)
                .with_funding(funding)
                .with_mint(mint)
                .with_sys_prog(keyed_account_for_system_program().0)
                .with_token_prog(spl_token::ID)
                .with_wallet(wallet)
                .build()
                .0,
            &CREATE_IX_IS_SIGNER.0,
            &CREATE_IX_IS_WRITABLE.0,
        ))
        .collect(),
        data: Vec::new(),
    }
}
