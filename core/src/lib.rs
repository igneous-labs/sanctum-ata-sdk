#![cfg_attr(not(test), no_std)]

pub mod instructions;

pub const ID_STR: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

pub const ID: [u8; 32] = const_crypto::bs58::decode_pubkey(ID_STR);
