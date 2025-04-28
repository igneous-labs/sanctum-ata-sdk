use core::{array, iter::Zip};

use jiminy_cpi::{account::AccountHandle, AccountPerms};

mod internal_utils;

pub type AtaInstr<'account, 'data, const ACCOUNTS: usize> = jiminy_cpi::Instr<
    'account,
    'data,
    Zip<
        array::IntoIter<AccountHandle<'account>, ACCOUNTS>,
        array::IntoIter<AccountPerms, ACCOUNTS>,
    >,
>;
