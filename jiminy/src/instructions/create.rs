use jiminy_cpi::{account::AccountHandle, AccountPerms};
use sanctum_ata_core::instructions::create::{
    CreateIdempotentIxData, CreateIxAccs, CreateIxData, CREATE_IX_ACCS_LEN, CREATE_IX_IS_SIGNER,
    CREATE_IX_IS_WRITABLE,
};

use super::{internal_utils::signer_writable_to_perms, AtaInstr};

pub type CreateIxAccounts<'a> = CreateIxAccs<AccountHandle<'a>>;
pub type CreateIxAccountPerms = CreateIxAccs<AccountPerms>;

/// Also applicable to create idempotent
pub const CREATE_IX_ACCOUNT_PERMS: CreateIxAccountPerms = CreateIxAccs(signer_writable_to_perms(
    CREATE_IX_IS_SIGNER.0,
    CREATE_IX_IS_WRITABLE.0,
));

#[inline]
pub fn create_ix<'account>(
    ata_prog: AccountHandle<'account>,
    accounts: CreateIxAccounts<'account>,
) -> AtaInstr<'account, CREATE_IX_ACCS_LEN> {
    create_ix_inner(ata_prog, accounts, &CreateIxData::DATA)
}

#[inline]
pub fn create_idempotent_ix<'account>(
    ata_prog: AccountHandle<'account>,
    accounts: CreateIxAccounts<'account>,
) -> AtaInstr<'account, CREATE_IX_ACCS_LEN> {
    create_ix_inner(ata_prog, accounts, &CreateIdempotentIxData::DATA)
}

#[inline]
fn create_ix_inner<'account>(
    ata_prog: AccountHandle<'account>,
    accounts: CreateIxAccounts<'account>,
    ix_discm: &'static u8,
) -> AtaInstr<'account, CREATE_IX_ACCS_LEN> {
    AtaInstr {
        prog: ata_prog,
        data: core::slice::from_ref(ix_discm),
        accounts: accounts.0.into_iter().zip(CREATE_IX_ACCOUNT_PERMS.0),
    }
}
