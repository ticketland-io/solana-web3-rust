use std::str::FromStr;
use eyre::Result;
use solana_sdk::pubkey::Pubkey;

pub fn pubkey_from_str(pubkey_str: &str) -> Result<Pubkey> {
  Pubkey::from_str(pubkey_str).map_err(Into::<_>::into)
}
