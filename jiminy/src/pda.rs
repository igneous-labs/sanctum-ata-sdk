use core::{mem::MaybeUninit, slice};

use jiminy_cpi::pda::{create_program_address_to, try_find_program_address_to, PdaSeed};
use sanctum_ata_core::pda::AtaPdaSeeds;

#[inline]
pub fn try_find_ata(seeds: &AtaPdaSeeds, ata_prog: &[u8; 32]) -> Option<([u8; 32], u8)> {
    let mut pda = MaybeUninit::uninit();
    let mut bump = MaybeUninit::uninit();
    try_find_ata_to(seeds, ata_prog, &mut pda, &mut bump)?;
    Some(unsafe { (pda.assume_init(), bump.assume_init()) })
}

#[inline]
pub fn try_find_ata_to<'pda, 'bump>(
    seeds: &AtaPdaSeeds,
    ata_prog: &[u8; 32],
    pda_dst: &'pda mut MaybeUninit<[u8; 32]>,
    bump_dst: &'bump mut MaybeUninit<u8>,
) -> Option<(&'pda mut [u8; 32], &'bump mut u8)> {
    let seeds = seeds.as_seeds().map(|s| PdaSeed::new(s.as_slice()));
    try_find_program_address_to(&seeds, ata_prog, pda_dst, bump_dst)
}

#[inline]
pub fn create_ata(seeds: &AtaPdaSeeds, bump: &u8, ata_prog: &[u8; 32]) -> Option<[u8; 32]> {
    let mut pda = MaybeUninit::uninit();
    create_ata_to(seeds, bump, ata_prog, &mut pda)?;
    Some(unsafe { pda.assume_init() })
}

#[inline]
pub fn create_ata_to<'pda>(
    seeds: &AtaPdaSeeds,

    // Changing this to pass bump by value (`u8` instead of `&u8`)
    // results in additional 16 bytes binary size and 2 CUs runtime cost
    bump: &u8,

    ata_prog: &[u8; 32],
    pda_dst: &'pda mut MaybeUninit<[u8; 32]>,
) -> Option<&'pda mut [u8; 32]> {
    let [s1, s2, s3] = seeds.as_seeds().map(|s| PdaSeed::new(s.as_slice()));
    create_program_address_to(
        &[s1, s2, s3, PdaSeed::new(slice::from_ref(bump))],
        ata_prog,
        pda_dst,
    )
}
