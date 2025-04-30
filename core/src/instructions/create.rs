use generic_array_struct::generic_array_struct;

use crate::instructions::internal_utils::impl_memset;

// Accounts

/// Also applicable to CreateIdempotent
#[generic_array_struct(builder pub)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CreateIxAccs<T> {
    pub funding: T,

    /// The new associated token account to be created
    pub ata: T,

    /// The authority AKA owner of `ata`
    pub wallet: T,

    pub mint: T,
    pub sys_prog: T,
    pub token_prog: T,
}

impl<T: Copy> CreateIxAccs<T> {
    impl_memset!(CREATE_IX_ACCS_LEN);
}

pub type CreateIxAccsFlag = CreateIxAccs<bool>;

/// Also applicable to CreateIdempotent
pub const CREATE_IX_IS_SIGNER: CreateIxAccsFlag =
    CreateIxAccsFlag::memset(false).const_with_funding(true);

/// Also applicable to CreateIdempotent
pub const CREATE_IX_IS_WRITABLE: CreateIxAccsFlag = CreateIxAccsFlag::memset(false)
    .const_with_funding(true)
    .const_with_ata(true);

// Data

pub const CREATE_IX_DISCM: u8 = 0;

pub const CREATE_IDEMPOTENT_IX_DISCM: u8 = 1;

/// Also applicable to CreateIdempotent
pub const CREATE_IX_DATA_LEN: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CreateIxData;

impl CreateIxData {
    pub const DATA: u8 = CREATE_IX_DISCM;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CreateIdempotentIxData;

impl CreateIdempotentIxData {
    pub const DATA: u8 = CREATE_IDEMPOTENT_IX_DISCM;
}
