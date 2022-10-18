use std::str::FromStr;
use solana_sdk::pubkey::Pubkey;
use lazy_static::lazy_static;

lazy_static! {
  pub static ref METAPLEX_PROGRAM: Pubkey = Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap();
  pub static ref TOKEN_PROGRAM: Pubkey = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
  pub static ref ASSOCIATED_TOKEN_PROGRAM: Pubkey = Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap();
}

