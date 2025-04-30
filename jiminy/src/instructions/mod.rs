use core::{array, iter::Zip};

use jiminy_cpi::{account::AccountHandle, AccountPerms};

pub mod create;

mod internal_utils;

/// The 'data lifetime is simply 'static because all ATA program instructions
///  have no args so ix data are all consts
pub type AtaInstr<'account, const ACCOUNTS: usize> = jiminy_cpi::Instr<
    'account,
    'static,
    Zip<
        array::IntoIter<AccountHandle<'account>, ACCOUNTS>,
        array::IntoIter<AccountPerms, ACCOUNTS>,
    >,
>;
