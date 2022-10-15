use std::sync::Arc;
use eyre::Result;
use solana_sdk::{
  account::Account,
  signature::{Keypair, Signer, Signature},
  transaction::Transaction,
  message::Message,
  instruction::{Instruction},
  commitment_config::CommitmentConfig,
  borsh::{try_from_slice_unchecked}, pubkey::Pubkey,
};
use solana_client::{
  nonblocking::rpc_client,
};

pub struct RpcClient {
  rpc_client: Arc<rpc_client::RpcClient>,
  payer: Arc<Keypair>,
}

impl RpcClient {
  pub fn new(solana_rpc: String, operator_priv_key: String) -> Self {
    let rpc_client =  Arc::new(rpc_client::RpcClient::new_with_commitment(solana_rpc, CommitmentConfig::confirmed()));
    let payer = Arc::new(Keypair::from_base58_string(&operator_priv_key));

    Self {
      rpc_client,
      payer,
    }
  }

  pub async fn send_tx(&self, ix: Instruction) -> Result<Signature> {
    let latest_blockhash = self.rpc_client.get_latest_blockhash().await.unwrap();
    let message = Message::new(&[ix], Some(&self.payer.pubkey()));
    let tx = Transaction::new(&[&*self.payer], message, latest_blockhash);
    let sig = self.rpc_client.send_and_confirm_transaction(&tx).await?;

    Ok(sig)
  }

  pub async fn get_multiple_accounts(&self, pubkeys: &[Pubkey]) -> Result<Vec<Option<Account>>> {
    let accounts = self.rpc_client.get_multiple_accounts(pubkeys).await?;
    Ok(accounts)
  }

  pub async fn get_account(&self, pubkey: &Pubkey) -> Result<Account> {
    let account = self.rpc_client.get_account(pubkey).await?;
    Ok(account)
  }

  pub async fn get_account_data<T>(&self, account: &Pubkey) -> Result<T>
  where 
    T: borsh::BorshDeserialize
  {
    let mut account = self.get_account(account).await?;
    // Note! the first 8 bytes represent the Anchor account discriminator so we need to get rid of it first
    account.data.drain(0..8);

    let account_data = try_from_slice_unchecked::<T>(&account.data)?;
    
    Ok(account_data)
  }
}
