#![cfg_attr(not(test), no_std)]

pub mod instructions;
pub mod pda;

pub const ID_STR: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";

pub const ID: [u8; 32] = const_crypto::bs58::decode_pubkey(ID_STR);
