use solana_sdk::pubkey::Pubkey;

pub fn get_ata(
  wallet_address: &Pubkey,
  token_mint_address: &Pubkey
) -> Pubkey {
  spl_associated_token_account::get_associated_token_address(wallet_address, token_mint_address)
}
